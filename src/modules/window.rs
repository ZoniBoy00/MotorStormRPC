use anyhow::Result;

#[cfg(windows)]
use windows::{
    core::{PCWSTR, HSTRING},
    Win32::Foundation::{LPARAM, WPARAM},

    Win32::UI::WindowsAndMessaging::{
        LoadImageW, SendMessageW,
        IMAGE_ICON, LR_DEFAULTSIZE, LR_LOADFROMFILE, WM_SETICON, ICON_BIG, ICON_SMALL,
    },
    Win32::System::Console::{GetConsoleWindow, SetConsoleTitleW},
};

// Set Console Title
pub fn set_console_title(title: &str) {
    #[cfg(windows)]
    unsafe {
        let title_h = HSTRING::from(title);
        let _ = SetConsoleTitleW(PCWSTR(title_h.as_ptr()));
    }
}

// Set Console Icon (Visual change for "Own Terminal")
pub fn set_console_icon() -> Result<()> {
    #[cfg(windows)]
    unsafe {
        let hwnd = GetConsoleWindow();
        if hwnd.0 == 0 {
            return Ok(());
        }

        
        // Load "icon.ico" from file
        let icon_path = HSTRING::from("icon.ico");
        let h_icon = LoadImageW(
            None,
            PCWSTR(icon_path.as_ptr()),
            IMAGE_ICON,
            0,
            0,
            LR_LOADFROMFILE | LR_DEFAULTSIZE
        )?;

        if h_icon.0 != 0 {
            SendMessageW(hwnd, WM_SETICON, WPARAM(ICON_SMALL as usize), LPARAM(h_icon.0));
            SendMessageW(hwnd, WM_SETICON, WPARAM(ICON_BIG as usize), LPARAM(h_icon.0));
        }
    }
    Ok(())
}
