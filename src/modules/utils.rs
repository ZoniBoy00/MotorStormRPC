use crate::modules::app::{AppState, LogLevel};
use crate::modules::config::{PROCESS_CHECK_INTERVAL_MS, START_COOLDOWN_MS, STOP_COOLDOWN_MS};
use crate::modules::discord::DiscordClient;
use crate::modules::process::ProcessScanner;
use chrono::Utc;
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub async fn run_background_tasks(app_state: Arc<Mutex<AppState>>) {
    let mut discord = DiscordClient::new();
    let mut scanner = ProcessScanner::new();

    let mut last_start_time = 0;
    let mut last_stop_time = 0;

    // Main Loop
    loop {
        let debug_mode = app_state.lock().debug_mode;

        // 1. Process Scan
         match scanner.scan(debug_mode) {
            Ok((found_rpcs3, found_game, matched_title)) => {
                // Update Process Usage (CPU/RAM) BEFORE potential drops
                let (cpu, ram) = scanner.get_own_usage();

                let mut state = app_state.lock();
                state.cpu_usage = cpu;
                state.ram_usage = ram;
                
                if debug_mode && found_rpcs3 {
                    // Log debug info occasionally if needed, avoided for clutter
                }

                let now = Utc::now().timestamp_millis();

                // Logic to update state
                if found_game && !state.game_running {
                    if now - last_start_time > START_COOLDOWN_MS {
                        let msg = format!("MotorStorm Detected: {}", matched_title.clone().unwrap_or_default());
                        state.add_log(LogLevel::Game, msg);
                        state.game_running = true;
                        state.start_timestamp = Some(Utc::now().timestamp());
                        state.matched_window = matched_title;
                        
                        // Use the timestamp we just set
                        let start_ts = state.start_timestamp.unwrap();
                        
                        last_start_time = now;
                        
                        // Update Discord
                         drop(state); // Drop lock before IO
                         
                         // Try to update presence, if it fails, try to reconnect and update
                         if let Err(e) = discord.update_presence(start_ts) {
                              app_state.lock().add_log(LogLevel::Warning, format!("Initial presence update failed: {}", e));
                              let _ = discord.connect(); // Try reconnect immediately
                              let _ = discord.update_presence(start_ts); // Retry update
                         }
                    }
                } else if (!found_rpcs3 && state.game_running) || (!found_game && state.game_running && found_rpcs3) {
                     if now - last_stop_time > STOP_COOLDOWN_MS {
                         if !found_rpcs3 {
                             state.add_log(LogLevel::Game, "RPCS3 process closed".to_string());
                         } else {
                              state.add_log(LogLevel::Game, "Game window no longer active".to_string());
                         }
                         state.game_running = false;
                         state.matched_window = None;
                         state.start_timestamp = None;
                         last_stop_time = now;

                         // Clear Discord
                         drop(state);
                         
                         if let Err(e) = discord.clear_presence() {
                             // Non-fatal, but log it
                             if debug_mode {
                                 app_state.lock().add_log(LogLevel::Error, format!("Clear presence failed: {}", e));
                             }
                         }
                     }
                } else {
                     // Update connection status in UI
                     state.discord_connected = discord.is_connected();
                }


            }
            Err(e) => {
                app_state.lock().add_log(LogLevel::Error, format!("Scan error: {}", e));
            }
        }

        // Reconnect logic if disconnected
        if !discord.is_connected() {
             if let Ok(_) = discord.connect() {
                 app_state.lock().add_log(LogLevel::Success, "Reconnected to Discord".to_string());
             }
        }

        sleep(Duration::from_millis(PROCESS_CHECK_INTERVAL_MS)).await;
    }
}
