<template>
  <div class="tasks-container">
    <!-- 顶部控制栏 -->
    <div class="tasks-header">
      <div class="header-controls">
        <div class="config-selector">
          <label class="control-label">{{ t('tasks.selectConfig') }}:</label>
          <CustomDropdown
            v-model="currentConfigIndex"
            :options="configOptions"
            placeholder=""
            @update:modelValue="loadConfig"
          />
        </div>
        <button class="control-button save-button" @click="saveConfig">
          <Save :size="16" />
          {{ t('tasks.saveConfig') }}
        </button>
        <button class="control-button new-button" @click="showCreateDialog = true">
          <Plus :size="16" />
          {{ t('tasks.newConfig') }}
        </button>
        <button class="control-button execute-button" @click="saveAndExecute">
          <Play :size="16" />
          {{ t('tasks.saveAndExecute') }}
        </button>
      </div>
    </div>

    <!-- 标签页内容 -->
    <div class="tasks-content">
      <div class="tabs-header">
        <div
          v-for="(tab, index) in tabs"
          :key="index"
          class="tab-item"
          :class="{ active: activeTab === index }"
          @click="activeTab = index"
        >
          {{ tab }}
        </div>
      </div>

      <div class="tab-content">
        <!-- 标签页 1: 启动游戏 -->
        <div v-show="activeTab === 0" class="tab-panel">
          <StartGamePanel v-model="config" />
        </div>

        <!-- 标签页 2: 清体力 -->
        <div v-show="activeTab === 1" class="tab-panel">
          <TrailblazePowerPanel v-model="config" />
        </div>

        <!-- 标签页 3: 领取奖励 -->
        <div v-show="activeTab === 2" class="tab-panel">
          <ReceiveRewardPanel v-model="config" />
        </div>

        <!-- 标签页 4: 旷宇纷争 -->
        <div v-show="activeTab === 3" class="tab-panel">
          <CosmicStrifePanel v-model="config" />
        </div>

        <!-- 标签页 5: 完成任务后 -->
        <div v-show="activeTab === 4" class="tab-panel">
          <AfterMissionPanel v-model="config" />
        </div>
      </div>
    </div>

    <!-- 新建配置对话框 -->
    <Transition name="modal">
      <div v-if="showCreateDialog" class="custom-modal-overlay" @click.self="showCreateDialog = false">
        <div class="custom-modal">
          <div class="modal-header">
            <h3 class="modal-title">{{ t('tasks.newConfig') }}</h3>
          </div>
          <div class="modal-content">
            <div class="input-group">
              <label for="config-name" class="input-label">{{ t('tasks.configName') }}:</label>
              <input
                id="config-name"
                v-model="newConfigName"
                type="text"
                class="config-input"
                :placeholder="t('tasks.configNamePlaceholder').value"
                @keyup.enter="confirmCreateConfig"
              />
            </div>
          </div>
          <div class="modal-actions">
            <button class="modal-button cancel-button" @click="cancelCreateConfig">{{ t('tasks.cancel') }}</button>
            <button class="modal-button confirm-button" @click="confirmCreateConfig" :disabled="!newConfigName.trim()">{{ t('tasks.confirm') }}</button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Save, Plus, Play } from 'lucide-vue-next'
import CustomDropdown from '../components/CustomDropdown.vue'
import StartGamePanel from '../components/tasks/StartGamePanel.vue'
import TrailblazePowerPanel from '../components/tasks/TrailblazePowerPanel.vue'
import ReceiveRewardPanel from '../components/tasks/ReceiveRewardPanel.vue'
import CosmicStrifePanel from '../components/tasks/CosmicStrifePanel.vue'
import AfterMissionPanel from '../components/tasks/AfterMissionPanel.vue'

interface TaskConfig {
  Name: string
  Version: number
  EnabledTasks: boolean[]
  
  // 启动游戏
  StartGameChannel: number
  StartGamePath: string
  StartGameAutoLogin: boolean
  StartGameAlwaysLogin: boolean
  StartGameUsername: string
  StartGamePassword: string
  
  // 清体力
  TrailblazePowerTaskList: any[]
  TrailblazePowerReplenishStamina: boolean
  TrailblazePowerReplenishWay: number
  TrailblazePowerReplenishTimes: number
  TrailblazePowerUseAssistant: boolean
  TrailblazePowerChangeLineup: boolean
  TrailblazePowerLineupCheck: boolean
  TrailblazePowerUseSkill: boolean
  
  // 领取奖励
  ReceiveRewards: boolean[]
  ReceiveRewardRedeemCodes: string
  
  // 旷宇纷争
  SimulatedUniverseEnable: boolean
  SimulatedUniverseMode: number
  SimulatedUniverseRunTimes: number
  CurrencyWarsEnable: boolean
  CurrencyWarsUsername: string
  CurrencyWarsPolicy: number
  CurrencyWarsRunTimes: number
  
  // 完成任务后
  AfterLogout: boolean
  AfterExitGame: boolean
  AfterExitApp: boolean
  AfterShutdown: boolean
  AfterSleep: boolean
}

import { useTranslation } from '../composables/useTranslation'

const { t } = useTranslation()

const tabs = computed(() => [
  t('tasks.startGame.title').value,
  t('tasks.trailblazePower.title').value,
  t('tasks.receiveReward.title').value,
  t('tasks.cosmicStrife.title').value,
  t('tasks.afterMission.title').value
])
const activeTab = ref(0)

const configs = ref<string[]>([])
const currentConfigIndex = ref(0)
const showCreateDialog = ref(false)
const newConfigName = ref('')

// 从 localStorage 加载上次选择的配置
const loadLastSelectedConfig = () => {
  try {
    const saved = localStorage.getItem('tasks-selected-config')
    return saved || null
  } catch {
    return null
  }
}

// 保存当前选择的配置到 localStorage
const saveSelectedConfig = (configName: string) => {
  try {
    localStorage.setItem('tasks-selected-config', configName)
  } catch (err) {
    console.error('Failed to save selected config:', err)
  }
}

const config = ref<TaskConfig>({
  Name: 'Default',
  Version: 3,
  EnabledTasks: [false, false, false, false, false],
  
  StartGameChannel: 0,
  StartGamePath: '',
  StartGameAutoLogin: false,
  StartGameAlwaysLogin: false,
  StartGameUsername: '',
  StartGamePassword: '',
  
  TrailblazePowerTaskList: [],
  TrailblazePowerReplenishStamina: false,
  TrailblazePowerReplenishWay: 0,
  TrailblazePowerReplenishTimes: 0,
  TrailblazePowerUseAssistant: false,
  TrailblazePowerChangeLineup: false,
  TrailblazePowerLineupCheck: false,
  TrailblazePowerUseSkill: false,
  
  ReceiveRewards: [false, false, false, false, false, false, false],
  ReceiveRewardRedeemCodes: '',
  
  SimulatedUniverseEnable: false,
  SimulatedUniverseMode: 0,
  SimulatedUniverseRunTimes: 0,
  CurrencyWarsEnable: false,
  CurrencyWarsUsername: '',
  CurrencyWarsPolicy: 0,
  CurrencyWarsRunTimes: 0,
  
  AfterLogout: false,
  AfterExitGame: false,
  AfterExitApp: false,
  AfterShutdown: false,
  AfterSleep: false
})

const configOptions = computed(() => {
  return configs.value.map((name, index) => ({
    label: name,
    value: index
  }))
})

const loadConfigs = async () => {
  try {
    const configNames = await invoke<string[]>('get_config_list')
    configs.value = configNames
  } catch (error) {
    console.error('Failed to load configs:', error)
  }
}

const loadConfig = async () => {
  try {
    const configName = configs.value[currentConfigIndex.value]
    if (!configName) return
    
    const loadedConfig = await invoke<TaskConfig>('load_config', { name: configName })
    config.value = loadedConfig
    
    // 保存选择
    saveSelectedConfig(configName)
  } catch (error) {
    console.error('Failed to load config:', error)
  }
}

const saveConfig = async () => {
  try {
    await invoke('save_config', { config: config.value })
    window.showNotification?.(t('tasks.notifications.configSaved').value, false, 3000)
  } catch (error) {
    console.error('Failed to save config:', error)
    window.showNotification?.(t('tasks.notifications.configSaveFailed').value, false, 3000)
  }
}

const saveAndExecute = async () => {
  try {
    // 先保存配置
    await saveConfig()
    
    // 验证配置
    const { validateCurrencyWars } = await import('../utils/configValidator')
    const validationError = validateCurrencyWars(config.value)
    if (validationError) {
      window.showNotification?.(validationError, false, 5000)
      return
    }
    
    // 执行当前选中的配置
    const configName = configs.value[currentConfigIndex.value]
    await invoke('task_run', { configName })
    
    window.showNotification?.(t('tasks.notifications.taskStarted', { name: configName }).value, false, 3000)
  } catch (error) {
    console.error('Failed to save and execute:', error)
    window.showNotification?.(t('tasks.notifications.executeFailed').value, false, 3000)
  }
}

const confirmCreateConfig = async () => {
  if (!newConfigName.value.trim()) return

  try {
    await invoke('create_new_config', { name: newConfigName.value.trim() })
    await loadConfigs()
    showCreateDialog.value = false
    newConfigName.value = ''
    window.showNotification?.(t('tasks.notifications.configCreated').value, false, 3000)
  } catch (error) {
    console.error('Failed to create config:', error)
    window.showNotification?.(t('tasks.notifications.configCreateFailed').value, false, 3000)
  }
}

const cancelCreateConfig = () => {
  showCreateDialog.value = false
  newConfigName.value = ''
}

onMounted(async () => {
  await loadConfigs()
  
  if (configs.value.length > 0) {
    // 尝试恢复上次选择的配置
    const lastSelected = loadLastSelectedConfig()
    if (lastSelected) {
      const index = configs.value.indexOf(lastSelected)
      if (index !== -1) {
        // 找到了上次选择的配置
        currentConfigIndex.value = index
      } else {
        // 上次选择的配置已被删除，切换到 Default
        const defaultIndex = configs.value.indexOf('Default')
        currentConfigIndex.value = defaultIndex !== -1 ? defaultIndex : 0
      }
    }
    
    await loadConfig()
  }
  
})
</script>

<style scoped>
.tasks-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 16px;
  box-sizing: border-box;
}

.tasks-header {
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  padding: 12px 16px;
  margin-bottom: 16px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.header-controls {
  display: flex;
  align-items: center;
  gap: 12px;
}

.config-selector {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
}

.control-label {
  font-size: 14px;
  font-weight: 500;
  color: #000;
  white-space: nowrap;
}

.control-button {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  white-space: nowrap;
}

.save-button {
  background: rgba(76, 175, 80, 0.8);
  color: white;
}

.save-button:hover {
  background: rgba(76, 175, 80, 0.9);
  transform: translateY(-1px);
}

.new-button {
  background: rgba(33, 150, 243, 0.8);
  color: white;
}

.new-button:hover {
  background: rgba(33, 150, 243, 0.9);
  transform: translateY(-1px);
}

.execute-button {
  background: rgba(255, 152, 0, 0.8);
  color: white;
}

.execute-button:hover {
  background: rgba(255, 152, 0, 0.9);
  transform: translateY(-1px);
}

.tasks-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.tabs-header {
  display: flex;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  background: rgba(255, 255, 255, 0.5);
}

.tab-item {
  flex: 1;
  padding: 12px 16px;
  text-align: center;
  font-size: 14px;
  font-weight: 500;
  color: #666;
  cursor: pointer;
  transition: all 0.3s ease;
  border-bottom: 2px solid transparent;
}

.tab-item:hover {
  background: rgba(0, 0, 0, 0.05);
  color: #000;
}

.tab-item.active {
  color: #007bff;
  border-bottom-color: #007bff;
  background: rgba(0, 123, 255, 0.05);
}

.tab-content {
  flex: 1;
  overflow: hidden;
}

.tab-panel {
  height: 100%;
  overflow-y: auto;
  padding: 16px;
  box-sizing: border-box;
}

/* 美化滚动条 */
.tab-panel::-webkit-scrollbar {
  width: 8px;
}

.tab-panel::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 4px;
}

.tab-panel::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
}

.tab-panel::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
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

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .tasks-header,
  .tasks-content {
    background: rgba(0, 0, 0, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .control-label {
    color: #fff;
  }

  .tabs-header {
    background: rgba(255, 255, 255, 0.1);
    border-bottom: 1px solid rgba(255, 255, 255, 0.2);
  }

  .tab-item {
    color: #ccc;
  }

  .tab-item:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
  }

  .tab-item.active {
    color: #007bff;
    background: rgba(0, 123, 255, 0.2);
  }

  .custom-modal {
    background: rgba(0, 0, 0, 0.95);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .modal-title,
  .input-label {
    color: #fff;
  }

  .config-input {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.3);
    color: #fff;
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
