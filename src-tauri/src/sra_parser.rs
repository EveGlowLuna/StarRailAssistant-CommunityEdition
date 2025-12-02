// SRA-cli 日志解析器
// 解析 SRA-cli 进程的输出并转换为结构化日志

use crate::logger::LogLevel;
use regex::Regex;
use std::sync::Mutex;

// 日志解析结果
#[derive(Debug, Clone)]
pub struct ParsedLog {
    pub level: LogLevel,
    pub message: String,
    #[allow(dead_code)]
    pub time: String, // HH:MM:SS 格式（保留用于调试）
}

// 日志解析器
pub struct SraLogParser {
    log_pattern: Regex,
    pending_lines: Mutex<Vec<String>>,      // 用于收集多行日志（如 Traceback）
    in_multiline: Mutex<bool>,              // 是否正在处理多行日志
    output_queue: Mutex<Vec<ParsedLog>>,    // 输出队列
}

impl SraLogParser {
    pub fn new() -> Self {
        // 日志格式: 14:20:07[40401] | INFO | 当前配置: Default
        let log_pattern = Regex::new(
            r"^(\d{2}:\d{2}:\d{2})\[\d+\]\s*\|\s*(SUCCESS|DEBUG|INFO|WARNING|ERROR|TRACE)\s*\|\s*(.+)$"
        ).unwrap();

        Self {
            log_pattern,
            pending_lines: Mutex::new(Vec::new()),
            in_multiline: Mutex::new(false),
            output_queue: Mutex::new(Vec::new()),
        }
    }

    // 解析单行输出，返回所有待输出的日志
    pub fn parse_line(&self, line: &str) -> Vec<ParsedLog> {
        let mut results = Vec::new();
        
        // 先检查输出队列
        let mut queue = self.output_queue.lock().unwrap();
        if !queue.is_empty() {
            results.append(&mut *queue);
        }
        drop(queue);
        
        // 处理当前行
        if let Some(log) = self.parse_line_internal(line) {
            results.push(log);
        }
        
        // 再次检查输出队列（处理过程中可能产生新的输出）
        let mut queue = self.output_queue.lock().unwrap();
        if !queue.is_empty() {
            results.append(&mut *queue);
        }
        drop(queue);
        
        results
    }
    
    fn parse_line_internal(&self, line: &str) -> Option<ParsedLog> {
        let line = line.trim();
        
        // 空行跳过
        if line.is_empty() {
            return None;
        }

        let mut in_multiline = self.in_multiline.lock().unwrap();
        let mut pending = self.pending_lines.lock().unwrap();

        // 尝试匹配日志格式
        if let Some(captures) = self.log_pattern.captures(line) {
            // 如果之前有待处理的 MSG 内容，先输出它们，并将当前日志加入队列
            if !*in_multiline && !pending.is_empty() {
                let msg_content = pending.join("\n");
                pending.clear();
                
                // 解析当前日志并加入输出队列
                let time = captures.get(1).unwrap().as_str().to_string();
                let level_str = captures.get(2).unwrap().as_str();
                let message = captures.get(3).unwrap().as_str().trim().to_string();
                
                let level = match level_str {
                    "SUCCESS" => LogLevel::SUCCESS,
                    "DEBUG" => LogLevel::DEBUG,
                    "INFO" => LogLevel::INFO,
                    "WARNING" => LogLevel::WARN,
                    "ERROR" => LogLevel::ERR,
                    "TRACE" => LogLevel::TRACE,
                    _ => LogLevel::INFO,
                };
                
                let mut queue = self.output_queue.lock().unwrap();
                queue.push(ParsedLog {
                    level,
                    message,
                    time,
                });
                drop(queue);
                
                drop(in_multiline);
                drop(pending);
                
                // 返回 MSG 日志
                return Some(ParsedLog {
                    level: LogLevel::MSG,
                    message: msg_content,
                    time: chrono::Local::now().format("%H:%M:%S").to_string(),
                });
            }

            // 解析日志
            let time = captures.get(1).unwrap().as_str().to_string();
            let level_str = captures.get(2).unwrap().as_str();
            let message = captures.get(3).unwrap().as_str().trim().to_string();

            // 映射日志级别
            let level = match level_str {
                "SUCCESS" => LogLevel::SUCCESS,
                "DEBUG" => LogLevel::DEBUG,
                "INFO" => LogLevel::INFO,
                "WARNING" => LogLevel::WARN,
                "ERROR" => LogLevel::ERR,
                "TRACE" => LogLevel::TRACE,
                _ => LogLevel::INFO,
            };

            // 如果是 ERROR 级别，标记为多行模式（可能有 Traceback）
            if matches!(level, LogLevel::ERR) {
                *in_multiline = true;
                pending.push(message.clone());
                drop(in_multiline);
                drop(pending);
                
                // 暂时不返回，等待收集完整的 Traceback
                return None;
            }

            // 非 ERROR 日志直接返回
            drop(in_multiline);
            drop(pending);
            
            return Some(ParsedLog {
                level,
                message,
                time,
            });
        }

        // 不符合日志格式的行
        if *in_multiline {
            // 如果正在处理多行日志（Traceback），添加到待处理列表
            pending.push(line.to_string());
            
            // 检查是否是 Traceback 的最后一行（通常以错误类型结尾）
            // 例如: FileNotFoundError: [Errno 2] No such file or directory: 'config.ini'
            if line.contains("Error:") || line.contains("Exception:") {
                // Traceback 结束，返回完整的错误日志
                let full_message = pending.join("\n");
                pending.clear();
                *in_multiline = false;
                
                drop(in_multiline);
                drop(pending);
                
                return Some(ParsedLog {
                    level: LogLevel::ERR,
                    message: full_message,
                    time: chrono::Local::now().format("%H:%M:%S").to_string(),
                });
            }
            
            drop(in_multiline);
            drop(pending);
            None
        } else {
            // 不在多行模式，这是一个 MSG 行
            pending.push(line.to_string());
            drop(in_multiline);
            drop(pending);
            None
        }
    }

    // 刷新待处理的行（在进程结束或需要强制输出时调用）
    pub fn flush(&self) -> Vec<ParsedLog> {
        let mut results = Vec::new();
        
        // 输出队列中的日志
        let mut queue = self.output_queue.lock().unwrap();
        if !queue.is_empty() {
            results.append(&mut *queue);
        }
        drop(queue);
        
        // 待处理的行
        let mut pending = self.pending_lines.lock().unwrap();
        let mut in_multiline = self.in_multiline.lock().unwrap();
        
        if !pending.is_empty() {
            let msg_content = pending.join("\n");
            pending.clear();
            *in_multiline = false;
            
            results.push(ParsedLog {
                level: LogLevel::MSG,
                message: msg_content,
                time: chrono::Local::now().format("%H:%M:%S").to_string(),
            });
        }
        
        results
    }
}

// 全局解析器实例
static SRA_PARSER: Mutex<Option<SraLogParser>> = Mutex::new(None);

// 初始化解析器
pub fn init_parser() {
    let mut guard = SRA_PARSER.lock().unwrap();
    *guard = Some(SraLogParser::new());
}

// 解析行
pub fn parse_line(line: &str) -> Vec<ParsedLog> {
    let guard = SRA_PARSER.lock().unwrap();
    if let Some(ref parser) = *guard {
        parser.parse_line(line)
    } else {
        Vec::new()
    }
}

// 刷新待处理的行
pub fn flush_parser() -> Vec<ParsedLog> {
    let guard = SRA_PARSER.lock().unwrap();
    if let Some(ref parser) = *guard {
        parser.flush()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_normal_log() {
        let parser = SraLogParser::new();
        let line = "14:20:07[40401] | INFO | 当前配置: Default";
        
        let result = parser.parse_line(line);
        assert!(!result.is_empty());
        
        let log = &result[0];
        assert_eq!(log.time, "14:20:07");
        assert!(matches!(log.level, LogLevel::INFO));
        assert_eq!(log.message, "当前配置: Default");
    }

    #[test]
    fn test_parse_msg() {
        let parser = SraLogParser::new();
        let line = "可用命令:";
        
        let result = parser.parse_line(line);
        // MSG 会被收集，需要多次调用或 flush
        assert!(result.is_empty());
        
        let flushed = parser.flush();
        assert!(!flushed.is_empty());
        
        let log = &flushed[0];
        assert!(matches!(log.level, LogLevel::MSG));
    }
}
