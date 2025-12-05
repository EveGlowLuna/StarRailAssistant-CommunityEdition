<template>
    <div class="console-container">
        <!-- 控制台头部 -->
        <div class="console-header">
            <div class="panel-header">
                <h2 class="panel-title">{{ t('console.title') }}</h2>
                <div class="header-actions">
                    <button class="export-button" @click="exportLogs">
                        {{ t('console.exportLogs') }}
                    </button>
                    <button class="clear-button" @click="clearMessages">
                        {{ t('console.clearConsole') }}
                    </button>
                </div>
            </div>
        </div>

        <!-- 过滤控制区域 -->
        <div class="filter-panel">
            <div class="filter-controls">
                <!-- 等级过滤 -->
                <div class="filter-group">
                    <span class="filter-label">{{ t('console.filterLevel') }}:</span>
                    <div class="filter-buttons">
                        <button
                            v-for="level in messageLevels"
                            :key="level"
                            class="filter-button"
                            :class="{
                                active: levelFilters[level],
                                [level.toLowerCase()]: true,
                            }"
                            @click="toggleLevelFilter(level)"
                        >
                            {{ level }}
                        </button>
                    </div>
                </div>
                <!-- 端口过滤 -->
                <div class="filter-group">
                    <span class="filter-label">{{ t('console.filterSource') }}:</span>
                    <div class="filter-buttons">
                        <button
                            v-for="source in messageSources"
                            :key="source"
                            class="filter-button"
                            :class="{
                                active: sourceFilters[source],
                                [source.toLowerCase()]: true,
                            }"
                            @click="toggleSourceFilter(source)"
                        >
                            {{ getSourceLabel(source) }}
                        </button>
                    </div>
                </div>
            </div>
        </div>

        <!-- 消息输出区域 -->
        <div class="output-panel">
            <div class="messages-container" ref="messagesContainer">
                <div
                    v-for="(message, index) in filteredMessages"
                    :key="index"
                    class="message-rectangle"
                    :class="(message as ConsoleMessage).level"
                >
                    <div class="message-header">
                        <span class="message-source">{{ getSourceLabel((message as ConsoleMessage).source) }}</span>
                        <span class="message-timestamp">[{{ formatTime((message as ConsoleMessage).time) }}]</span>
                        <span class="message-level">{{ (message as ConsoleMessage).level }}</span>
                    </div>
                    <div class="message-body">
                        {{ getMessageText(message as ConsoleMessage) }}
                    </div>
                </div>
            </div>
        </div>

        <!-- 消息输入区域 -->
        <div class="input-panel">
            <div class="input-container">
                <input
                    v-model="inputMessage"
                    @keyup.enter="sendMessage"
                    placeholder="输入消息..."
                    class="message-input"
                />
                <button
                    class="send-button"
                    @click="sendMessage"
                    :disabled="!inputMessage.trim()"
                >
                    {{ t('console.send') }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick, onMounted, onActivated, watch, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';
import { useConsoleMessages, type MessageLevel, type MessageSource, type ConsoleMessage } from "../composables/useConsoleMessages";
import { useTranslation } from "../composables/useTranslation";

const { t } = useTranslation();

// 使用全局消息管理器
const { messages, addMessage, clearMessages: clearGlobalMessages, loadAllLogs } = useConsoleMessages();

// 响应式数据
const inputMessage = ref("");
const messagesContainer = ref<HTMLElement>();
const isUserScrolling = ref(false);

// 从 localStorage 加载过滤器状态
const loadFilters = () => {
    try {
        const savedLevelFilters = localStorage.getItem('console-level-filters');
        const savedSourceFilters = localStorage.getItem('console-source-filters');
        
        return {
            level: savedLevelFilters ? JSON.parse(savedLevelFilters) : {
                INFO: true,
                WARN: true,
                ERR: true,
                TRACE: true,
                DEBUG: true,
                SUCCESS: true,
                MSG: true,
            },
            source: savedSourceFilters ? JSON.parse(savedSourceFilters) : {
                前端: true,
                后端: true,
                进程端: true,
            }
        };
    } catch {
        return {
            level: {
                INFO: true,
                WARN: true,
                ERR: true,
                TRACE: true,
                DEBUG: true,
                SUCCESS: true,
                MSG: true,
            },
            source: {
                前端: true,
                后端: true,
                进程端: true,
            }
        };
    }
};

const savedFilters = loadFilters();

// 过滤状态
const levelFilters = ref<Record<MessageLevel, boolean>>(savedFilters.level);
const sourceFilters = ref<Record<MessageSource, boolean>>(savedFilters.source);

// 常量
const messageLevels: MessageLevel[] = ["INFO", "WARN", "ERR", "TRACE", "DEBUG", "SUCCESS", "MSG"];
const messageSources: MessageSource[] = ["前端", "后端", "进程端"];

// 获取源标签的翻译
const getSourceLabel = (source: MessageSource): string => {
    const sourceMap: Record<MessageSource, string> = {
        '前端': t('console.sources.frontend').value,
        '后端': t('console.sources.backend').value,
        '进程端': t('console.sources.process').value
    };
    return sourceMap[source] || source;
};

// 计算属性：过滤后的消息
const filteredMessages = computed(() => {
    return messages.value.filter((message: ConsoleMessage) => {
        return (
            levelFilters.value[message.level] &&
            sourceFilters.value[message.source]
        );
    });
});

// 导出日志
const exportLogs = async () => {
    if (messages.value.length === 0) {
        window.showNotification?.("没有日志可导出", 3000);
        return;
    }

    try {
        const defaultFileName = `sra-logs-${new Date().toISOString().slice(0, 19).replace(/:/g, "-")}.txt`;
        
        const filePath = await save({
            defaultPath: defaultFileName,
            filters: [
                { name: '文本文件', extensions: ['txt'] },
                { name: '所有文件', extensions: ['*'] }
            ],
            title: '导出日志'
        });

        if (!filePath) {
            return;
        }

        const logContent = messages.value
            .map((message: ConsoleMessage) => {
                return `${formatTime(message.time)} [${message.source}] [${message.level}] ${message.message}`;
            })
            .join("\n");

        await writeTextFile(filePath, logContent);
        window.showNotification?.("日志导出成功", 3000);
    } catch (error) {
        console.error("Failed to export logs:", error);
        window.showNotification?.("日志导出失败", 3000);
    }
};

// 方法
const sendMessage = async () => {
    if (inputMessage.value.trim()) {
        const message = inputMessage.value.trim();
        
        // 发送到后端进程
        try {
            await invoke("send_input_to_sra", { input: message });
            // 发送成功，后端会自动记录日志
        } catch (error: any) {
            // 发送失败，显示错误
            console.error("Failed to send message to process:", error);
            const errorMsg = typeof error === 'string' ? error : (error.message || t('console.sendFailed').value);
            await addMessage("前端", "ERR", `${t('console.sendMessageFailed').value}: ${errorMsg}`);
        }
        
        inputMessage.value = "";
        scrollToBottom();
    }
};

const clearMessages = () => {
    clearGlobalMessages();
};

const toggleLevelFilter = (level: MessageLevel) => {
    levelFilters.value[level] = !levelFilters.value[level];
    // 保存到 localStorage
    localStorage.setItem('console-level-filters', JSON.stringify(levelFilters.value));
};

const toggleSourceFilter = (source: MessageSource) => {
    sourceFilters.value[source] = !sourceFilters.value[source];
    // 保存到 localStorage
    localStorage.setItem('console-source-filters', JSON.stringify(sourceFilters.value));
};

// 检查是否在底部（允许10px的误差）
const isAtBottom = () => {
    if (!messagesContainer.value) return false;
    const { scrollTop, scrollHeight, clientHeight } = messagesContainer.value;
    return scrollHeight - scrollTop - clientHeight < 10;
};

// 滚动到底部
const scrollToBottom = async () => {
    await nextTick();
    if (messagesContainer.value) {
        messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
    }
};

// 处理滚动事件
const handleScroll = () => {
    if (!messagesContainer.value) return;
    
    // 如果用户滚动到底部，标记为非手动滚动状态
    if (isAtBottom()) {
        isUserScrolling.value = false;
    } else {
        // 用户向上滚动，标记为手动滚动状态
        isUserScrolling.value = true;
    }
};

// 获取消息显示文本
const getMessageText = (message: ConsoleMessage) => {
    return message.message || '[空消息]';
};

// 格式化时间显示
const formatTime = (isoTime: string) => {
    try {
        const date = new Date(isoTime);
        return date.toLocaleTimeString('zh-CN', { hour12: false });
    } catch {
        return isoTime;
    }
};

// 监听消息变化，智能滚动和检测任务开始
watch(() => messages.value.length, async (newLength, oldLength) => {
    // 只有在用户没有手动滚动时才自动滚动到底部
    if (!isUserScrolling.value) {
        await scrollToBottom();
    }
    
    // 检测新消息中是否有 [Start] 标记
    if (newLength > oldLength) {
        const newMessage = messages.value[newLength - 1];
        // 检查是否是进程端的消息且包含 [Start]
        if (newMessage.source === '进程端' && newMessage.message.includes('[Start]')) {
            window.showNotification?.(t('console.taskStarted').value, 3000);
        }
    }
});

// 生命周期
onMounted(async () => {
    // 加载所有历史日志
    await loadAllLogs();
    scrollToBottom();
    
    // 添加滚动事件监听
    if (messagesContainer.value) {
        messagesContainer.value.addEventListener('scroll', handleScroll);
    }
});

// 当页面被 keep-alive 激活时
onActivated(() => {
    // 重置用户滚动状态
    isUserScrolling.value = false;
    
    // 使用 setTimeout 确保在下一个事件循环中执行
    setTimeout(() => {
        if (messagesContainer.value) {
            messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
        }
    }, 0);
    
    // 再次确保滚动（有时需要多次尝试）
    setTimeout(() => {
        if (messagesContainer.value) {
            messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
        }
    }, 50);
});

onBeforeUnmount(() => {
    // 移除滚动事件监听
    if (messagesContainer.value) {
        messagesContainer.value.removeEventListener('scroll', handleScroll);
    }
});


</script>

<style scoped>
.console-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 16px;
  box-sizing: border-box;
  gap: 16px;
}

.console-header,
.filter-panel,
.output-panel,
.input-panel {
    background: rgba(255, 255, 255, 0.8);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
}

.console-header {
    min-height: 60px;
    padding: 0;
}

.filter-panel {
    min-height: auto;
    padding: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.output-panel {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
}

.input-panel {
    min-height: 60px;
    flex-shrink: 0;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}

.panel-title {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: #000;
  user-select: none;
}

.filter-controls {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: flex-start;
  gap: 16px;
  padding: 8px 16px;
  margin: 0;
}

.filter-group {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
}


.filter-label {
  font-size: 12px;
  font-weight: 500;
  color: #000;
  user-select: none;
}

.filter-buttons {
    display: flex;
    gap: 4px;
}

.filter-button {
  padding: 4px 10px;
  border: 1px solid rgba(0, 0, 0, 0.2);
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  background: rgba(255, 255, 255, 0.3);
  color: #000;
  cursor: pointer;
  transition: all 0.3s ease;
  user-select: none;
}

.filter-button:hover {
    transform: translateY(-1px);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.filter-button.active {
    color: white;
    border-color: transparent;
}

.filter-button.info.active {
    background: rgba(0, 0, 0, 0.7);
}

.filter-button.warn.active {
    background: rgba(255, 193, 7, 0.9);
}

.filter-button.err.active {
    background: rgba(244, 67, 54, 0.9);
}

.filter-button.trace.active {
    background: rgba(156, 39, 176, 0.9);
}

.filter-button.debug.active {
    background: rgba(33, 150, 243, 0.9);
}

.filter-button.success.active {
    background: rgba(76, 175, 80, 0.9);
}

.filter-button.msg.active {
    background: rgba(158, 158, 158, 0.9);
}

.filter-button.前端.active {
    background: rgba(76, 175, 80, 0.9);
}

.filter-button.后端.active {
    background: rgba(255, 152, 0, 0.9);
}

.filter-button.进程端.active {
    background: rgba(103, 58, 183, 0.9);
}

.export-button {
  padding: 6px 12px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  background: rgba(33, 150, 243, 0.8);
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  user-select: none;
}

.export-button:hover {
    background: rgba(33, 150, 243, 1);
}

.clear-button {
  padding: 6px 12px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  background: rgba(244, 67, 54, 0.8);
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  user-select: none;
}

.clear-button:hover {
    background: rgba(244, 67, 54, 1);
}

.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 6px;
  user-select: text;
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
  align-items: stretch;
}

/* 美化滚动条 */
.messages-container::-webkit-scrollbar {
    width: 8px;
}

.messages-container::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.05);
    border-radius: 4px;
}

.messages-container::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 4px;
    transition: background-color 0.3s ease;
}

.messages-container::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.3);
}

/* Firefox 滚动条样式 */
.messages-container {
    scrollbar-width: thin;
    scrollbar-color: rgba(0, 0, 0, 0.2) rgba(0, 0, 0, 0.05);
}

.message-rectangle {
  padding: 8px 12px;
  border-radius: 6px;
  width: 100%;
  max-width: 100%;
  word-wrap: break-word;
  font-family: "Consolas", "Monaco", monospace;
  font-size: 13px;
  line-height: 1.4;
  align-self: stretch;
  user-select: text;
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
  cursor: text;
  display: flex;
  flex-direction: column;
  gap: 3px;
  box-sizing: border-box;
}

.message-rectangle.INFO {
    background: rgba(255, 255, 255, 0.9);
    color: #000;
    border: 1px solid rgba(0, 0, 0, 0.1);
}

.message-rectangle.WARN {
    background: rgba(255, 193, 7, 0.9);
    color: #000;
    border: 1px solid rgba(255, 193, 7, 0.5);
}

.message-rectangle.ERR {
    background: rgba(244, 67, 54, 0.9);
    color: white;
    border: 1px solid rgba(244, 67, 54, 0.5);
}

.message-rectangle.TRACE {
    background: rgba(156, 39, 176, 0.9);
    color: white;
    border: 1px solid rgba(156, 39, 176, 0.5);
}

.message-rectangle.DEBUG {
    background: rgba(33, 150, 243, 0.9);
    color: white;
    border: 1px solid rgba(33, 150, 243, 0.5);
}

.message-rectangle.SUCCESS {
    background: rgba(76, 175, 80, 0.9);
    color: white;
    border: 1px solid rgba(76, 175, 80, 0.5);
}

.message-rectangle.MSG {
    background: rgba(158, 158, 158, 0.9);
    color: white;
    border: 1px solid rgba(158, 158, 158, 0.5);
}


.message-header {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  margin-bottom: 2px;
}

.message-source {
  font-weight: bold;
  color: rgba(0, 0, 0, 0.7);
}

.message-timestamp {
  color: rgba(0, 0, 0, 0.7);
  font-family: "Consolas", "Monaco", monospace;
}

.message-level {
  font-weight: bold;
  padding: 2px 6px;
  border-radius: 4px;
  background: rgba(0, 0, 0, 0.15);
  color: rgba(0, 0, 0, 0.8);
  font-size: 11px;
}

/* 深色模式下的消息头部样式 */
@media (prefers-color-scheme: dark) {
  .message-source,
  .message-timestamp {
    color: rgba(255, 255, 255, 0.8);
  }
  
  .message-level {
    background: rgba(255, 255, 255, 0.15);
    color: rgba(255, 255, 255, 0.9);
  }
}

.message-body {
  font-size: 13px;
  color: inherit;
  word-break: break-all;
  line-height: 1.3;
  margin-top: 0;
  white-space: pre-wrap;
  overflow-wrap: break-word;
}

/* 深色模式下的消息级别样式 */
@media (prefers-color-scheme: dark) {
  .message-header {
    color: #aaa;
  }
  
  .message-level {
    background: rgba(255, 255, 255, 0.1);
  }
}

.input-container {
  display: flex;
  gap: 8px;
  padding: 12px 16px;
  align-items: center;
}

.message-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid rgba(0, 0, 0, 0.2);
  border-radius: 4px;
  font-size: 14px;
  background: rgba(255, 255, 255, 0.9);
  color: #000;
  outline: none;
  transition: all 0.3s ease;
  user-select: text;
}

.message-input:focus {
    border-color: #007bff;
    background: rgba(255, 255, 255, 0.8);
}

.send-button {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  background: rgba(76, 175, 80, 0.8);
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 60px;
  user-select: none;
}

.send-button:hover:not(:disabled) {
    background: rgba(76, 175, 80, 1);
    transform: translateY(-1px);
}

.send-button:disabled {
    background: rgba(0, 0, 0, 0.2);
    color: rgba(255, 255, 255, 0.5);
    cursor: not-allowed;
    transform: none;
}

/* 深色模式样式 */
@media (prefers-color-scheme: dark) {
    .console-header,
    .filter-panel,
    .output-panel,
    .input-panel {
        background: rgba(0, 0, 0, 0.8);
        border: 1px solid rgba(255, 255, 255, 0.2);
    }

    .panel-header {
        border-bottom: 1px solid rgba(255, 255, 255, 0.2);
    }

    .panel-title {
        color: #fff;
    }

    .filter-label {
      color: #fff;
    }

    .filter-button {
      background: rgba(255, 255, 255, 0.1);
      border: 1px solid rgba(255, 255, 255, 0.2);
      color: #fff;
    }

    .export-button {
        background: rgba(33, 150, 243, 0.6);
    }

    .export-button:hover {
        background: rgba(33, 150, 243, 1);
    }

    .clear-button {
        background: rgba(244, 67, 54, 0.6);
    }

    .clear-button:hover {
        background: rgba(244, 67, 54, 1);
    }

    .message-rectangle.INFO {
        background: rgba(255, 255, 255, 0.1);
        color: #fff;
        border: 1px solid rgba(255, 255, 255, 0.2);
    }

    .message-rectangle.WARN {
        background: rgba(255, 193, 7, 0.8);
        color: #000;
    }

    .message-input {
      background: rgba(255, 255, 255, 0.1);
      border: 1px solid rgba(255, 255, 255, 0.2);
      color: #fff;
    }

    .message-input:focus {
        background: rgba(0, 0, 0, 0.8);
    }

    .clear-button {
        background: rgba(244, 67, 54, 0.8);
    }

    .clear-button:hover {
        background: rgba(244, 67, 54, 1);
    }

    /* 深色模式滚动条 */
    .messages-container::-webkit-scrollbar-track {
        background: rgba(255, 255, 255, 0.05);
    }

    .messages-container::-webkit-scrollbar-thumb {
        background: rgba(255, 255, 255, 0.2);
    }

    .messages-container::-webkit-scrollbar-thumb:hover {
        background: rgba(255, 255, 255, 0.3);
    }

    .messages-container {
        scrollbar-color: rgba(255, 255, 255, 0.2) rgba(255, 255, 255, 0.05);
    }
}
</style>
