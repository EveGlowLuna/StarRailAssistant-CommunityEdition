// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// This application is only supported on Windows
#[cfg(not(target_os = "windows"))]
compile_error!("This application is only supported on Windows");

#[cfg(target_os = "windows")]
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
#[cfg(target_os = "windows")]
use windows::Win32::UI::Shell::ShellExecuteW;
#[cfg(target_os = "windows")]
use windows::core::PCWSTR;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HANDLE;

#[cfg(target_os = "windows")]
#[allow(dead_code)]
fn is_elevated() -> bool {
    unsafe {
        let mut token: HANDLE = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }
        let mut elevation: TOKEN_ELEVATION = std::mem::zeroed();
        let mut size = 0;
        if GetTokenInformation(token, TokenElevation, Some(&mut elevation as *mut _ as *mut _), std::mem::size_of::<TOKEN_ELEVATION>() as u32, &mut size).is_err() {
            return false;
        }
        elevation.TokenIsElevated != 0
    }
}

#[cfg(target_os = "windows")]
#[allow(dead_code)]
fn run_as_admin() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_path_utf16: Vec<u16> = exe_path.to_string_lossy().encode_utf16().chain(std::iter::once(0)).collect();
    use windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD;
    unsafe {
        ShellExecuteW(
            None,
            windows::core::w!("runas"),
            PCWSTR::from_raw(exe_path_utf16.as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            SHOW_WINDOW_CMD(5), // SW_SHOW
        );
    }
    std::process::exit(0);
}

fn main() {
    // Request admin privileges on Windows (only in release mode)
    #[cfg(target_os = "windows")]
    #[cfg(not(debug_assertions))]
    if !is_elevated() {
        run_as_admin();
    }

    starrailassistant_lib::run()
}
