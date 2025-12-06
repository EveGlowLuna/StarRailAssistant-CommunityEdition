// Tauri 命令定义模块

use crate::process;
use crate::config;
use crate::logger;
use crate::wallpaper;
use crate::announcement;
use crate::settings;
use crate::shortcut;

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
pub async fn get_announcements(lang: String) -> Result<Vec<announcement::Announcement>, String> {
    announcement::get_announcements(lang).await
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

#[tauri::command]
pub fn save_download_region(region: String) -> Result<(), String> {
    settings::save_download_region(region)
}

// 版本管理命令
#[tauri::command]
pub fn get_frontend_version(app_handle: tauri::AppHandle) -> Result<String, String> {
    // 从 Tauri 配置获取应用版本
    let version = app_handle.package_info().version.to_string();
    Ok(version)
}

#[tauri::command]
pub fn get_backend_version(_app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    use std::fs;
    
    // 获取当前可执行文件目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("Failed to get current exe path: {}", e))?
        .parent()
        .ok_or("Failed to get parent directory")?
        .to_path_buf();
    
    // 构建 version.json 路径（与可执行文件在同一目录）
    let version_path = exe_dir.join("version.json");
    
    // 尝试读取 version.json
    match fs::read_to_string(&version_path) {
        Ok(content) => {
            // 解析 JSON
            let version_data: serde_json::Value = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse version.json: {}", e))?;
            
            Ok(version_data)
        },
        Err(_) => {
            // 文件不存在，返回未知版本
            Ok(serde_json::json!({
                "version": "未知",
                "channel": "stable"
            }))
        }
    }
}

#[tauri::command]
pub async fn get_remote_versions() -> Result<serde_json::Value, String> {
    // 使用 reqwest 获取远程版本信息
    let response = reqwest::get("https://services.starrailassistant.xyz/get-latest-version")
        .await
        .map_err(|e| format!("Failed to fetch remote versions: {}", e))?;
    
    let data = response.json::<serde_json::Value>()
        .await
        .map_err(|e| format!("Failed to parse remote versions: {}", e))?;
    
    Ok(data)
}

// 检查磁盘空间
#[tauri::command]
pub fn check_disk_space(required_bytes: u64) -> Result<bool, String> {
    use std::env;
    
    let temp_dir = env::temp_dir();
    
    #[cfg(target_os = "windows")]
    {
        use std::path::Path;
        
        // 获取驱动器根路径
        let drive = temp_dir.to_str()
            .and_then(|s| s.chars().take(3).collect::<String>().into())
            .unwrap_or_else(|| "C:\\".to_string());
        
        match std::fs::metadata(Path::new(&drive)) {
            Ok(_) => {
                // Windows 上使用 GetDiskFreeSpaceEx API
                use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
                use windows::core::PCWSTR;
                
                let drive_wide: Vec<u16> = drive.encode_utf16().chain(std::iter::once(0)).collect();
                let mut free_bytes: u64 = 0;
                
                unsafe {
                    let result = GetDiskFreeSpaceExW(
                        PCWSTR::from_raw(drive_wide.as_ptr()),
                        None,
                        None,
                        Some(&mut free_bytes as *mut u64 as *mut _)
                    );
                    
                    if result.is_ok() {
                        Ok(free_bytes > required_bytes)
                    } else {
                        // 如果 API 调用失败，假设有足够空间
                        Ok(true)
                    }
                }
            },
            Err(_) => Ok(true) // 无法获取元数据，假设有足够空间
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // 非 Windows 平台暂不支持，假设有足够空间
        let _ = required_bytes; // 避免未使用警告
        Ok(true)
    }
}

// 删除临时文件
#[tauri::command]
pub fn delete_temp_file(file_path: String) -> Result<(), String> {
    use std::fs;
    use std::path::Path;
    
    let path = Path::new(&file_path);
    if path.exists() {
        fs::remove_file(path)
            .map_err(|e| format!("Failed to delete file: {}", e))?;
    }
    Ok(())
}

// 下载更新文件
#[tauri::command]
pub async fn download_update(
    app_handle: tauri::AppHandle,
    download_url: String,
    file_name: String,
    _update_type: String
) -> Result<String, String> {
    use std::fs::File;
    use std::io::Write;
    use tauri::Emitter;
    
    // 获取临时目录
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(&file_name);
    
    // 创建文件
    let mut file = File::create(&file_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    // 开始下载
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300)) // 5分钟超时
        .build()
        .map_err(|e| format!("Failed to create client: {}", e))?;
    
    let mut response = client.get(&download_url)
        .send()
        .await
        .map_err(|e| format!("Failed to start download: {}", e))?;
    
    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    
    let start_time = std::time::Instant::now();
    let mut last_update = std::time::Instant::now();
    
    while let Some(chunk) = response.chunk().await.map_err(|e| format!("Failed to read chunk: {}", e))? {
        file.write_all(&chunk)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        downloaded += chunk.len() as u64;
        
        // 每100ms发送一次进度更新
        if last_update.elapsed().as_millis() >= 100 {
            let elapsed = start_time.elapsed().as_secs_f64();
            let speed = if elapsed > 0.0 {
                downloaded as f64 / elapsed
            } else {
                0.0
            };
            
            let percentage = if total_size > 0 {
                (downloaded as f64 / total_size as f64) * 100.0
            } else {
                0.0
            };
            
            let _ = app_handle.emit("download-progress", serde_json::json!({
                "downloaded": downloaded,
                "total": total_size,
                "speed": speed,
                "percentage": percentage
            }));
            
            last_update = std::time::Instant::now();
        }
    }
    
    // 发送完成事件
    let _ = app_handle.emit("download-progress", serde_json::json!({
        "downloaded": downloaded,
        "total": total_size,
        "speed": 0,
        "percentage": 100.0
    }));
    
    Ok(file_path.to_string_lossy().to_string())
}

// 安装前端更新
#[tauri::command]
pub fn install_frontend_update(installer_path: String) -> Result<(), String> {
    use std::fs;
    use std::process::Command;
    
    // 获取当前应用路径
    let app_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get app path: {}", e))?;
    
    // 获取临时目录
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("sra_update.bat");
    
    // Windows: 获取短路径（8.3 格式）以避免特殊字符问题
    #[cfg(target_os = "windows")]
    let (installer_short, app_short) = {
        use windows::core::PCWSTR;
        use windows::Win32::Storage::FileSystem::GetShortPathNameW;
        
        // 获取安装包短路径
        let installer_wide: Vec<u16> = installer_path.encode_utf16().chain(std::iter::once(0)).collect();
        let mut installer_short_buf = [0u16; 260];
        let installer_short_len = unsafe {
            GetShortPathNameW(
                PCWSTR::from_raw(installer_wide.as_ptr()),
                Some(&mut installer_short_buf)
            )
        };
        
        let installer_short = if installer_short_len > 0 && installer_short_len < 260 {
            String::from_utf16_lossy(&installer_short_buf[..installer_short_len as usize])
        } else {
            installer_path.clone()
        };
        
        // 获取应用短路径
        let app_wide: Vec<u16> = app_path.to_string_lossy().encode_utf16().chain(std::iter::once(0)).collect();
        let mut app_short_buf = [0u16; 260];
        let app_short_len = unsafe {
            GetShortPathNameW(
                PCWSTR::from_raw(app_wide.as_ptr()),
                Some(&mut app_short_buf)
            )
        };
        
        let app_short = if app_short_len > 0 && app_short_len < 260 {
            String::from_utf16_lossy(&app_short_buf[..app_short_len as usize])
        } else {
            app_path.to_string_lossy().to_string()
        };
        
        (installer_short, app_short)
    };
    
    #[cfg(not(target_os = "windows"))]
    let (installer_short, app_short) = (installer_path.clone(), app_path.to_string_lossy().to_string());
    
    // 记录路径用于调试
    println!("Original installer path: {}", installer_path);
    println!("Short installer path: {}", installer_short);
    println!("Original app path: {}", app_path.display());
    println!("Short app path: {}", app_short);
    
    // 生成批处理脚本（使用短路径，直接构建字符串）
    let mut script_content = String::new();
    script_content.push_str("@echo off\r\n");
    script_content.push_str("set LOG_FILE=%TEMP%\\sra_update.log\r\n");
    script_content.push_str("set ERROR_FILE=%TEMP%\\sra_update_error.txt\r\n");
    script_content.push_str("\r\n");
    script_content.push_str("echo ========================================= >> %LOG_FILE%\r\n");
    script_content.push_str("echo [%date% %time%] 开始更新流程 >> %LOG_FILE%\r\n");
    script_content.push_str(&format!("echo 安装包路径: {} >> %LOG_FILE%\r\n", installer_short));
    script_content.push_str(&format!("echo 应用路径: {} >> %LOG_FILE%\r\n", app_short));
    script_content.push_str("echo ========================================= >> %LOG_FILE%\r\n");
    script_content.push_str("\r\n");
    script_content.push_str("REM 检查安装包是否存在\r\n");
    script_content.push_str(&format!("if not exist {} (\r\n", installer_short));
    script_content.push_str("    echo [%date% %time%] 错误: 安装包不存在 >> %LOG_FILE%\r\n");
    script_content.push_str("    echo 安装包文件不存在 > %ERROR_FILE%\r\n");
    script_content.push_str("    goto cleanup\r\n");
    script_content.push_str(")\r\n");
    script_content.push_str("\r\n");
    script_content.push_str("REM 延迟2秒确保应用完全退出\r\n");
    script_content.push_str("echo [%date% %time%] 等待应用退出... >> %LOG_FILE%\r\n");
    script_content.push_str("ping 127.0.0.1 -n 3 >nul\r\n");
    script_content.push_str("echo [%date% %time%] 延迟完成，开始安装 >> %LOG_FILE%\r\n");
    script_content.push_str("\r\n");
    script_content.push_str("REM 静默安装\r\n");
    script_content.push_str("echo [%date% %time%] 执行安装程序... >> %LOG_FILE%\r\n");
    script_content.push_str(&format!("start \"\" /wait {} /SILENT /SUPPRESSMSGBOXES /NORESTART /SP-\r\n", installer_short));
    script_content.push_str("\r\n");
    script_content.push_str("if %ERRORLEVEL% EQU 0 (\r\n");
    script_content.push_str("    echo [%date% %time%] 安装成功 >> %LOG_FILE%\r\n");
    script_content.push_str("    \r\n");
    script_content.push_str("    REM 检查应用是否存在\r\n");
    script_content.push_str(&format!("    if exist {} (\r\n", app_short));
    script_content.push_str("        echo [%date% %time%] 启动新版本应用... >> %LOG_FILE%\r\n");
    script_content.push_str(&format!("        start \"\" {}\r\n", app_short));
    script_content.push_str("        echo [%date% %time%] 应用已启动 >> %LOG_FILE%\r\n");
    script_content.push_str("    ) else (\r\n");
    script_content.push_str("        echo [%date% %time%] 警告: 应用文件不存在 >> %LOG_FILE%\r\n");
    script_content.push_str("        echo 应用文件不存在，请手动启动 > %ERROR_FILE%\r\n");
    script_content.push_str("    )\r\n");
    script_content.push_str(") else (\r\n");
    script_content.push_str("    echo [%date% %time%] 安装失败，错误代码: %ERRORLEVEL% >> %LOG_FILE%\r\n");
    script_content.push_str("    echo 安装失败，错误代码: %ERRORLEVEL% > %ERROR_FILE%\r\n");
    script_content.push_str(")\r\n");
    script_content.push_str("\r\n");
    script_content.push_str(":cleanup\r\n");
    script_content.push_str("REM 删除安装包\r\n");
    script_content.push_str(&format!("if exist {} (\r\n", installer_short));
    script_content.push_str("    echo [%date% %time%] 删除安装包... >> %LOG_FILE%\r\n");
    script_content.push_str(&format!("    del {}\r\n", installer_short));
    script_content.push_str(")\r\n");
    script_content.push_str("\r\n");
    script_content.push_str("echo [%date% %time%] 更新流程结束 >> %LOG_FILE%\r\n");
    script_content.push_str("echo ========================================= >> %LOG_FILE%\r\n");
    script_content.push_str("\r\n");
    script_content.push_str("REM 删除脚本自身\r\n");
    script_content.push_str("del %~f0\r\n");
    
    // 写入脚本
    fs::write(&script_path, script_content)
        .map_err(|e| format!("Failed to write script: {}", e))?;
    
    // 启动脚本
    Command::new("cmd")
        .args(["/c", "start", "", "/b", script_path.to_str().unwrap()])
        .spawn()
        .map_err(|e| format!("Failed to start script: {}", e))?;
    
    // 退出应用
    std::process::exit(0);
}

// 安装后端更新
#[tauri::command]
pub async fn install_backend_update(
    app_handle: tauri::AppHandle,
    zip_path: String
) -> Result<(), String> {
    use std::fs::{File, create_dir_all, remove_dir_all};
    use std::io::copy;
    use zip::ZipArchive;
    use tauri::Emitter;
    
    // 1. 停止SRA进程
    let _ = app_handle.emit("backend-update-progress", "stopping");
    crate::process::stop_sra_process_command(app_handle.clone())?;
    
    // 等待进程完全停止
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 2. 获取应用目录
    let _ = app_handle.emit("backend-update-progress", "extracting");
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?
        .parent()
        .ok_or("Failed to get parent directory")?
        .to_path_buf();
    
    // 3. 创建备份目录
    let backup_dir = exe_dir.join("StarRailAssistant_backup");
    let sra_dir = exe_dir.join("StarRailAssistant");
    
    // 如果存在旧备份，先删除
    if backup_dir.exists() {
        let _ = remove_dir_all(&backup_dir);
    }
    
    // 备份当前后端文件
    if sra_dir.exists() {
        create_dir_all(&backup_dir)
            .map_err(|e| format!("Failed to create backup dir: {}", e))?;
        
        // 复制整个目录
        copy_dir_recursive(&sra_dir, &backup_dir)
            .map_err(|e| format!("Failed to backup files: {}", e))?;
    }
    
    // 4. 解压zip文件
    let file = File::open(&zip_path)
        .map_err(|e| format!("Failed to open zip: {}", e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read zip: {}", e))?;
    
    let extract_result = (|| -> Result<(), String> {
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| format!("Failed to read file in zip: {}", e))?;
            
            let outpath = exe_dir.join(file.name());
            
            if file.is_dir() {
                create_dir_all(&outpath)
                    .map_err(|e| format!("Failed to create dir: {}", e))?;
            } else {
                if let Some(p) = outpath.parent() {
                    create_dir_all(p)
                        .map_err(|e| format!("Failed to create parent dir: {}", e))?;
                }
                
                let mut outfile = File::create(&outpath)
                    .map_err(|e| format!("Failed to create file: {}", e))?;
                
                copy(&mut file, &mut outfile)
                    .map_err(|e| format!("Failed to copy file: {}", e))?;
            }
        }
        Ok(())
    })();
    
    // 5. 如果解压失败，恢复备份
    if let Err(e) = extract_result {
        if backup_dir.exists() {
            // 删除损坏的文件
            if sra_dir.exists() {
                let _ = remove_dir_all(&sra_dir);
            }
            
            // 恢复备份
            if let Err(restore_err) = copy_dir_recursive(&backup_dir, &sra_dir) {
                return Err(format!("Extract failed and restore failed: {} | {}", e, restore_err));
            }
            
            // 删除备份
            let _ = remove_dir_all(&backup_dir);
            
            return Err(format!("Extract failed, backup restored: {}", e));
        }
        return Err(e);
    }
    
    // 6. 删除临时zip文件
    let _ = std::fs::remove_file(&zip_path);
    
    // 7. 删除备份（更新成功）
    if backup_dir.exists() {
        let _ = remove_dir_all(&backup_dir);
    }
    
    // 8. 重启SRA进程
    let _ = app_handle.emit("backend-update-progress", "restarting");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 如果重启失败，尝试恢复备份
    if let Err(e) = crate::process::start_sra_process_command(app_handle.clone(), None) {
        return Err(format!("Failed to restart backend: {}", e));
    }
    
    // 9. 发送完成事件
    let _ = app_handle.emit("backend-update-progress", "completed");
    
    Ok(())
}

// 递归复制目录的辅助函数
fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    use std::fs::{create_dir_all, copy as fs_copy, read_dir};
    
    if !dst.exists() {
        create_dir_all(dst)?;
    }
    
    for entry in read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs_copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

// 订阅管理命令
#[tauri::command]
pub fn get_subscription() -> Result<Option<serde_json::Value>, String> {
    let subscription = settings::get_subscription()?;
    match subscription {
        Some(sub) => {
            let value = serde_json::to_value(&sub)
                .map_err(|e| format!("Failed to serialize subscription: {}", e))?;
            Ok(Some(value))
        },
        None => Ok(None)
    }
}

#[tauri::command]
pub fn save_subscription(r#type: String, channel: String, version: String) -> Result<(), String> {
    settings::save_subscription(r#type, channel, version)
}

// 快捷方式管理命令
#[tauri::command]
pub fn check_desktop_shortcut_needed() -> Result<bool, String> {
    // 检查是否需要提示创建快捷方式
    let settings = settings::load_settings()?;
    
    // 如果用户选择了"不再提示"，直接返回false
    if settings.skip_desktop_shortcut_prompt {
        return Ok(false);
    }
    
    // 检查是否有桌面文件夹
    let has_desktop = shortcut::has_desktop_folder()?;
    if !has_desktop {
        return Ok(false);
    }
    
    // 检查是否已有快捷方式
    let has_shortcut = shortcut::has_desktop_shortcut()?;
    Ok(!has_shortcut)
}

#[tauri::command]
pub fn create_desktop_shortcut() -> Result<(), String> {
    shortcut::create_desktop_shortcut()
}

#[tauri::command]
pub fn save_skip_shortcut_prompt() -> Result<(), String> {
    shortcut::save_skip_prompt_setting()
}
