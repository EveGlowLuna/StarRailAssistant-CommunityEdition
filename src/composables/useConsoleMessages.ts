// 全局控制台消息管理器
// 确保在任何页面都能接收和存储消息

import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// 消息级别
export type MessageLevel = "INFO" | "WARN" | "ERR" | "TRACE" | "DEBUG" | "SUCCESS" | "MSG"
export type MessageSource = "前端" | "后端" | "进程端"

// 消息接口（与后端保持一致）
export interface ConsoleMessage {
    source: MessageSource
    level: MessageLevel
    message: string
    time: string // ISO 8601 格式
}

class ConsoleMessageManager {
    private messages = ref<ConsoleMessage[]>([])
    private unlistenLogMessage: (() => void) | null = null
    private initialized = false

    // 获取消息列表
    getMessages() {
        return this.messages
    }

    // 添加消息（发送到后端）
    async addMessage(source: MessageSource, level: MessageLevel, message: string) {
        try {
            await invoke('log_from_frontend', {
                source,
                level,
                message
            })
        } catch (error) {
            console.error('发送日志到后端失败:', error)
        }
    }

    // 清空消息（仅清空前端显示，不影响后端日志文件）
    clearMessages() {
        this.messages.value = []
        this.addMessage("前端", "INFO", "控制台已清空")
    }

    // 加载所有日志
    async loadAllLogs() {
        try {
            const logs = await invoke<ConsoleMessage[]>('get_all_logs')
            this.messages.value = logs
        } catch (error) {
            console.error('加载日志失败:', error)
        }
    }

    // 初始化全局监听器
    async initialize() {
        if (this.initialized) return
        
        // 加载所有历史日志
        await this.loadAllLogs()
        
        // 监听新的日志消息
        try {
            this.unlistenLogMessage = await listen<ConsoleMessage>('log-message', (event) => {
                this.messages.value.push(event.payload)
            })
            console.log("日志监听器已启动")
        } catch (error) {
            console.error("启动日志监听器失败:", error)
        }
        
        // 设置前端错误捕获
        this.setupFrontendErrorCapture()
        
        // 设置控制台捕获
        this.setupConsoleCapture()
        
        this.initialized = true
        
        // 发送启动消息
        this.addMessage("前端", "INFO", "控制台已启动")
    }

    // 前端错误捕获
    private setupFrontendErrorCapture() {
        // 捕获未处理的Promise拒绝
        window.addEventListener('unhandledrejection', (event) => {
            const error = event.reason
            const errorMessage = error?.message || String(error)
            this.addMessage("前端", "ERR", `未处理的Promise: ${errorMessage}`)
        })

        // 捕获全局错误
        window.addEventListener('error', (event) => {
            this.addMessage("前端", "ERR", `${event.message} (${event.filename}:${event.lineno})`)
        })
    }

    // 控制台捕获
    private setupConsoleCapture() {
        const originalConsoleError = console.error
        console.error = (...args: any[]) => {
            originalConsoleError.apply(console, args)
            const message = args.map(arg =>
                typeof arg === 'object' ? JSON.stringify(arg) : String(arg)
            ).join(' ')
            this.addMessage("前端", "ERR", `控制台错误: ${message}`)
        }

        const originalConsoleWarn = console.warn
        console.warn = (...args: any[]) => {
            originalConsoleWarn.apply(console, args)
            const message = args.map(arg =>
                typeof arg === 'object' ? JSON.stringify(arg) : String(arg)
            ).join(' ')
            this.addMessage("前端", "WARN", `控制台警告: ${message}`)
        }

        const originalConsoleInfo = console.info
        console.info = (...args: any[]) => {
            originalConsoleInfo.apply(console, args)
            const message = args.map(arg =>
                typeof arg === 'object' ? JSON.stringify(arg) : String(arg)
            ).join(' ')
            this.addMessage("前端", "INFO", `控制台信息: ${message}`)
        }
    }

    // 清理监听器（应用关闭时调用）
    cleanup() {
        if (this.unlistenLogMessage) {
            this.unlistenLogMessage()
            this.unlistenLogMessage = null
        }
        this.initialized = false
    }
}

// 创建全局单例
const consoleMessageManager = new ConsoleMessageManager()

// 导出 composable
export const useConsoleMessages = () => {
    return {
        messages: consoleMessageManager.getMessages(),
        addMessage: consoleMessageManager.addMessage.bind(consoleMessageManager),
        clearMessages: consoleMessageManager.clearMessages.bind(consoleMessageManager),
        loadAllLogs: consoleMessageManager.loadAllLogs.bind(consoleMessageManager),
        initialize: consoleMessageManager.initialize.bind(consoleMessageManager)
    }
}

// 在应用启动时初始化
export const initializeConsoleMessages = async () => {
    await consoleMessageManager.initialize()
}

// 全局日志函数，供所有组件调用
declare global {
    interface Window {
        logToConsole: (source: MessageSource, level: MessageLevel, message: string) => Promise<void>
    }
}

// 在应用启动时挂载全局函数
export const setupGlobalConsoleLogger = () => {
    const { addMessage } = useConsoleMessages()
    window.logToConsole = addMessage
}
