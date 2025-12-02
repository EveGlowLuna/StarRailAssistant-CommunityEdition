// 日志系统模块
// 负责日志的收集、存储和分发

use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Emitter;

// 日志级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    INFO,
    WARN,
    ERR,
    TRACE,
    DEBUG,
    SUCCESS,
    MSG,
}

impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::INFO => "INFO",
            LogLevel::WARN => "WARN",
            LogLevel::ERR => "ERR",
            LogLevel::TRACE => "TRACE",
            LogLevel::DEBUG => "DEBUG",
            LogLevel::SUCCESS => "SUCCESS",
            LogLevel::MSG => "MSG",
        }
    }
}

// 日志来源
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogSource {
    #[serde(rename = "后端")]
    Backend,
    #[serde(rename = "进程端")]
    Process,
    #[serde(rename = "前端")]
    Frontend,
}

impl LogSource {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        match self {
            LogSource::Backend => "后端",
            LogSource::Process => "进程端",
            LogSource::Frontend => "前端",
        }
    }
}

// 日志消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMessage {
    pub source: LogSource,
    pub level: LogLevel,
    pub message: String,
    pub time: String, // ISO 8601 格式
}

// 日志管理器
pub struct LogManager {
    log_file: Mutex<Option<File>>,
    log_buffer: Mutex<Vec<LogMessage>>,
    app_handle: Mutex<Option<tauri::AppHandle>>,
}

impl LogManager {
    pub fn new() -> Self {
        Self {
            log_file: Mutex::new(None),
            log_buffer: Mutex::new(Vec::new()),
            app_handle: Mutex::new(None),
        }
    }

    // 初始化日志系统
    pub fn initialize(&self, app_handle: tauri::AppHandle) -> Result<(), String> {
        // 保存 app_handle
        let mut handle_guard = self.app_handle.lock().map_err(|e| format!("Lock error: {}", e))?;
        *handle_guard = Some(app_handle);
        drop(handle_guard);

        // 创建日志文件
        let log_path = self.create_log_file()?;
        
        // 记录启动日志
        self.log(
            LogSource::Backend,
            LogLevel::INFO,
            format!("日志系统已启动，日志文件: {}", log_path.display()),
        )?;

        Ok(())
    }

    // 创建日志文件
    fn create_log_file(&self) -> Result<PathBuf, String> {
        // 获取 APPDATA 路径
        let appdata = std::env::var("APPDATA").map_err(|_| "Failed to get APPDATA path")?;
        let log_dir = PathBuf::from(appdata).join("SRA").join("SRA-CE-Logs");

        // 确保目录存在
        std::fs::create_dir_all(&log_dir)
            .map_err(|e| format!("Failed to create log directory: {}", e))?;

        // 生成日志文件名: log-2025-11-29_13-20-11.log
        let now = chrono::Local::now();
        let filename = format!("log-{}.log", now.format("%Y-%m-%d_%H-%M-%S"));
        let log_path = log_dir.join(&filename);

        // 创建日志文件
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(|e| format!("Failed to create log file: {}", e))?;

        let mut file_guard = self.log_file.lock().map_err(|e| format!("Lock error: {}", e))?;
        *file_guard = Some(file);

        Ok(log_path)
    }

    // 记录日志
    pub fn log(&self, source: LogSource, level: LogLevel, message: String) -> Result<(), String> {
        let now = chrono::Local::now();
        let time = now.to_rfc3339(); // ISO 8601 格式

        let log_msg = LogMessage {
            source: source.clone(),
            level: level.clone(),
            message: message.clone(),
            time: time.clone(),
        };

        // 添加到缓冲区
        let mut buffer_guard = self.log_buffer.lock().map_err(|e| format!("Lock error: {}", e))?;
        buffer_guard.push(log_msg.clone());
        
        // 限制缓冲区大小（保留最近 1000 条）
        if buffer_guard.len() > 1000 {
            buffer_guard.remove(0);
        }
        drop(buffer_guard);

        // 写入文件: 2024-11-29 13:45:30 [INFO] 消息内容
        let formatted_time = now.format("%Y-%m-%d %H:%M:%S");
        let log_line = format!(
            "{} [{}] {}\n",
            formatted_time,
            level.as_str(),
            message
        );

        let mut file_guard = self.log_file.lock().map_err(|e| format!("Lock error: {}", e))?;
        if let Some(ref mut file) = *file_guard {
            file.write_all(log_line.as_bytes())
                .map_err(|e| format!("Failed to write log: {}", e))?;
            file.flush()
                .map_err(|e| format!("Failed to flush log: {}", e))?;
        }
        drop(file_guard);

        // 发送到前端（如果前端正在监听）
        let handle_guard = self.app_handle.lock().map_err(|e| format!("Lock error: {}", e))?;
        if let Some(ref app_handle) = *handle_guard {
            let _ = app_handle.emit("log-message", log_msg);
        }

        Ok(())
    }

    // 获取所有日志
    pub fn get_all_logs(&self) -> Result<Vec<LogMessage>, String> {
        let buffer_guard = self.log_buffer.lock().map_err(|e| format!("Lock error: {}", e))?;
        Ok(buffer_guard.clone())
    }
}

// 全局日志管理器
static LOG_MANAGER: Mutex<Option<LogManager>> = Mutex::new(None);

// 初始化全局日志管理器
pub fn init_logger(app_handle: tauri::AppHandle) -> Result<(), String> {
    let manager = LogManager::new();
    manager.initialize(app_handle)?;
    
    let mut guard = LOG_MANAGER.lock().map_err(|e| format!("Lock error: {}", e))?;
    *guard = Some(manager);
    
    Ok(())
}

// 记录日志（全局函数）
pub fn log(source: LogSource, level: LogLevel, message: String) -> Result<(), String> {
    let guard = LOG_MANAGER.lock().map_err(|e| format!("Lock error: {}", e))?;
    if let Some(ref manager) = *guard {
        manager.log(source, level, message)
    } else {
        Err("Logger not initialized".to_string())
    }
}

// 获取所有日志（全局函数）
pub fn get_all_logs() -> Result<Vec<LogMessage>, String> {
    let guard = LOG_MANAGER.lock().map_err(|e| format!("Lock error: {}", e))?;
    if let Some(ref manager) = *guard {
        manager.get_all_logs()
    } else {
        Err("Logger not initialized".to_string())
    }
}
