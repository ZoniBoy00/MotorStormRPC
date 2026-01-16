use crate::modules::config::LOG_CAPACITY;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct LogMessage {
    pub message: String,
    pub level: LogLevel,
}

#[derive(Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Success,
    #[allow(dead_code)]
    Warning,
    Error,
    Game,
}

pub struct AppState {
    pub game_running: bool,
    pub discord_connected: bool,
    pub debug_mode: bool,
    pub logs: VecDeque<LogMessage>,
    pub start_timestamp: Option<i64>,
    pub matched_window: Option<String>,
    pub cpu_usage: f32,
    pub ram_usage: u64,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            game_running: false,
            discord_connected: false,
            debug_mode: false,
            logs: VecDeque::with_capacity(LOG_CAPACITY),
            start_timestamp: None,
            matched_window: None,
            cpu_usage: 0.0,
            ram_usage: 0,
        }
    }

    pub fn add_log(&mut self, level: LogLevel, msg: String) {
        if self.logs.len() >= LOG_CAPACITY {
            self.logs.pop_front();
        }
        self.logs.push_back(LogMessage { message: msg, level });
    }
}
