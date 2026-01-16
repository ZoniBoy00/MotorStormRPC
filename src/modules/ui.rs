use crate::modules::app::{AppState, LogLevel};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Line, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{io, sync::Arc};
use parking_lot::Mutex;
use std::time::Duration;

use std::sync::atomic::{AtomicBool, Ordering};

pub fn run_tui(app_state: Arc<Mutex<AppState>>, running: Arc<AtomicBool>) -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app_state, running);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app_state: Arc<Mutex<AppState>>,
    running: Arc<AtomicBool>,
) -> io::Result<()> {
    while running.load(Ordering::Relaxed) {
        terminal.draw(|f| ui(f, &app_state))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('d') => {
                            let mut state = app_state.lock();
                            state.debug_mode = !state.debug_mode;
                            let s = if state.debug_mode { "ON" } else { "OFF" };
                            state.add_log(LogLevel::Info, format!("Debug mode {}", s));
                        }
                        KeyCode::Char('c') => {
                             let mut state = app_state.lock();
                             state.logs.clear();
                             state.add_log(LogLevel::Info, "Logs cleared".to_string());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app_state: &Arc<Mutex<AppState>>) {
    let state = app_state.lock();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Length(10), // Status Dashboard
                Constraint::Min(10),   // Logs
                Constraint::Length(3), // Footer
            ]
            .as_ref(),
        )
        .split(f.size());

    // Header
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    
    let title_text = Text::from(Line::from(vec![
        Span::styled("MotorStorm®: Pacific Rift", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        Span::raw(" - Discord RPC"),
    ]));
    
    let paragraph = Paragraph::new(title_text)
        .block(title_block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[0]);

    // Status Dashboard
    let status_block = Block::default()
        .title(" Status ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let game_status = if state.game_running {
        Span::styled("RUNNING", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("NOT DETECTED", Style::default().fg(Color::Red))
    };

    let discord_status = if state.discord_connected {
        Span::styled("CONNECTED", Style::default().fg(Color::Green))
    } else {
        Span::styled("DISCONNECTED", Style::default().fg(Color::Red))
    };

    let window_info = if let Some(w) = &state.matched_window {
        Span::raw(w)
    } else {
        Span::styled("N/A", Style::default().fg(Color::DarkGray))
    };

     let uptime = if let Some(start) = state.start_timestamp {
        let now = chrono::Utc::now().timestamp();
        let diff = now - start;
        let h = diff / 3600;
        let m = (diff % 3600) / 60;
        let s = diff % 60;
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        "--:--:--".to_string()
    };

    let ram_mb = state.ram_usage as f32 / 1024.0 / 1024.0;

    let status_text = vec![
        Line::from(vec![Span::raw("Game Status:      "), game_status]),
        Line::from(vec![Span::raw("Discord Status:   "), discord_status]),
        Line::from(vec![Span::raw("Current Session:  "), Span::raw(uptime)]),
        Line::from(vec![Span::raw("App Usage:        "), Span::raw(format!("CPU: {:.1}% | RAM: {:.2} MB", state.cpu_usage, ram_mb))]),
        Line::from(vec![Span::raw("Detected Window:  "), window_info]),
        Line::from(vec![]),
        Line::from(vec![Span::styled("Monitoring RPCS3 behavior...", Style::default().fg(Color::Gray))]),
    ];

    let status_p = Paragraph::new(status_text)
        .block(status_block)
        .wrap(Wrap { trim: true });
    f.render_widget(status_p, chunks[1]);

    // Logs
    let logs_block = Block::default()
        .title(" Logs ")
        .borders(Borders::ALL);
    
    let mut log_lines = Vec::new();
    for log in &state.logs {
        let style = match log.level {
            LogLevel::Info => Style::default().fg(Color::Cyan),
            LogLevel::Success => Style::default().fg(Color::Green),
            LogLevel::Warning => Style::default().fg(Color::Yellow),
            LogLevel::Error => Style::default().fg(Color::Red),
            LogLevel::Game => Style::default().fg(Color::Magenta),
        };
        let icon = match log.level {
             LogLevel::Info => "ℹ",
            LogLevel::Success => "✓",
            LogLevel::Warning => "⚠",
            LogLevel::Error => "✗",
            LogLevel::Game => "🎮",
        };
        log_lines.push(Line::from(vec![
            Span::styled(format!("{} ", icon), style),
            Span::raw(&log.message),
        ]));
    }
    
    // Reverse logs to show newest at bottom if we want normal log behavior, 
    // but usually in TUI we want scrolling. Ratatui renders top-down. 
    // Let's just show them as is, but maybe scroll to bottom if too many?
    // For simplicity, we just take the last N that fit.
    
    let logs_p = Paragraph::new(log_lines)
        .block(logs_block)
        .wrap(Wrap { trim: false });
    f.render_widget(logs_p, chunks[2]);

    // Footer
    let footer_text = vec![
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("Q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to Quit | "),
            Span::styled("D", Style::default().add_modifier(Modifier::BOLD)),
             Span::raw(" Toggle Debug | "),
             Span::styled("C", Style::default().add_modifier(Modifier::BOLD)),
             Span::raw(" Clear Logs"),
        ])
    ];
    let footer_p = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::TOP))
        .alignment(Alignment::Center);

    f.render_widget(footer_p, chunks[3]);
}
