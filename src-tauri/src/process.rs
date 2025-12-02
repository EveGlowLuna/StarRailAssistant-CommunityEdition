// SRA 进程管理模块

use crate::types::{SraProcess, SraStatus};
use crate::logger::{log, LogSource};
use crate::sra_parser;
use std::sync::Mutex;
use std::process::{Child, Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;
use tauri::Emitter;

pub static SRA_PROCESS: Mutex<Option<SraProcess>> = Mutex::new(None);

// 启动 SRA-cli 进程
pub fn start_sra_process(app_handle: tauri::AppHandle, arguments: Option<String>) -> Result<Child, String> {
    
    // 获取当前可执行文件目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("Failed to get current exe path: {}", e))?
        .parent()
        .ok_or("Failed to get parent directory")?
        .to_path_buf();
    
    let (sra_cli_path, working_dir) = if let Some(target_dir) = exe_dir.parent() {
        if let Some(src_tauri_dir) = target_dir.parent() {
            if let Some(project_root) = src_tauri_dir.parent() {
                let dev_path = project_root.join("StarRailAssistant").join("SRA-cli.exe");
                let dev_working_dir = project_root.join("StarRailAssistant");
                
                if dev_path.exists() {
                    (dev_path, dev_working_dir)
                } else {
                    let prod_path = exe_dir.join("SRA-cli.exe");
                    if prod_path.exists() {
                        (prod_path, exe_dir.clone())
                    } else {
                        let error_msg = format!(
                            "无法找到文件 SRA-cli.exe，请检查安装完整性。\n路径: {}",
                            prod_path.display()
                        );
                        let _ = app_handle.emit("console-output", error_msg.clone());
                        return Err(error_msg);
                    }
                }
            } else {
                let prod_path = exe_dir.join("SRA-cli.exe");
                if prod_path.exists() {
                    (prod_path, exe_dir.clone())
                } else {
                    let error_msg = format!(
                        "无法找到文件 SRA-cli.exe，请检查安装完整性。\n路径: {}",
                        prod_path.display()
                    );
                    let _ = app_handle.emit("console-output", error_msg.clone());
                    return Err(error_msg);
                }
            }
        } else {
            let prod_path = exe_dir.join("SRA-cli.exe");
            if prod_path.exists() {
                (prod_path, exe_dir.clone())
            } else {
                let error_msg = format!(
                    "无法找到文件 SRA-cli.exe，请检查安装完整性。\n路径: {}",
                    prod_path.display()
                );
                let _ = app_handle.emit("console-output", error_msg.clone());
                return Err(error_msg);
            }
        }
    } else {
        let prod_path = exe_dir.join("SRA-cli.exe");
        if prod_path.exists() {
            (prod_path, exe_dir.clone())
        } else {
            let error_msg = format!(
                "无法找到文件 SRA-cli.exe，请检查安装完整性。\n路径: {}",
                prod_path.display()
            );
            let _ = app_handle.emit("console-output", error_msg.clone());
            return Err(error_msg);
        }
    };
    
    // 构建命令
    let mut command = Command::new(&sra_cli_path);
    
    // 添加 --inline 参数（隐藏 prompt）
    command.arg("--inline");
    
    // 添加用户指定的参数
    if let Some(args) = arguments {
        for arg in args.split_whitespace() {
            command.arg(arg);
        }
    }
    
    command
        .current_dir(&working_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("PYTHONUNBUFFERED", "1")  // 强制 Python 无缓冲输出
        .env("PYTHONIOENCODING", "utf-8");  // 强制 Python 使用 UTF-8 编码
    
    // 生产环境下隐藏命令行窗口
    #[cfg(target_os = "windows")]
    {
        if !cfg!(debug_assertions) {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            command.creation_flags(CREATE_NO_WINDOW);
        }
    }
    
    let mut child = command
        .spawn()
        .map_err(|e| format!("启动失败: {}", e))?;
    
    // 获取标准输出和错误输出用于日志解析
    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;
    
    // 启动定时刷新线程（使用更长的间隔减少 callback 调用）
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1)); // 从 300ms 改为 1s
            
            // 刷新待处理的日志
            let flushed_logs = sra_parser::flush_parser();
            for parsed in flushed_logs {
                let _ = log(LogSource::Process, parsed.level, parsed.message);
            }
            
            // 检查进程是否还在运行
            let process_guard = SRA_PROCESS.lock();
            if let Ok(guard) = process_guard {
                if let Some(ref proc) = *guard {
                    if proc.child.is_none() {
                        break; // 进程已退出，停止刷新线程
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    });
    
    // 启动线程读取并解析标准输出
    let app_handle_clone = app_handle.clone();
    thread::spawn(move || {
        use std::io::Read;
        let mut reader = BufReader::new(stdout);
        
        // 记录开始读取
        let _ = log(LogSource::Backend, crate::logger::LogLevel::DEBUG, "开始读取 SRA 进程输出".to_string());
        
        let mut buffer = String::new();
        let mut read_buf = [0u8; 4096];
        
        loop {
            match reader.read(&mut read_buf) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    // 将读取的字节转换为字符串（处理 UTF-8）
                    let text = String::from_utf8_lossy(&read_buf[..n]);
                    buffer.push_str(&text);
                    
                    // 按行分割处理
                    while let Some(pos) = buffer.find('\n') {
                        let line = buffer[..pos].to_string();
                        buffer = buffer[pos + 1..].to_string();
                        
                        // 过滤空行
                        let trimmed = line.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        
                        // 调试：记录原始行
                        let _ = log(LogSource::Backend, crate::logger::LogLevel::TRACE, format!("[RAW] {}", line));
                        
                        // 如果是 prompt，触发刷新但不解析
                        if trimmed == "sra>" {
                            let flushed_logs = sra_parser::flush_parser();
                            for parsed in flushed_logs {
                                let _ = log(LogSource::Process, parsed.level, parsed.message);
                            }
                            continue;
                        }
                        
                        // 解析日志
                        let parsed_logs = sra_parser::parse_line(&line);
                        for parsed in parsed_logs {
                            let _ = log(LogSource::Process, parsed.level, parsed.message);
                        }
                        
                        // 更新运行状态
                        if line.contains("[Start]") {
                            let _ = app_handle_clone.emit("sra-status-changed", "task-running");
                        } else if line.contains("[Done]") {
                            let _ = app_handle_clone.emit("sra-status-changed", "running");
                        }
                    }
                }
                Err(e) => {
                    let _ = log(LogSource::Backend, crate::logger::LogLevel::ERR, format!("读取 stdout 错误: {}", e));
                    break;
                }
            }
        }
        
        // 处理剩余的缓冲区内容
        if !buffer.trim().is_empty() {
            let parsed_logs = sra_parser::parse_line(&buffer);
            for parsed in parsed_logs {
                let _ = log(LogSource::Process, parsed.level, parsed.message);
            }
        }
        
        // 进程退出，刷新剩余的日志
        let flushed_logs = sra_parser::flush_parser();
        for parsed in flushed_logs {
            let _ = log(LogSource::Process, parsed.level, parsed.message);
        }
        
        // 更新进程状态为已退出
        let mut process_guard = SRA_PROCESS.lock().unwrap();
        if let Some(ref mut proc) = *process_guard {
            proc.child = None;
            proc.status = SraStatus::NotRunning;
        }
        drop(process_guard);
        
        let _ = app_handle_clone.emit("sra-status-changed", "not-running");
        let _ = log(LogSource::Backend, crate::logger::LogLevel::WARN, "SRA 进程意外退出".to_string());
    });
    
    // 启动线程读取并解析错误输出
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.trim().is_empty() {
                    continue;
                }
                
                // 解析日志（可能返回多条）
                let parsed_logs = sra_parser::parse_line(&line);
                for parsed in parsed_logs {
                    let _ = log(LogSource::Process, parsed.level, parsed.message);
                }
            }
        }
    });
    
    Ok(child)
}

// 启动SRA进程
pub fn start_sra_process_command(app_handle: tauri::AppHandle, arguments: Option<String>) -> Result<(), String> {
    let mut process_guard = SRA_PROCESS.lock().map_err(|e| format!("Lock error: {}", e))?;
    
    // 检查是否已经在运行
    if let Some(ref mut proc) = *process_guard {
        if let Some(ref mut child) = proc.child {
            // 检查进程是否还活着
            match child.try_wait() {
                Ok(None) => {
                    // 进程还在运行，不重复启动
                    return Ok(());
                }
                _ => {
                    // 进程已退出，继续启动新进程
                }
            }
        }
    }
    
    // 初始化解析器
    sra_parser::init_parser();
    
    // 启动新进程
    match start_sra_process(app_handle.clone(), arguments.clone()) {
        Ok(child) => {
            *process_guard = Some(SraProcess {
                child: Some(child),
                status: SraStatus::Running,
            });
            
            // 记录启动日志
            let _ = log(LogSource::Backend, crate::logger::LogLevel::INFO, "SRA 进程已启动".to_string());
            
            // 发送状态变更事件
            let _ = app_handle.emit("sra-status-changed", "running");
            Ok(())
        }
        Err(e) => {
            *process_guard = Some(SraProcess {
                child: None,
                status: SraStatus::Error,
            });
            
            let _ = log(LogSource::Backend, crate::logger::LogLevel::ERR, format!("启动失败: {}", e));
            let _ = app_handle.emit("sra-status-changed", "error");
            Err(e)
        }
    }
}

// 停止SRA进程
pub fn stop_sra_process_command(app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut process_guard = SRA_PROCESS.lock().map_err(|e| format!("Lock error: {}", e))?;
    
    if let Some(ref mut proc) = *process_guard {
        if let Some(ref mut child) = proc.child {
            if child.try_wait().map_or(false, |status| status.is_some()) {
                // 进程已经退出
                proc.child = None;
                proc.status = SraStatus::NotRunning;
                return Ok(());
            }
            
            // 优先发送停止命令，优雅退出
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(b"exit\n");
                let _ = stdin.flush();
            }
            
            // 等待1秒，若未退出则强制终止
            thread::sleep(Duration::from_secs(1));
            if child.try_wait().map_or(false, |status| status.is_some()) {
                // 进程已正常退出
                proc.child = None;
                proc.status = SraStatus::NotRunning;
                let _ = app_handle.emit("sra-status-changed", "not-running");
                return Ok(());
            }
            
            // 强制终止
            let _ = child.kill();
            let _ = child.wait();
            
            proc.child = None;
            proc.status = SraStatus::NotRunning;
            let _ = app_handle.emit("sra-status-changed", "not-running");
            return Ok(());
        }
    }
    
    Ok(())
}

// 重启SRA进程
pub fn restart_sra_process_command(app_handle: tauri::AppHandle, arguments: Option<String>) -> Result<(), String> {
    // 先停止进程
    stop_sra_process_command(app_handle.clone())?;
    
    // 等待进程完全退出
    thread::sleep(Duration::from_millis(500));
    
    // 启动新进程，传播错误
    start_sra_process_command(app_handle, arguments)
}

// 获取SRA状态
pub fn get_sra_status() -> Result<String, String> {
    let process_guard = SRA_PROCESS.lock().map_err(|e| format!("Lock error: {}", e))?;
    
    if let Some(ref proc) = *process_guard {
        let status = match proc.status {
            SraStatus::NotRunning => "not-running",
            SraStatus::Running => "running",
            SraStatus::TaskRunning => "task-running",
            SraStatus::Error => "error",
        };
        Ok(status.to_string())
    } else {
        Ok("not-running".to_string())
    }
}

// 发送输入到进程
pub fn send_input_to_sra(_app_handle: tauri::AppHandle, input: String) -> Result<(), String> {
    // 检查输入是否为空
    if input.trim().is_empty() {
        let _ = log(LogSource::Backend, crate::logger::LogLevel::WARN, "尝试发送空输入到 SRA 进程".to_string());
        return Err("输入不能为空".to_string());
    }
    
    let mut process_guard = SRA_PROCESS.lock().map_err(|e| format!("Lock error: {}", e))?;
    
    // 检查进程是否存在
    if let Some(ref mut proc) = *process_guard {
        if let Some(ref mut child) = proc.child {
            // 检查进程是否还在运行
            if child.try_wait().map_or(false, |status| status.is_some()) {
                let error_msg = format!("发送失败: 进程未运行（输入: {}）", input);
                let _ = log(LogSource::Backend, crate::logger::LogLevel::WARN, error_msg.clone());
                return Err("进程未运行".to_string());
            }
            
            // 发送输入到 stdin
            if let Some(ref mut stdin) = child.stdin {
                // 记录发送日志
                let _ = log(
                    LogSource::Backend, 
                    crate::logger::LogLevel::INFO, 
                    format!("发送输入到 SRA 进程: {}", input)
                );
                
                // 写入命令
                let command = format!("{}\n", input);
                match stdin.write_all(command.as_bytes()) {
                    Ok(_) => {
                        match stdin.flush() {
                            Ok(_) => {
                                // 发送成功，记录用户输入为 MSG 日志
                                let _ = log(
                                    LogSource::Process,
                                    crate::logger::LogLevel::MSG,
                                    format!(">>> {}", input)
                                );
                                return Ok(());
                            }
                            Err(e) => {
                                let error_msg = format!("发送失败: {}（输入: {}）", e, input);
                                let _ = log(LogSource::Backend, crate::logger::LogLevel::ERR, error_msg.clone());
                                return Err(format!("刷新失败: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        let error_msg = format!("发送失败: {}（输入: {}）", e, input);
                        let _ = log(LogSource::Backend, crate::logger::LogLevel::ERR, error_msg.clone());
                        return Err(format!("写入失败: {}", e));
                    }
                }
            }
        }
    }
    
    // 进程不存在或未运行
    let error_msg = format!("发送失败: 进程未运行（输入: {}）", input);
    let _ = log(LogSource::Backend, crate::logger::LogLevel::WARN, error_msg);
    Err("进程未运行".to_string())
}

// 执行任务
pub fn task_run(app_handle: tauri::AppHandle, config_name: Option<String>) -> Result<(), String> {
    let command = if let Some(config) = config_name {
        if config.is_empty() {
            "task run".to_string()
        } else {
            format!("task run {}", config)
        }
    } else {
        "task run".to_string()
    };
    
    send_input_to_sra(app_handle, command)
}

// 停止任务
pub fn task_stop(app_handle: tauri::AppHandle) -> Result<(), String> {
    send_input_to_sra(app_handle, "task stop".to_string())
}
