use crate::modules::config::{GAME_WINDOW_TITLES, PROCESS_NAMES};
use anyhow::Result;
use sysinfo::{ProcessRefreshKind, RefreshKind, System, UpdateKind};

#[cfg(windows)]
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible,
};

pub struct ProcessScanner {
    sys: System,
}

impl ProcessScanner {
    pub fn new() -> Self {
        Self {
            sys: System::new_with_specifics(
                RefreshKind::new()
                    .with_processes(ProcessRefreshKind::new()
                        .with_user(UpdateKind::Always)
                        .with_cpu()
                        .with_memory()),
            ),
        }
    }

    pub fn scan(&mut self, _debug_mode: bool) -> Result<(bool, bool, Option<String>)> {
        self.sys.refresh_processes();

        let mut found_rpcs3 = false;
        let mut found_game = false;
        let mut matched_title = None;

        for (pid, process) in self.sys.processes() {
            let name = process.name().to_lowercase();
            if PROCESS_NAMES.contains(&name.as_str()) {
                found_rpcs3 = true;

                // Check window title on Windows
                #[cfg(windows)]
                {
                    if let Some(title) = get_window_title_for_pid(pid.as_u32()) {
                        let t_lower = title.to_lowercase();
                        for kw in GAME_WINDOW_TITLES {
                            if t_lower.contains(kw) {
                                found_game = true;
                                matched_title = Some(title);
                                break;
                            }
                        }
                    }
                }
            }

            if found_game {
                break;
            }
        }

        Ok((found_rpcs3, found_game, matched_title))
    }
    pub fn get_own_usage(&self) -> (f32, u64) {
        let pid = sysinfo::Pid::from_u32(std::process::id());
        if let Some(process) = self.sys.process(pid) {
            return (process.cpu_usage(), process.memory());
        }
        (0.0, 0)
    }
}

// Windows Specific Helper
#[cfg(windows)]
fn get_window_title_for_pid(pid: u32) -> Option<String> {
    struct EnumData {
        pid: u32,
        title: Option<String>,
    }

    unsafe extern "system" fn enum_window_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let data = &mut *(lparam.0 as *mut EnumData);
        let mut window_pid = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut window_pid));

        if window_pid == data.pid {
            if IsWindowVisible(hwnd).as_bool() {
                let len = GetWindowTextLengthW(hwnd);
                if len > 0 {
                    let mut buf = vec![0u16; (len + 1) as usize];
                    let copied = GetWindowTextW(hwnd, &mut buf);
                    if copied > 0 {
                        buf.truncate(copied as usize);
                        data.title = Some(String::from_utf16_lossy(&buf));
                        return BOOL(0);
                    }
                }
            }
        }
        BOOL(1)
    }

    let mut data = EnumData { pid, title: None };
    unsafe {
        let _ = EnumWindows(
            Some(enum_window_callback),
            LPARAM(&mut data as *mut _ as isize),
        );
    }
    data.title
}
