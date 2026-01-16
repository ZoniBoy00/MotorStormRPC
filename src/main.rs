mod modules;

use modules::app::AppState;
use modules::ui::run_tui;
use modules::utils::run_background_tasks;
use modules::window::{set_console_title, set_console_icon};
use parking_lot::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 0. Ensure Admin Privileges
    if !modules::admin::is_elevated() {
        // Attempt to restart as admin
        if let Ok(_) = modules::admin::run_as_admin() {
            return Ok(());
        }
        // If failed (user said no), we continue but warn.
        // Actually, for a TUI, we might just want to print and exit, 
        // but let's try to continue in case user doesn't need admin for their setup.
    }

    // 1. Setup Window Appearance (Title & Icon)
    set_console_title("MotorStorm®: Pacific Rift - Discord RPC");
    let _ = set_console_icon(); // Best effort

    // 2. Initialize State
    let app_state = Arc::new(Mutex::new(AppState::new()));
    let running = Arc::new(AtomicBool::new(true));

    // 4. Spawn Background Task (Logic)
    let state_clone = app_state.clone();
    tokio::spawn(async move {
        run_background_tasks(state_clone).await;
    });

    // 5. Run TUI on Main Thread
    let res = run_tui(app_state, running.clone());
    
    res
}
