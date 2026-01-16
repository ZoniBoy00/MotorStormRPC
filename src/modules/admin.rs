use std::io::Error;
use std::ptr::null_mut;

#[cfg(windows)]
use windows::{
    core::PCWSTR,
    Win32::Foundation::{HANDLE, HWND},
    Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    },
    Win32::System::Threading::{GetCurrentProcess, OpenProcessToken},
    Win32::UI::Shell::ShellExecuteW,
    Win32::UI::WindowsAndMessaging::SW_SHOW,
};

pub fn is_elevated() -> bool {
    #[cfg(windows)]
    unsafe {
        let mut handle = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle).is_ok() {
            let mut elevation = TOKEN_ELEVATION::default();
            let mut size = 0;
            let result = GetTokenInformation(
                handle,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut size,
            );
            // GetTokenInformation returns Result, handle it.
            if result.is_ok() {
                return elevation.TokenIsElevated != 0;
            }
        }
        false
    }
    #[cfg(not(windows))]
    true
}

pub fn run_as_admin() -> Result<(), Error> {
    #[cfg(windows)]
    unsafe {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use std::env;

        let exe_path = env::current_exe()?;
        let exe_path_wide: Vec<u16> = OsStr::new(&exe_path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        // Operation "runas" requests elevation
        let operation = "runas\0".encode_utf16().collect::<Vec<u16>>();
        
        let result = ShellExecuteW(
            HWND(0),
            PCWSTR(operation.as_ptr()),
            PCWSTR(exe_path_wide.as_ptr()),
            PCWSTR(null_mut()),
            PCWSTR(null_mut()),
            SW_SHOW,
        );

        if result.0 as isize <= 32 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}
