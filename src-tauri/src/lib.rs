// Tauri 应用主入口
// 模块化结构：将功能分离到不同模块中

mod types;
mod process;
mod config;
mod commands;
mod logger;
mod sra_parser;
mod wallpaper;
mod announcement;
mod settings;
mod encryption;

use std::thread;
use std::time::Duration;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::get_config_list,
            commands::load_config,
            commands::save_config,
            commands::create_default_config,
            commands::create_new_config,
            commands::delete_config,
            commands::save_task_order,
            commands::load_task_order,
            commands::start_sra_process_command,
            commands::stop_sra_process_command,
            commands::restart_sra_process_command,
            commands::get_sra_status,
            commands::send_input_to_sra,
            commands::task_run,
            commands::task_stop,
            commands::get_all_logs,
            commands::log_from_frontend,
            commands::set_wallpaper,
            commands::reset_wallpaper,
            commands::get_current_wallpaper,
            commands::get_wallpaper_base64,
            commands::get_announcements,
            commands::load_app_settings,
            commands::save_app_settings,
            #[cfg(target_os = "windows")]
            commands::run_elevated_command
        ])
        .setup(|app| {
            // 初始化日志系统
            if let Err(e) = logger::init_logger(app.handle().clone()) {
                eprintln!("Failed to initialize logger: {}", e);
            }
            
            // 初始化 SRA 日志解析器
            sra_parser::init_parser();
            
            // 异步启动SRA进程，不阻塞UI加载
            let app_handle = app.handle().clone();
            thread::spawn(move || {
                // 短暂延迟，让UI先加载
                thread::sleep(Duration::from_millis(500));
                if let Err(e) = process::start_sra_process_command(app_handle.clone(), None) {
                    use tauri::Emitter;
                    let _ = app_handle.emit("sra-status-changed", "error");
                    let _ = logger::log(
                        logger::LogSource::Backend,
                        logger::LogLevel::ERR,
                        format!("启动 SRA 进程失败: {}", e)
                    );
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
