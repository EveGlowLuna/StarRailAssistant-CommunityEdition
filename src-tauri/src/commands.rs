// Tauri 命令定义模块

use crate::process;
use crate::config;
use crate::logger;
use crate::wallpaper;
use crate::announcement;
use crate::settings;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Windows 提权命令
#[cfg(target_os = "windows")]
#[tauri::command]
pub fn run_elevated_command(command: String) -> Result<(), String> {
    use windows::core::PCWSTR;
    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD;

    let command_utf16: Vec<u16> = command.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        let result = ShellExecuteW(
            None,
            windows::core::w!("runas"),
            PCWSTR::from_raw(command_utf16.as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            SHOW_WINDOW_CMD(1), // SW_SHOWNORMAL
        );
        if result.0 <= 32 {
            // ShellExecute returns a handle > 32 on success
            Err("Failed to run command with elevated privileges".to_string())
        } else {
            Ok(())
        }
    }
}

// 进程管理命令
#[tauri::command]
pub fn start_sra_process_command(app_handle: tauri::AppHandle, arguments: Option<String>) -> Result<(), String> {
    process::start_sra_process_command(app_handle, arguments)
}

#[tauri::command]
pub fn stop_sra_process_command(app_handle: tauri::AppHandle) -> Result<(), String> {
    process::stop_sra_process_command(app_handle)
}

#[tauri::command]
pub fn restart_sra_process_command(app_handle: tauri::AppHandle, arguments: Option<String>) -> Result<(), String> {
    process::restart_sra_process_command(app_handle, arguments)
}

#[tauri::command]
pub fn get_sra_status() -> Result<String, String> {
    process::get_sra_status()
}

#[tauri::command]
pub fn send_input_to_sra(app_handle: tauri::AppHandle, input: String) -> Result<(), String> {
    process::send_input_to_sra(app_handle, input)
}

#[tauri::command]
pub fn task_run(app_handle: tauri::AppHandle, config_name: Option<String>) -> Result<(), String> {
    process::task_run(app_handle, config_name)
}

#[tauri::command]
pub fn task_stop(app_handle: tauri::AppHandle) -> Result<(), String> {
    process::task_stop(app_handle)
}

// 配置管理命令
#[tauri::command]
pub fn get_config_list() -> Result<Vec<String>, String> {
    config::get_config_list()
}

#[tauri::command]
pub fn load_config(name: String) -> Result<serde_json::Value, String> {
    config::load_config(name)
}

#[tauri::command]
pub fn save_config(config: serde_json::Value) -> Result<(), String> {
    config::save_config(config)
}

#[tauri::command]
pub fn create_default_config() -> Result<(), String> {
    config::create_default_config()
}

#[tauri::command]
pub fn create_new_config(name: String) -> Result<(), String> {
    config::create_new_config(name)
}

#[tauri::command]
pub fn delete_config(name: String) -> Result<(), String> {
    config::delete_config(name)
}

#[tauri::command]
pub fn save_task_order(task_order: Vec<String>) -> Result<(), String> {
    config::save_task_order(task_order)
}

#[tauri::command]
pub fn load_task_order() -> Result<Vec<String>, String> {
    config::load_task_order()
}

// 日志管理命令
#[tauri::command]
pub fn get_all_logs() -> Result<Vec<logger::LogMessage>, String> {
    logger::get_all_logs()
}

#[tauri::command]
pub fn log_from_frontend(source: String, level: String, message: String) -> Result<(), String> {
    let log_source = match source.as_str() {
        "前端" => logger::LogSource::Frontend,
        "后端" => logger::LogSource::Backend,
        "进程端" => logger::LogSource::Process,
        _ => logger::LogSource::Frontend,
    };
    
    let log_level = match level.as_str() {
        "INFO" => logger::LogLevel::INFO,
        "WARN" => logger::LogLevel::WARN,
        "ERR" => logger::LogLevel::ERR,
        "TRACE" => logger::LogLevel::TRACE,
        "DEBUG" => logger::LogLevel::DEBUG,
        "SUCCESS" => logger::LogLevel::SUCCESS,
        "MSG" => logger::LogLevel::MSG,
        _ => logger::LogLevel::INFO,
    };
    
    logger::log(log_source, log_level, message)
}

// 壁纸管理命令
#[tauri::command]
pub fn set_wallpaper(source_path: String) -> Result<String, String> {
    wallpaper::set_wallpaper(source_path)
}

#[tauri::command]
pub fn reset_wallpaper() -> Result<(), String> {
    wallpaper::reset_wallpaper()
}

#[tauri::command]
pub fn get_current_wallpaper() -> Result<Option<String>, String> {
    wallpaper::get_current_wallpaper()
}

#[tauri::command]
pub fn get_wallpaper_base64() -> Result<Option<String>, String> {
    wallpaper::get_wallpaper_base64()
}

// 公告管理命令
#[tauri::command]
pub async fn get_announcements() -> Result<Vec<announcement::Announcement>, String> {
    announcement::get_announcements().await
}

// 设置管理命令
#[tauri::command]
pub fn load_app_settings() -> Result<settings::AppSettings, String> {
    settings::load_settings()
}

#[tauri::command]
pub fn save_app_settings(settings: settings::AppSettings) -> Result<(), String> {
    settings::save_settings(settings)
}
