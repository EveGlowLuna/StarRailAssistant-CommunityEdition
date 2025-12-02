// 类型定义和常量

use std::process::Child;

// SRA 进程状态
#[derive(Debug, Clone, PartialEq)]
pub enum SraStatus {
    NotRunning,      // 未运行
    Running,         // 运行中
    #[allow(dead_code)]
    TaskRunning,     // 任务执行中（保留用于未来功能）
    Error,           // 异常
}

pub struct SraProcess {
    pub child: Option<Child>,
    pub status: SraStatus,
}

// 常量定义
// TODO: 日志系统 - 待重新实现
