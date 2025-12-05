<template>
  <div class="settings-container">
    <!-- 设置标题 -->
    <div class="settings-header">
      <h2 class="settings-title">{{ t('settings.title') }}</h2>
      <div class="settings-actions">
        <button class="action-button save-button" @click="saveSettings" :disabled="!hasChanges">
          <Save class="button-icon" :size="16" />
          {{ t('settings.saveSettings') }}
        </button>
        <button class="action-button reset-button" @click="resetSettings">
          <RotateCcw class="button-icon" :size="16" />
          {{ t('settings.resetSettings') }}
        </button>
      </div>
    </div>

    <!-- 设置内容 -->
    <div class="settings-content">
      <!-- 主题设置 -->
      <div class="settings-section">
        <div class="section-header">
          <Palette class="section-icon" :size="20" />
          <h3 class="section-title">{{ t('settings.theme.title') }}</h3>
        </div>
        <div class="section-content">
          <div class="setting-item">
            <label class="setting-label">{{ t('settings.theme.customWallpaper') }}</label>
            <div class="wallpaper-controls">
              <button class="control-button select-button" @click="selectWallpaper">
                <Image class="button-icon" :size="16" />
                {{ t('settings.theme.selectWallpaper') }}
              </button>
              <button class="control-button reset-button" @click="resetWallpaper">
                <RotateCcw class="button-icon" :size="16" />
                {{ t('settings.theme.resetWallpaper') }}
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 界面设置 -->
      <div class="settings-section">
        <div class="section-header">
          <Monitor class="section-icon" :size="20" />
          <h3 class="section-title">{{ t('settings.interface.title') }}</h3>
        </div>
        <div class="section-content">
          <div class="setting-item">
            <label class="setting-label">{{ t('settings.interface.language') }}</label>
            <CustomDropdown
              v-model="settings.language"
              :options="[
                { label: '简体中文', value: 0 },
                { label: 'English', value: 1 }
              ]"
              placeholder="选择语言"
              @update:modelValue="onLanguageChange"
            />
          </div>
          <div class="setting-note">
            <Info :size="16" class="note-icon" />
            <span class="note-text">{{ languageNote }}</span>
          </div>
          <div class="setting-item">
            <label class="setting-label">{{ t('settings.interface.zoom') }}</label>
            <input
              v-model.number="settings.zoom"
              type="number"
              min="50"
              max="200"
              step="5"
              class="setting-input"
            />
            <span class="input-suffix">%</span>
          </div>
          <div class="setting-item">
            <label class="setting-label">{{ t('settings.interface.confidence') }}</label>
            <input
              v-model="settings.confidence_threshold"
              type="range"
              min="0.5"
              max="1.0"
              step="0.05"
              class="setting-slider"
            />
            <span class="slider-value">{{ (settings.confidence_threshold * 100).toFixed(0) }}%</span>
          </div>
        </div>
      </div>

      <!-- 游戏内快捷键 -->
      <div class="settings-section">
        <div class="section-header">
          <Settings as="div" class="section-icon" :size="20" />
          <h3 class="section-title">{{ t('settings.hotkeys.title') }}</h3>
        </div>
        <div class="section-content">
          <div class="setting-note">
            <Info :size="16" class="note-icon" />
            <span class="note-text">{{ t('settings.hotkeys.clickToSet') }}</span>
          </div>
          <div class="setting-item hotkey-item">
            <label class="setting-label">{{ t('settings.hotkeys.activity') }}</label>
            <div class="hotkey-controls">
              <input
                :value="settings.activity_hotkey"
                type="text"
                class="setting-input hotkey-input"
                :class="{ 'listening': listeningKey === 'activity_hotkey' }"
                :placeholder="getHotkeyPlaceholder('activity_hotkey', 'F1')"
                readonly
                @click="startListening('activity_hotkey')"
              />
              <button class="control-button hotkey-reset-button" @click="resetHotkey('activity_hotkey', 'F1')">
                <RotateCcw class="button-icon" :size="14" />
              </button>
            </div>
          </div>
          <div class="setting-item hotkey-item">
            <label class="setting-label">{{ t('settings.hotkeys.chronicle') }}</label>
            <div class="hotkey-controls">
              <input
                :value="settings.chronicle_hotkey"
                type="text"
                class="setting-input hotkey-input"
                :class="{ 'listening': listeningKey === 'chronicle_hotkey' }"
                :placeholder="getHotkeyPlaceholder('chronicle_hotkey', 'F2')"
                readonly
                @click="startListening('chronicle_hotkey')"
              />
              <button class="control-button hotkey-reset-button" @click="resetHotkey('chronicle_hotkey', 'F2')">
                <RotateCcw class="button-icon" :size="14" />
              </button>
            </div>
          </div>
          <div class="setting-item hotkey-item">
            <label class="setting-label">{{ t('settings.hotkeys.warp') }}</label>
            <div class="hotkey-controls">
              <input
                :value="settings.warp_hotkey"
                type="text"
                class="setting-input hotkey-input"
                :class="{ 'listening': listeningKey === 'warp_hotkey' }"
                :placeholder="getHotkeyPlaceholder('warp_hotkey', 'F3')"
                readonly
                @click="startListening('warp_hotkey')"
              />
              <button class="control-button hotkey-reset-button" @click="resetHotkey('warp_hotkey', 'F3')">
                <RotateCcw class="button-icon" :size="14" />
              </button>
            </div>
          </div>
          <div class="setting-item hotkey-item">
            <label class="setting-label">{{ t('settings.hotkeys.guide') }}</label>
            <div class="hotkey-controls">
              <input
                :value="settings.guide_hotkey"
                type="text"
                class="setting-input hotkey-input"
                :class="{ 'listening': listeningKey === 'guide_hotkey' }"
                :placeholder="getHotkeyPlaceholder('guide_hotkey', 'F4')"
                readonly
                @click="startListening('guide_hotkey')"
              />
              <button class="control-button hotkey-reset-button" @click="resetHotkey('guide_hotkey', 'F4')">
                <RotateCcw class="button-icon" :size="14" />
              </button>
            </div>
          </div>
          <div class="setting-item hotkey-item">
            <label class="setting-label">{{ t('settings.hotkeys.map') }}</label>
            <div class="hotkey-controls">
              <input
                :value="settings.map_hotkey"
                type="text"
                class="setting-input hotkey-input"
                :class="{ 'listening': listeningKey === 'map_hotkey' }"
                :placeholder="getHotkeyPlaceholder('map_hotkey', 'M')"
                readonly
                @click="startListening('map_hotkey')"
              />
              <button class="control-button hotkey-reset-button" @click="resetHotkey('map_hotkey', 'M')">
                <RotateCcw class="button-icon" :size="14" />
              </button>
            </div>
          </div>
          <div class="setting-item hotkey-item">
            <label class="setting-label">{{ t('settings.hotkeys.technique') }}</label>
            <div class="hotkey-controls">
              <input
                :value="settings.technique_hotkey"
                type="text"
                class="setting-input hotkey-input"
                :class="{ 'listening': listeningKey === 'technique_hotkey' }"
                :placeholder="getHotkeyPlaceholder('technique_hotkey', 'E')"
                readonly
                @click="startListening('technique_hotkey')"
              />
              <button class="control-button hotkey-reset-button" @click="resetHotkey('technique_hotkey', 'E')">
                <RotateCcw class="button-icon" :size="14" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 通知设置 -->
      <div class="settings-section">
        <div class="section-header">
          <Globe class="section-icon" :size="20" />
          <h3 class="section-title">{{ t('settings.notifications.title') }}</h3>
        </div>
        <div class="section-content">
          <div class="setting-item checkbox-item">
            <input
              v-model="settings.allow_notifications"
              type="checkbox"
              id="allow-notifications"
              class="setting-checkbox"
            />
            <label for="allow-notifications" class="checkbox-label">{{ t('settings.notifications.allowNotifications') }}</label>
          </div>
          <div class="setting-item checkbox-item">
            <input
              v-model="settings.allow_system_notifications"
              type="checkbox"
              id="allow-system-notifications"
              class="setting-checkbox"
            />
            <label for="allow-system-notifications" class="checkbox-label">{{ t('settings.notifications.allowSystemNotifications') }}</label>
          </div>
          <div class="setting-item checkbox-item">
            <input
              v-model="settings.allow_email_notifications"
              type="checkbox"
              id="allow-email-notifications"
              class="setting-checkbox"
            />
            <label for="allow-email-notifications" class="checkbox-label">{{ t('settings.notifications.allowEmailNotifications') }}</label>
          </div>
          
          <!-- SMTP设置 -->
          <div v-if="settings.allow_email_notifications" class="smtp-settings">
            <h4 class="smtp-title">{{ t('settings.notifications.smtpSettings') }}</h4>
            <div class="setting-item">
              <label class="setting-label">{{ t('settings.notifications.smtpServer') }}</label>
              <input
                v-model="settings.smtp_server"
                type="text"
                class="setting-input"
                placeholder="smtp.qq.com"
              />
            </div>
            <div class="setting-item">
              <label class="setting-label">{{ t('settings.notifications.smtpPort') }}</label>
              <input
                v-model.number="settings.smtp_port"
                type="number"
                class="setting-input"
                placeholder="465"
              />
            </div>
            <div class="setting-item">
              <label class="setting-label">{{ t('settings.notifications.emailSender') }}</label>
              <input
                v-model="settings.email_sender"
                type="text"
                class="setting-input"
                :placeholder="t('settings.notifications.emailSenderPlaceholder').value"
              />
            </div>
            <div class="setting-item">
              <label class="setting-label">{{ t('settings.notifications.emailAuthCode') }}</label>
              <input
                v-model="settings.email_auth_code"
                type="password"
                class="setting-input"
                :placeholder="t('settings.notifications.emailAuthCodePlaceholder').value"
              />
            </div>
            <div class="setting-item">
              <label class="setting-label">{{ t('settings.notifications.emailReceiver') }}</label>
              <input
                v-model="settings.email_receiver"
                type="text"
                class="setting-input"
                :placeholder="t('settings.notifications.emailReceiverPlaceholder').value"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- 应用设置 -->
      <div class="settings-section">
        <div class="section-header">
          <Settings as="div" class="section-icon" :size="20" />
          <h3 class="section-title">{{ t('settings.app.title') }}</h3>
        </div>
        <div class="section-content">
          <div class="setting-item checkbox-item">
            <input
              v-model="settings.enable_startup_launch"
              type="checkbox"
              id="enable-startup-launch"
              class="setting-checkbox"
            />
            <label for="enable-startup-launch" class="checkbox-label">{{ t('settings.app.enableStartupLaunch') }}</label>
          </div>
          <div class="setting-item checkbox-item">
            <input
              v-model="settings.enable_minimize_to_tray"
              type="checkbox"
              id="enable-minimize-to-tray"
              class="setting-checkbox"
            />
            <label for="enable-minimize-to-tray" class="checkbox-label">{{ t('settings.app.enableMinimizeToTray') }}</label>
          </div>
        </div>
      </div>
    </div>

    <!-- 重置设置确认模态框 -->
    <div v-if="showResetModal" class="custom-modal-overlay" @click.self="cancelReset">
      <div class="custom-modal">
        <div class="modal-header">
          <h3 class="modal-title">{{ t('settings.resetSettings') }}</h3>
        </div>
        <div class="modal-content">
          <p>{{ t('settings.resetConfirm') }}</p>
        </div>
        <div class="modal-actions">
          <button class="modal-button cancel-button" @click="cancelReset">{{ t('common.cancel') }}</button>
          <button class="modal-button confirm-button" @click="confirmReset">{{ t('common.confirm') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import {
  Save,
  RotateCcw,
  Palette,
  Monitor,
  Settings,
  Globe,
  Image,
  Info
} from 'lucide-vue-next'
import CustomDropdown from '../components/CustomDropdown.vue'
import { setLocale } from '../locales'
import { useTranslation } from '../composables/useTranslation'

const { t } = useTranslation()

interface AppSettings {
  language: number
  zoom: number
  confidence_threshold: number
  allow_notifications: boolean
  allow_system_notifications: boolean
  allow_email_notifications: boolean
  smtp_server: string
  smtp_port: number
  email_sender: string
  email_auth_code: string
  email_receiver: string
  activity_hotkey: string
  chronicle_hotkey: string
  warp_hotkey: string
  guide_hotkey: string
  map_hotkey: string
  technique_hotkey: string
  enable_startup_launch: boolean
  enable_minimize_to_tray: boolean
  wallpaper_path: string | null
}

// 响应式数据
const originalSettings = ref<AppSettings | null>(null)
const settings = reactive<AppSettings>({
  language: 0, // 默认中文
  zoom: 125,
  confidence_threshold: 0.9,
  allow_notifications: true,
  allow_system_notifications: true,
  allow_email_notifications: false,
  smtp_server: 'smtp.qq.com',
  smtp_port: 465,
  email_sender: '',
  email_auth_code: '',
  email_receiver: '',
  activity_hotkey: 'F1',
  chronicle_hotkey: 'F2',
  warp_hotkey: 'F3',
  guide_hotkey: 'F4',
  map_hotkey: 'M',
  technique_hotkey: 'E',
  enable_startup_launch: false,
  enable_minimize_to_tray: false,
  wallpaper_path: null
})

const showResetModal = ref(false)
const listeningKey = ref<string | null>(null)

// 计算属性
const hasChanges = computed(() => {
  if (!originalSettings.value) return false
  return JSON.stringify(settings) !== JSON.stringify(originalSettings.value)
})

const languageNote = computed(() => {
  return settings.language === 0 
    ? '使用该程序进行自动化需要您游戏界面设置为简体中文'
    : 'To use automation features, your game interface must be set to Simplified Chinese'
})



// 方法
const onLanguageChange = (value: string | number) => {
  // 切换界面语言 (0=中文, 1=英文)
  const lang = typeof value === 'number' ? value : parseInt(value)
  setLocale(lang === 0 ? 'zh-CN' : 'en-US')
}

// 方法
const loadSettings = async () => {
  try {
    // 从后端加载设置
    const loadedSettings = await invoke<AppSettings>('load_app_settings')
    Object.assign(settings, loadedSettings)
    originalSettings.value = JSON.parse(JSON.stringify(settings))
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : t('settings.notifications.unknownError').value
    await window.logToConsole?.('前端', 'ERR', t('console.logs.loadSettingsFailed', { error: errorMsg }).value)
  }
}

const saveSettings = async () => {
  try {
    // 保存到后端
    await invoke('save_app_settings', { settings })
    originalSettings.value = JSON.parse(JSON.stringify(settings))
    
    window.showNotification?.(t('settings.notifications.settingsSaved').value, false, 3000)
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : t('settings.notifications.unknownError').value
    await window.logToConsole?.('前端', 'ERR', `${t('settings.notifications.saveSettingsFailed').value}: ${errorMsg}`)
    window.showNotification?.(t('settings.notifications.saveSettingsFailed').value, false, 3000)
  }
}

const resetSettings = () => {
  showResetModal.value = true
  document.body.classList.add('modal-open')
}

const confirmReset = () => {
  Object.assign(settings, {
    language: 0, // 默认中文
    zoom: 125,
    confidence_threshold: 0.9,
    allow_notifications: true,
    allow_system_notifications: true,
    allow_email_notifications: false,
    smtp_server: 'smtp.qq.com',
    smtp_port: 465,
    email_sender: '',
    email_auth_code: '',
    email_receiver: '',
    activity_hotkey: 'F1',
    chronicle_hotkey: 'F2',
    warp_hotkey: 'F3',
    guide_hotkey: 'F4',
    map_hotkey: 'M',
    technique_hotkey: 'E',
    enable_startup_launch: false,
    enable_minimize_to_tray: false,
    wallpaper_path: null
  })
  resetWallpaper()
  showResetModal.value = false
  document.body.classList.remove('modal-open')
}

const cancelReset = () => {
  showResetModal.value = false
  document.body.classList.remove('modal-open')
}

const selectWallpaper = async () => {
  try {
    await window.logToConsole?.('前端', 'INFO', t('console.logs.openWallpaperDialog').value)
    
    const selected = await open({
      multiple: false,
      filters: [
        { name: '图片文件', extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif'] },
        { name: '所有文件', extensions: ['*'] }
      ],
      title: '选择壁纸'
    })
    
    if (!selected) {
      await window.logToConsole?.('前端', 'INFO', t('console.logs.userCancelledWallpaper').value)
      return
    }
    
    await window.logToConsole?.('前端', 'DEBUG', t('console.logs.userSelectedWallpaper', { path: selected }).value)
    
    // 调用后端命令设置壁纸（会复制到缓存目录）
    await window.logToConsole?.('前端', 'DEBUG', t('console.logs.callSetWallpaper').value)
    const cachedPath = await invoke<string>('set_wallpaper', { sourcePath: selected })
    await window.logToConsole?.('前端', 'DEBUG', t('console.logs.wallpaperCached', { path: cachedPath }).value)
    
    settings.wallpaper_path = cachedPath
    
    await window.logToConsole?.('前端', 'SUCCESS', t('settings.notifications.wallpaperSet').value)
    window.showNotification?.(t('settings.notifications.wallpaperSet').value, false, 3000)
    
    // 获取base64数据并立即应用壁纸
    await window.logToConsole?.('前端', 'DEBUG', t('console.logs.callGetWallpaperBase64').value)
    const base64Data = await invoke<string | null>('get_wallpaper_base64')
    const result = base64Data ? t('console.logs.hasDataWithSize', { size: base64Data.length }).value : t('console.logs.noData').value
    await window.logToConsole?.('前端', 'DEBUG', t('console.logs.getWallpaperBase64Result', { result }).value)
    
    if (base64Data) {
      await window.logToConsole?.('前端', 'DEBUG', t('console.logs.startApplyWallpaper').value)
      applyWallpaperToBackground(base64Data)
      await window.logToConsole?.('前端', 'SUCCESS', t('console.logs.wallpaperApplied').value)
    } else {
      await window.logToConsole?.('前端', 'WARN', t('console.logs.getWallpaperBase64Empty').value)
    }
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : t('settings.notifications.unknownError').value
    await window.logToConsole?.('前端', 'ERR', `${t('settings.notifications.selectWallpaperFailed').value}: ${errorMsg}`)
    window.showNotification?.(t('settings.notifications.selectWallpaperFailed').value, false, 3000)
  }
}

const resetWallpaper = async () => {
  try {
    await window.logToConsole?.('前端', 'INFO', t('console.logs.startResetWallpaper').value)
    
    // 调用后端命令重置壁纸
    await invoke('reset_wallpaper')
    
    settings.wallpaper_path = null
    
    await window.logToConsole?.('前端', 'SUCCESS', t('settings.notifications.wallpaperReset').value)
    window.showNotification?.(t('settings.notifications.wallpaperReset').value, false, 3000)
    
    // 立即移除背景壁纸
    removeWallpaperFromBackground()
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : t('settings.notifications.unknownError').value
    await window.logToConsole?.('前端', 'ERR', `${t('settings.notifications.resetWallpaperFailed').value}: ${errorMsg}`)
    window.showNotification?.(t('settings.notifications.resetWallpaperFailed').value, false, 3000)
  }
}

// 应用壁纸到背景
const applyWallpaperToBackground = (base64Data: string) => {
  const appContainer = document.querySelector('.app-container') as HTMLElement
  if (appContainer) {
    window.logToConsole?.('前端', 'DEBUG', t('console.logs.wallpaperDataLength', { length: base64Data.length }).value)
    
    // 使用setProperty设置CSS变量，确保优先级
    appContainer.style.setProperty('background-image', `url('${base64Data}')`, 'important')
    appContainer.style.setProperty('background-size', 'cover', 'important')
    appContainer.style.setProperty('background-position', 'center', 'important')
    appContainer.style.setProperty('background-repeat', 'no-repeat', 'important')
    
    window.logToConsole?.('前端', 'DEBUG', t('console.logs.wallpaperStyleApplied').value)
  } else {
    window.logToConsole?.('前端', 'ERR', t('console.logs.appContainerNotFound').value)
  }
}

// 移除背景壁纸
const removeWallpaperFromBackground = () => {
  const appContainer = document.querySelector('.app-container') as HTMLElement
  if (appContainer) {
    appContainer.style.removeProperty('background-image')
    window.logToConsole?.('前端', 'DEBUG', t('console.logs.wallpaperStyleRemoved').value)
  }
}

// 快捷键监听相关方法
const getHotkeyPlaceholder = (key: string, defaultKey: string) => {
  return listeningKey.value === key ? t('settings.hotkeys.pressKey').value : defaultKey
}

const startListening = (key: string) => {
  listeningKey.value = key
  // 添加全局键盘监听
  window.addEventListener('keyup', handleKeyPress)
  window.addEventListener('click', handleClickOutside, true)
}

const stopListening = () => {
  listeningKey.value = null
  window.removeEventListener('keyup', handleKeyPress)
  window.removeEventListener('click', handleClickOutside, true)
}

const handleKeyPress = (event: KeyboardEvent) => {
  event.preventDefault()
  event.stopPropagation()
  
  if (!listeningKey.value) return
  
  // 使用 event.code 获取物理按键代码（如 "Digit3", "KeyA", "F1" 等）
  // 这样可以得到类似源码中的 "D3" 效果
  let keyName = event.code
  
  // 处理特殊按键映射，使其更友好
  if (keyName.startsWith('Digit')) {
    // Digit0-9 -> D0-D9
    keyName = 'D' + keyName.substring(5)
  } else if (keyName.startsWith('Key')) {
    // KeyA-Z -> A-Z
    keyName = keyName.substring(3)
  } else if (keyName === 'Space') {
    keyName = 'Space'
  } else if (keyName === 'Minus') {
    keyName = '-'
  } else if (keyName === 'Equal') {
    keyName = '='
  } else if (keyName === 'BracketLeft') {
    keyName = '['
  } else if (keyName === 'BracketRight') {
    keyName = ']'
  } else if (keyName === 'Backslash') {
    keyName = '\\'
  } else if (keyName === 'Semicolon') {
    keyName = ';'
  } else if (keyName === 'Quote') {
    keyName = "'"
  } else if (keyName === 'Comma') {
    keyName = ','
  } else if (keyName === 'Period') {
    keyName = '.'
  } else if (keyName === 'Slash') {
    keyName = '/'
  } else if (keyName === 'Backquote') {
    keyName = '`'
  }
  // F1-F12, Enter, Escape 等保持原样
  
  // 更新设置
  const key = listeningKey.value
  if (key === 'activity_hotkey') settings.activity_hotkey = keyName
  else if (key === 'chronicle_hotkey') settings.chronicle_hotkey = keyName
  else if (key === 'warp_hotkey') settings.warp_hotkey = keyName
  else if (key === 'guide_hotkey') settings.guide_hotkey = keyName
  else if (key === 'map_hotkey') settings.map_hotkey = keyName
  else if (key === 'technique_hotkey') settings.technique_hotkey = keyName
  
  // 停止监听
  stopListening()
}

const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as HTMLElement
  // 如果点击的不是快捷键输入框，则停止监听
  if (!target.classList.contains('hotkey-input')) {
    stopListening()
  }
}

const resetHotkey = (key: string, defaultValue: string) => {
  if (key === 'activity_hotkey') settings.activity_hotkey = defaultValue
  else if (key === 'chronicle_hotkey') settings.chronicle_hotkey = defaultValue
  else if (key === 'warp_hotkey') settings.warp_hotkey = defaultValue
  else if (key === 'guide_hotkey') settings.guide_hotkey = defaultValue
  else if (key === 'map_hotkey') settings.map_hotkey = defaultValue
  else if (key === 'technique_hotkey') settings.technique_hotkey = defaultValue
}

// 生命周期
onMounted(async () => {
  await loadSettings()
  // 初始化语言 (0=中文, 1=英文)
  setLocale(settings.language === 0 ? 'zh-CN' : 'en-US')
})
</script>

<style scoped>
.settings-container {
  height: 100%;
  padding: 16px;
  overflow-y: auto;
  box-sizing: border-box;
}

/* 当模态框显示时防止页面滚动 */
body.modal-open {
  overflow: hidden;
}

/* 美化滚动条 */
.settings-container::-webkit-scrollbar {
  width: 8px;
}

.settings-container::-webkit-scrollbar-track {
  background: var(--scrollbar-track);
  border-radius: 4px;
}

.settings-container::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb);
  border-radius: 4px;
  transition: background-color 0.3s ease;
}

.settings-container::-webkit-scrollbar-thumb:hover {
  background: var(--scrollbar-thumb-hover);
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding: 16px 20px;
  background: var(--bg-secondary);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.settings-title {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: var(--text-color);
}

.settings-actions {
  display: flex;
  gap: 10px;
}

.action-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
}

.save-button {
  background: rgba(76, 175, 80, 0.8);
  color: white;
}

.save-button:hover:not(:disabled) {
  background: rgba(76, 175, 80, 0.9);
  transform: translateY(-1px);
}

.save-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.reset-button {
  background: rgba(255, 152, 0, 0.8);
  color: white;
}

.reset-button:hover {
  background: rgba(255, 152, 0, 0.9);
  transform: translateY(-1px);
}

.settings-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.settings-section {
  background: var(--bg-secondary);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.section-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border-color);
}

.section-icon {
  color: var(--text-secondary);
}

.section-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-color);
}

.section-content {
  padding: 16px;
}

.setting-item {
  display: flex;
  align-items: center;
  margin-bottom: 12px;
  padding: 8px 0;
}

.setting-note {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 12px;
  margin-top: -8px;
  margin-bottom: 12px;
  background: rgba(33, 150, 243, 0.1);
  border-left: 3px solid #2196f3;
  border-radius: 4px;
}

.note-icon {
  flex-shrink: 0;
  color: #2196f3;
  margin-top: 2px;
}

.note-text {
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.5;
}

.setting-item:last-child {
  margin-bottom: 0;
}

.setting-label {
  width: 180px;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-color);
  flex-shrink: 0;
}

.setting-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--border-secondary);
  border-radius: 4px;
  background: var(--input-bg);
  color: var(--text-color);
  font-size: 14px;
  transition: border-color 0.3s ease;
}

.setting-input:focus {
  outline: none;
  border-color: #007bff;
}

/* 快捷键设置项特殊样式 */
.hotkey-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.hotkey-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.hotkey-input {
  cursor: pointer;
  user-select: none;
  text-align: center;
  font-weight: 500;
  min-width: 120px;
  max-width: 150px;
  flex: 0 0 auto;
}

.hotkey-input:hover {
  border-color: #007bff;
  background: var(--bg-tertiary);
}

.hotkey-input.listening {
  border-color: #ff9800;
  background: rgba(255, 152, 0, 0.1);
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    box-shadow: 0 0 0 0 rgba(255, 152, 0, 0.4);
  }
  50% {
    box-shadow: 0 0 0 4px rgba(255, 152, 0, 0);
  }
}

.hotkey-reset-button {
  background: rgba(255, 152, 0, 0.8);
  color: white;
  padding: 8px;
  min-width: auto;
  flex-shrink: 0;
}

.hotkey-reset-button:hover {
  background: rgba(255, 152, 0, 0.9);
}

.input-suffix {
  margin-left: 8px;
  font-size: 14px;
  color: var(--text-secondary);
}

.setting-select {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--border-secondary);
  border-radius: 4px;
  background: var(--input-bg);
  color: var(--text-color);
  font-size: 14px;
  transition: border-color 0.3s ease;
}

.setting-select:focus {
  outline: none;
  border-color: #007bff;
}

.setting-slider {
  flex: 1;
  margin: 0 12px;
}

.slider-value {
  width: 50px;
  font-size: 14px;
  color: var(--text-secondary);
  text-align: right;
}

.checkbox-item {
  display: flex;
  align-items: center;
}

.setting-checkbox {
  margin-right: 8px;
  width: 16px;
  height: 16px;
  accent-color: #007bff;
}

.checkbox-label {
  font-size: 14px;
  color: var(--text-color);
  cursor: pointer;
}

.wallpaper-controls {
  display: flex;
  gap: 8px;
  align-items: center;
}

.control-button {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border: none;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
}

.select-button {
  background: rgba(33, 150, 243, 0.8);
  color: white;
}

.select-button:hover {
  background: rgba(33, 150, 243, 0.9);
}

.add-button {
  background: rgba(76, 175, 80, 0.8);
  color: white;
}

.add-button:hover {
  background: rgba(76, 175, 80, 0.9);
}



.proxies-list {
  margin-bottom: 8px;
}

.proxy-item {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.proxy-input {
  flex: 1;
  padding: 6px 10px;
  border: 1px solid rgba(0, 0, 0, 0.2);
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.9);
  font-size: 13px;
}

.proxy-remove {
  padding: 4px;
  border: none;
  border-radius: 4px;
  background: rgba(244, 67, 54, 0.8);
  color: white;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.proxy-remove:hover:not(:disabled) {
  background: rgba(244, 67, 54, 1);
}

.proxy-remove:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  flex-shrink: 0;
}

/* SMTP设置样式 */
.smtp-settings {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid var(--border-color);
}

.smtp-title {
  margin: 0 0 12px 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-color);
}

/* 自定义模态框样式 */
.custom-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(5px);
  -webkit-backdrop-filter: blur(5px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
}

.custom-modal {
  background: var(--bg-tertiary);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 0;
  min-width: 400px;
  max-width: 500px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  animation: modal-appear 0.3s ease;
}

@keyframes modal-appear {
  from {
    opacity: 0;
    transform: scale(0.9) translateY(-20px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.modal-header {
  padding: 20px 24px 0;
  border-bottom: 1px solid var(--border-color);
}

.modal-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-color);
}

.modal-content {
  padding: 24px;
}

.modal-content p {
  margin: 0;
  font-size: 14px;
  color: var(--text-color);
  line-height: 1.5;
}

.modal-actions {
  padding: 0 24px 20px;
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.modal-button {
  padding: 8px 20px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 80px;
}

.cancel-button {
  background: rgba(0, 0, 0, 0.1);
  color: #000;
}

.cancel-button:hover {
  background: rgba(0, 0, 0, 0.2);
}

.confirm-button {
  background: rgba(244, 67, 54, 0.8);
  color: white;
}

.confirm-button:hover {
  background: rgba(244, 67, 54, 1);
}

/* 所有深色模式样式现在通过CSS变量自动处理 */
</style>