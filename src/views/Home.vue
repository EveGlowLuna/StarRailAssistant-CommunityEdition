<template>
    <div class="home-container">
        <!-- 上方面板：状态和控制 -->
        <div class="top-panel">
            <div class="panel-content">
                <div class="status-control-container">
                    <!-- 左侧：核心状态 -->
                    <div class="status-section">
                        <div class="core-status">
                            <div class="status-indicator">
                                <div
                                    class="status-circle"
                                    :class="coreStatusClass"
                                ></div>
                                <span class="status-text">{{
                                    coreStatusText
                                }}</span>
                            </div>
                        </div>
                    </div>

                    <!-- 右侧：控制按钮 -->
                    <div class="control-section">
                        <div class="control-buttons">
                            <button
                                class="control-button main-button"
                                :class="{
                                    start: !isExecutingTask,
                                    stop: isExecutingTask,
                                }"
                                :disabled="coreStatus === 'error' || coreStatus === 'not-running' || !hasSelectedConfigs"
                                @click="toggleTaskExecution"
                            >
                                {{ isExecutingTask ? t('home.stopTask') : t('home.startTask') }}
                            </button>
                            <button
                                class="control-button restart-button"
                                @click="restartCore"
                            >
                                {{ coreStatus === 'not-running' ? t('home.startCore') : t('home.restartCore') }}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 下方面板：合并的功能和配置区 -->
        <div class="bottom-panel">
            <div class="panel-content">
                <div class="bottom-content-container">
                    <!-- 左侧：功能展示区 -->
                    <div class="left-section">
                        <div class="section-header">
                            <h3 class="section-title">{{ t('home.quickAccess') }}</h3>
                        </div>
                        <div class="feature-buttons">
                            <button class="feature-button version-button" @click="showVersionUpdate">
                                <div class="feature-icon">
                                    <RefreshCw :size="24" />
                                </div>
                                <div class="feature-text">
                                    <div class="feature-title">{{ t('home.versionUpdate') }}</div>
                                    <div class="feature-desc">{{ t('home.versionUpdateDesc') }}</div>
                                </div>
                            </button>
                            
                            <button class="feature-button announcement-button" @click="showAnnouncement">
                                <div class="feature-icon">
                                    <Megaphone :size="24" />
                                </div>
                                <div class="feature-text">
                                    <div class="feature-title">{{ t('home.announcement') }}</div>
                                    <div class="feature-desc">{{ t('home.announcementDesc') }}</div>
                                </div>
                            </button>
                            
                            <button class="feature-button tutorial-button" @click="showTutorial">
                                <div class="feature-icon">
                                    <BookOpen :size="24" />
                                </div>
                                <div class="feature-text">
                                    <div class="feature-title">{{ t('home.tutorial') }}</div>
                                    <div class="feature-desc">{{ t('home.tutorialDesc') }}</div>
                                </div>
                            </button>
                        </div>
                    </div>

                    <!-- 垂直分割线 -->
                    <div class="vertical-divider"></div>

                    <!-- 右侧：配置管理区 -->
                    <div class="right-section">
                        <div class="section-header">
                            <h3 class="section-title">{{ t('home.configManagement') }}</h3>
                        </div>
                        
                        <!-- 操作按钮 -->
                        <div class="action-buttons">
                            <button
                                class="action-button refresh-button"
                                @click="refreshConfigs"
                            >
                                {{ t('home.refresh') }}
                            </button>
                            <button
                                class="action-button new-button"
                                @click="showCreateDialog = true"
                            >
                                {{ t('home.newConfig') }}
                            </button>
                            <button
                                class="action-button select-all-button"
                                @click="selectAllConfigs"
                            >
                                {{ t('home.selectAll') }}
                            </button>
                            <button
                                class="action-button invert-selection-button"
                                @click="deselectAllConfigs"
                            >
                                {{ t('home.invertSelection') }}
                            </button>
                            <button
                                class="action-button delete-button"
                                :disabled="!hasSelectedConfigs"
                                @click="deleteSelectedConfigs"
                            >
                                {{ t('home.deleteSelected') }}
                            </button>
                        </div>

                        <!-- 配置列表 -->
                        <div class="config-list" v-if="configs.length > 0">
                            <div
                                v-for="config in orderedConfigs"
                                :key="config.name"
                                class="config-item"
                                @mouseenter="hoveredConfig = config.name"
                                @mouseleave="hoveredConfig = null"
                            >
                                <div class="config-order">{{ config.order }}</div>
                                <input
                                    type="checkbox"
                                    :id="config.name"
                                    v-model="selectedConfigs"
                                    :value="config.name"
                                    class="config-checkbox"
                                    @change="saveTaskOrder"
                                />
                                <label :for="config.name" class="config-label">
                                    {{ config.name }}
                                </label>
                                <button
                                    v-if="hoveredConfig === config.name"
                                    class="delete-config-button"
                                    @click="deleteConfig(config.name)"
                                    title="删除配置"
                                >
                                    <Trash :size="14" :color="'white'" />
                                </button>
                            </div>
                        </div>

                        <!-- 空状态 -->
                        <div v-else class="empty-state">
                            <span class="empty-text">没有可用的配置，请添加配置。</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 新建配置对话框 -->
        <Transition name="modal">
            <div v-if="showCreateDialog" class="custom-modal-overlay" @click.self="showCreateDialog = false">
                <div class="custom-modal">
                    <div class="modal-header">
                        <h3 class="modal-title">{{ t('home.newConfig') }}</h3>
                    </div>
                    <div class="modal-content">
                        <div class="input-group">
                            <label for="config-name" class="input-label">{{ t('home.configName') }}:</label>
                            <input
                                id="config-name"
                                v-model="newConfigName"
                                type="text"
                                class="config-input"
                                placeholder=""
                                @keyup.enter="confirmCreateConfig"
                            />
                        </div>
                    </div>
                    <div class="modal-actions">
                        <button class="modal-button cancel-button" @click="cancelCreateConfig">{{ t('home.cancel') }}</button>
                        <button class="modal-button confirm-button" @click="confirmCreateConfig" :disabled="!newConfigName.trim()">{{ t('home.create') }}</button>
                    </div>
                </div>
            </div>
        </Transition>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Trash, RefreshCw, Megaphone, BookOpen } from "lucide-vue-next";
import { useTranslation } from "../composables/useTranslation";

const { t } = useTranslation();

interface Config {
    name: string;
    order?: number;
}

// 响应式数据
const configs = ref<Config[]>([]);
const selectedConfigs = ref<string[]>([]);
const hoveredConfig = ref<string | null>(null);
const showCreateDialog = ref(false);
const newConfigName = ref("");
const taskOrder = ref<string[]>([]);

// 核心状态相关
const coreStatus = ref<"running" | "task-running" | "not-running" | "error">("not-running");
const isExecutingTask = ref(false); // 是否正在执行任务
let statusCheckInterval: number | null = null;
let unlistenStatusChange: (() => void) | null = null;

const coreStatusClass = computed(() => {
    switch (coreStatus.value) {
        case "running":
            return "status-green";
        case "task-running":
            return "status-green";
        case "not-running":
            return "status-yellow";
        case "error":
            return "status-red";
        default:
            return "status-yellow";
    }
});

const coreStatusText = computed(() => {
    switch (coreStatus.value) {
        case "running":
            return isExecutingTask.value ? t('home.taskRunning').value : t('home.running').value;
        case "task-running":
            return t('home.taskRunning').value;
        case "not-running":
            return t('home.notRunning').value;
        case "error":
            return t('home.error').value;
        default:
            return t('home.notRunning').value;
    }
});

const hasSelectedConfigs = computed(() => {
    return selectedConfigs.value.length > 0;
});

const orderedConfigs = computed(() => {
    // 根据选中的配置和顺序进行排序
    const selectedSet = new Set(selectedConfigs.value);
    const selectedWithOrder = configs.value
        .filter(config => selectedSet.has(config.name))
        .map(config => ({
            ...config,
            order: selectedConfigs.value.indexOf(config.name) + 1
        }));
    
    const unselected = configs.value
        .filter(config => !selectedSet.has(config.name))
        .map(config => ({
            ...config,
            order: 0
        }));
    
    // 对未选中的配置按照 Default 第一，其他按首字母排序
    unselected.sort((a, b) => {
        if (a.name === 'Default') return -1;
        if (b.name === 'Default') return 1;
        return a.name.localeCompare(b.name, undefined, { sensitivity: 'base' });
    });
    
    // 合并：选中的在前（按选中顺序），未选中的在后（按字母顺序）
    return [...selectedWithOrder, ...unselected];
});

// 方法
const loadConfigs = async () => {
    try {
        const configNames = await invoke<string[]>("get_config_list");
        configs.value = configNames.map((name) => ({ name }));
        await loadTaskOrder();
    } catch (error) {
        console.error("Failed to load configs:", error);
    }
};

const loadTaskOrder = async () => {
    try {
        taskOrder.value = await invoke<string[]>("load_task_order");
        // 更新选中的配置
        selectedConfigs.value = taskOrder.value;
    } catch (error) {
        console.error("Failed to load task order:", error);
    }
};

const saveTaskOrder = async () => {
    try {
        await invoke("save_task_order", { taskOrder: selectedConfigs.value });
        taskOrder.value = [...selectedConfigs.value];
    } catch (error) {
        console.error("Failed to save task order:", error);
    }
};

const refreshConfigs = async () => {
    await loadConfigs();
};

const selectAllConfigs = () => {
    // 按照 Default 第一，其他按首字母排序
    const sortedConfigs = [...configs.value].sort((a, b) => {
        if (a.name === 'Default') return -1;
        if (b.name === 'Default') return 1;
        return a.name.localeCompare(b.name, undefined, { sensitivity: 'base' });
    });
    selectedConfigs.value = sortedConfigs.map(c => c.name);
    saveTaskOrder();
};

const deselectAllConfigs = () => {
    // 反选：选中未选的，取消已选的
    const allConfigNames = configs.value.map(c => c.name);
    const currentSelected = new Set(selectedConfigs.value);
    
    // 获取未选中的配置，按 Default 第一，其他按首字母排序
    const unselected = allConfigNames.filter(name => !currentSelected.has(name));
    unselected.sort((a, b) => {
        if (a === 'Default') return -1;
        if (b === 'Default') return 1;
        return a.localeCompare(b, undefined, { sensitivity: 'base' });
    });
    
    selectedConfigs.value = unselected;
    saveTaskOrder();
};

const confirmCreateConfig = async () => {
    if (!newConfigName.value.trim()) return;

    try {
        await invoke("create_new_config", { name: newConfigName.value.trim() });
        await loadConfigs();
        showCreateDialog.value = false;
        newConfigName.value = "";
    } catch (error) {
        console.error("Failed to create config:", error);
    }
};

const cancelCreateConfig = () => {
    showCreateDialog.value = false;
    newConfigName.value = "";
};

const deleteSelectedConfigs = async () => {
    if (selectedConfigs.value.length === 0) return;

    // 检查是否只选中了默认配置
    if (
        selectedConfigs.value.length === 1 &&
        selectedConfigs.value.includes("Default")
    ) {
        window.showNotification?.(t('home.notifications.cannotDeleteDefault').value, false, 3000);
        return;
    }

    // 过滤掉默认配置
    const filteredConfigs = selectedConfigs.value.filter(
        (name) => name !== "Default",
    );

    if (filteredConfigs.length === 0) {
        window.showNotification?.(t('home.notifications.cannotDeleteDefault').value, false, 3000);
        return;
    }

    try {
        for (const configName of filteredConfigs) {
            await invoke("delete_config", { name: configName });
        }
        // 从选中列表中移除已删除的配置
        selectedConfigs.value = selectedConfigs.value.filter(
            name => !filteredConfigs.includes(name)
        );
        await loadConfigs();
        await saveTaskOrder();
    } catch (error) {
        console.error("Failed to delete configs:", error);
    }
};

const deleteConfig = async (configName: string) => {
    if (configName === "Default") {
        window.showNotification?.(t('home.notifications.cannotDeleteDefault').value, false, 3000);
        return;
    }

    try {
        await invoke("delete_config", { name: configName });
        // 从选中列表中移除已删除的配置
        selectedConfigs.value = selectedConfigs.value.filter(name => name !== configName);
        await loadConfigs();
        await saveTaskOrder();
    } catch (error) {
        console.error("Failed to delete config:", error);
    }
};

const loadCoreStatus = async () => {
    try {
        const status = await invoke<string>("get_sra_status");
        coreStatus.value = status as any;
        // 如果状态是task-running，设置isExecutingTask为true
        if (status === "task-running") {
            isExecutingTask.value = true;
        }
    } catch (error) {
        console.error("Failed to get core status:", error);
    }
};

// 切换任务执行状态
const toggleTaskExecution = async () => {
    if (isExecutingTask.value) {
        // 停止任务
        try {
            await invoke("task_stop");
            isExecutingTask.value = false;
            window.showNotification?.(t('home.notifications.taskStopped').value, false, 3000);
        } catch (error) {
            console.error("Failed to stop task:", error);
            window.showNotification?.(t('home.notifications.stopTaskFailed').value, false, 3000);
        }
    } else {
        // 开始执行任务
        if (selectedConfigs.value.length === 0) {
            window.showNotification?.(t('home.notifications.selectConfigFirst').value, false, 3000);
            return;
        }
        
        // 验证配置
        const { validateConfigs } = await import('../utils/configValidator');
        const validationError = await validateConfigs(selectedConfigs.value);
        if (validationError) {
            window.showNotification?.(validationError, false, 5000);
            return;
        }
        
        try {
            const configName = selectedConfigs.value.length > 0 ? selectedConfigs.value.join(' ') : null;
            await invoke("task_run", { configName });
            isExecutingTask.value = true;
            window.showNotification?.(t('home.notifications.taskStarted', { names: selectedConfigs.value.join(', ') }).value, false, 3000);
        } catch (error: any) {
            console.error("Failed to execute task:", error);
            const errorMsg = typeof error === 'string' ? error : t('home.notifications.executeTaskFailed').value;
            window.showNotification?.(errorMsg, false, 5000);
        }
    }
};

const restartCore = async () => {
    try {
        if (coreStatus.value === 'not-running') {
            // 如果核心未运行，调用启动命令
            await invoke("start_sra_process_command", { arguments: null });
            window.showNotification?.(t('home.notifications.coreStarted').value, false, 3000);
        } else {
            // 否则调用重启命令
            await invoke("restart_sra_process_command", { arguments: null });
            window.showNotification?.(t('home.notifications.coreRestarted').value, false, 3000);
        }
    } catch (error: any) {
        console.error("Failed to restart/start core:", error);
        coreStatus.value = "error";
        const errorMsg = typeof error === 'string' ? error : t('home.notifications.coreRestartFailed').value;
        window.showNotification?.(errorMsg, false, 5000);
    }
};

// 功能按钮点击处理
const showVersionUpdate = async () => {
    const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow');
    
    new WebviewWindow('version-update', {
        url: '/version-update',
        title: t('home.versionUpdate').value,
        width: 800,
        height: 600,
        center: true,
        resizable: true,
        minWidth: 600,
        minHeight: 400
    });
};

const showAnnouncement = async () => {
    const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow');
    
    new WebviewWindow('announcement', {
        url: '/announcement-window',
        title: t('announcement.title').value,
        width: 800,
        height: 600,
        center: true,
        resizable: true,
        minWidth: 600,
        minHeight: 400
    });
};

const showTutorial = () => {
    console.log("显示教程文档");
    // TODO: 实现教程文档功能
};

// 生命周期
onMounted(async () => {
    loadConfigs();
    loadCoreStatus();
    
    // 监听SRA状态变化
    unlistenStatusChange = await listen<string>("sra-status-changed", (event) => {
        const newStatus = event.payload as any;
        coreStatus.value = newStatus;
        
        // 根据状态更新任务执行标志
        if (newStatus === "task-running") {
            isExecutingTask.value = true;
        } else if (newStatus === "running") {
            isExecutingTask.value = false;
        } else if (newStatus === "not-running") {
            isExecutingTask.value = false;
        }
    });
    
    // 定期检查核心状态
    statusCheckInterval = window.setInterval(() => {
        loadCoreStatus();
    }, 3000);
});

onUnmounted(() => {
    if (statusCheckInterval) {
        clearInterval(statusCheckInterval);
    }
    if (unlistenStatusChange) {
        unlistenStatusChange();
    }
});
</script>

<style scoped>
.home-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: 100%;
  padding: 16px;
  box-sizing: border-box;
}

.top-panel {
    background: rgba(255, 255, 255, 0.8);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    min-height: 60px;
    transform: translateZ(0);
    will-change: transform;
}

.bottom-panel {
    flex: 1;
    background: rgba(255, 255, 255, 0.8);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    min-height: 0;
    transform: translateZ(0);
    will-change: transform;
}

.panel-content {
  padding: 12px;
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.bottom-content-container {
    flex: 1;
    display: flex;
    gap: 0;
    min-height: 0;
}

.vertical-divider {
    width: 1px;
    background: rgba(0, 0, 0, 0.1);
    margin: 0 16px;
}

.left-section,
.right-section {
    display: flex;
    flex-direction: column;
    min-height: 0;
}

.left-section {
    flex: 2;
}

.right-section {
    flex: 3;
}

.section-header {
  margin-bottom: 12px;
}

.section-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #000;
}

/* 功能按钮样式 */
.feature-buttons {
  display: flex;
  flex-direction: column;
  gap: 12px;
  isolation: isolate;
}

.feature-button {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  border: none;
  border-radius: 8px;
  background: rgba(0, 0, 0, 0.05);
  cursor: pointer;
  transition: background-color 0.3s ease;
  text-align: left;
  transform: translate3d(0, 0, 0);
  backface-visibility: hidden;
  perspective: 1000px;
}

.feature-button:hover {
  background: rgba(0, 0, 0, 0.1);
}

.feature-icon {
  flex-shrink: 0;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  color: #333;
}

.feature-text {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.feature-title {
  font-size: 16px;
  font-weight: 600;
  color: #000;
}

.feature-desc {
  font-size: 12px;
  color: #666;
}



/* 上方面板样式 */
.status-control-container {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
}

.status-section {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    flex-shrink: 0;
}

.control-section {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    flex-shrink: 0;
}

.core-status {
    margin-bottom: 0;
    padding: 0;
}

.status-indicator {
    display: flex;
    align-items: center;
    gap: 10px;
}

.status-circle {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    margin-right: 8px;
}

.status-green {
    background-color: #4caf50;
}

.status-yellow {
    background-color: #ff9800;
}

.status-red {
    background-color: #f44336;
}

.status-text {
  font-size: 16px;
  font-weight: 500;
  color: #000;
}

.control-buttons {
    display: flex;
    gap: 6px;
    align-items: center;
}

.control-button {
    padding: 10px 20px;
    border: none;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
    background: rgba(0, 0, 0, 0.1);
    min-width: 80px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.control-button:hover:not(:disabled) {
    background: rgba(0, 0, 0, 0.2);
    transform: translateY(-2px);
}

.control-button:active:not(:disabled) {
    transform: translateY(0);
}

.control-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.main-button.start {
    background: rgba(76, 175, 80, 0.8);
    color: white;
}

.main-button.start:hover:not(:disabled) {
    background: rgba(76, 175, 80, 0.9);
}

.main-button.stop {
    background: rgba(244, 67, 54, 0.8);
    color: white;
}

.main-button.stop:hover:not(:disabled) {
    background: rgba(244, 67, 54, 0.9);
}

.restart-button {
    background: rgba(255, 152, 0, 0.8);
    color: white;
}

.restart-button:hover:not(:disabled) {
    background: rgba(255, 152, 0, 0.9);
}

/* 下方面板样式 */
.action-buttons {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.action-button {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
    background: rgba(0, 0, 0, 0.1);
}

.action-button:hover {
    background: rgba(0, 0, 0, 0.2);
    transform: translateY(-1px);
}

.action-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background: rgba(0, 0, 0, 0.1);
}

.delete-button:disabled {
    background: rgba(244, 67, 54, 0.3);
}

.delete-button:not(:disabled) {
    background: rgba(244, 67, 54, 0.8);
    color: white;
}

.delete-button:not(:disabled):hover {
    background: rgba(244, 67, 54, 1);
}

.config-list {
    flex: 1;
    overflow-y: auto;
}

.config-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  margin-bottom: 6px;
  border-radius: 4px;
  background: rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
  position: relative;
}

.config-item:hover {
    background: rgba(0, 0, 0, 0.1);
}

.config-checkbox {
    margin-right: 6px;
    width: 14px;
    height: 14px;
    accent-color: #007bff;
    cursor: pointer;
}

.config-label {
    flex: 1;
    cursor: pointer;
    font-size: 14px;
    color: #000;
}

.delete-config-button {
    width: 22px;
    height: 22px;
    border: none;
    border-radius: 50%;
    background: rgba(244, 67, 54, 0.8);
    color: white;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
    margin-left: 4px;
    padding: 0;
    min-width: 22px;
    min-height: 22px;
}

.delete-config-button:hover {
    background: rgba(244, 67, 54, 1);
    transform: scale(1.1);
}

.delete-config-button svg {
    color: white;
    width: 14px;
    height: 14px;
}

.empty-state {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100px;
}

.empty-text {
    font-size: 14px;
    color: #666;
    text-align: center;
}

/* 深色模式样式 */
@media (prefers-color-scheme: dark) {
    .top-panel,
    .bottom-panel {
        background: rgba(0, 0, 0, 0.8);
        border: 1px solid rgba(255, 255, 255, 0.2);
    }

    .vertical-divider {
        background: rgba(255, 255, 255, 0.1);
    }

    .status-text {
        color: #fff;
    }

    .task-text {
        color: #ccc;
    }

    .control-button {
        background: rgba(255, 255, 255, 0.1);
    }

    .control-button:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.2);
    }

    .action-button {
        background: rgba(255, 255, 255, 0.1);
    }

    .action-button:hover {
        background: rgba(255, 255, 255, 0.2);
    }

    .config-item {
        background: rgba(255, 255, 255, 0.1);
    }

    .config-item:hover {
        background: rgba(255, 255, 255, 0.2);
    }

    .config-label {
        color: #fff;
    }

    .empty-text {
        color: #ccc;
    }

    .section-title {
        color: #fff;
    }

    .feature-button {
        background: rgba(255, 255, 255, 0.1);
    }

    .feature-button:hover {
        background: rgba(255, 255, 255, 0.15);
    }

    .feature-icon {
        background: rgba(0, 0, 0, 0.3);
        color: #fff;
    }

    .feature-title {
        color: #fff;
    }

    .feature-desc {
        color: #ccc;
    }

    .version-button:hover .feature-icon {
        background: rgba(33, 150, 243, 0.3);
    }

    .announcement-button:hover .feature-icon {
        background: rgba(255, 152, 0, 0.3);
    }

    .tutorial-button:hover .feature-icon {
        background: rgba(76, 175, 80, 0.3);
    }
}

/* 配置顺序样式 */
.config-order {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #007bff;
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    font-weight: 500;
    margin-right: 8px;
    flex-shrink: 0;
}

/* 模态对话框动画 */
.modal-enter-active,
.modal-leave-active {
    transition: opacity 0.3s ease;
}

.modal-enter-active .custom-modal,
.modal-leave-active .custom-modal {
    transition: transform 0.3s ease, opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
    opacity: 0;
}

.modal-enter-from .custom-modal,
.modal-leave-to .custom-modal {
    transform: scale(0.9);
    opacity: 0;
}

/* 模态对话框样式 */
.custom-modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    -webkit-backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.custom-modal {
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    width: 400px;
    max-width: 90vw;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.modal-header {
    padding: 16px 20px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.modal-title {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
    color: #000;
}

.modal-content {
    padding: 20px;
}

.input-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.input-label {
    font-size: 14px;
    font-weight: 500;
    color: #000;
}

.config-input {
    padding: 10px 12px;
    border: 1px solid rgba(0, 0, 0, 0.2);
    border-radius: 4px;
    font-size: 14px;
    background: rgba(255, 255, 255, 0.9);
    transition: all 0.3s ease;
}

.config-input:focus {
    outline: none;
    border-color: #007bff;
    box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.2);
}

.modal-actions {
    padding: 16px 20px;
    border-top: 1px solid rgba(0, 0, 0, 0.1);
    display: flex;
    justify-content: flex-end;
    gap: 8px;
}

.modal-button {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
    min-width: 60px;
}

.cancel-button {
    background: rgba(0, 0, 0, 0.1);
    color: #000;
}

.cancel-button:hover {
    background: rgba(0, 0, 0, 0.2);
}

.confirm-button {
    background: #007bff;
    color: white;
}

.confirm-button:hover:not(:disabled) {
    background: #0056b3;
}

.confirm-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

/* 深色模式下的模态对话框样式 */
@media (prefers-color-scheme: dark) {
    .custom-modal {
        background: rgba(0, 0, 0, 0.95);
        border: 1px solid rgba(255, 255, 255, 0.2);
    }

    .modal-title {
        color: #fff;
    }

    .input-label {
        color: #fff;
    }

    .config-input {
        background: rgba(255, 255, 255, 0.1);
        border-color: rgba(255, 255, 255, 0.3);
        color: #fff;
    }

    .config-input:focus {
        border-color: #007bff;
    }

    .cancel-button {
        background: rgba(255, 255, 255, 0.1);
        color: #fff;
    }

    .cancel-button:hover {
        background: rgba(255, 255, 255, 0.2);
    }
}
</style>
