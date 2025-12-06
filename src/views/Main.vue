<template>
  <div class="app-container">
    <!-- 通知系统 -->
    <NotificationSystem ref="notificationSystem" />

    <!-- 左侧导航栏 -->
    <aside class="sidebar">
      <div class="sidebar-content">
        <!-- 应用图标 -->
        <div class="app-info">
          <img src="../assets/SRAico.png" alt="StarRailAssistant" class="app-icon" />
          <h2 class="app-name">StarRailAssistant</h2>
          <p class="app-version">v{{ appVersion }} Community Edition</p>
        </div>

        <!-- 导航菜单 -->
        <nav class="nav-menu">
          <router-link to="/" class="nav-item" :class="{ active: isActive('/') }">
            <House class="nav-icon" />
            <span class="nav-text">{{ t('nav.home') }}</span>
          </router-link>
          <router-link to="/tasks" class="nav-item" :class="{ active: isActive('/tasks') }">
            <SquareCheck class="nav-icon" />
            <span class="nav-text">{{ t('nav.tasks') }}</span>
          </router-link>
          <router-link to="/extensions" class="nav-item" :class="{ active: isActive('/extensions') }">
            <Plug class="nav-icon" />
            <span class="nav-text">{{ t('nav.extensions') }}</span>
          </router-link>
          <router-link to="/console" class="nav-item" :class="{ active: isActive('/console') }">
            <SquareTerminal class="nav-icon" />
            <span class="nav-text">{{ t('nav.console') }}</span>
          </router-link>
          <router-link to="/settings" class="nav-item" :class="{ active: isActive('/settings') }">
            <Settings class="nav-icon" />
            <span class="nav-text">{{ t('nav.settings') }}</span>
          </router-link>
        </nav>
      </div>
    </aside>

    <!-- 主内容区域包装器 -->
    <div class="main-wrapper">
      <main class="main-content">
        <router-view v-slot="{ Component }">
          <keep-alive>
            <component :is="Component" />
          </keep-alive>
        </router-view>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { House, SquareCheck, Plug, SquareTerminal, Settings } from 'lucide-vue-next'
import NotificationSystem from '../components/NotificationSystem.vue'
import { useTranslation } from '../composables/useTranslation'

const { t } = useTranslation()

const route = useRoute()
const appVersion = ref('0.1.0') // 从 Tauri 配置获取版本号
const isExecutingTask = ref(false)
let unlistenStatusChange: (() => void) | null = null

const isActive = (path: string) => {
  return route.path === path
}

// 加载默认背景图片
const loadDefaultBackground = async () => {
  try {
    // 动态导入背景图片，让 Vite 正确处理路径
    const backgroundModule = await import('../assets/background-lt.jpg')
    const backgroundUrl = backgroundModule.default
    
    // 设置 CSS 变量
    const appContainer = document.querySelector('.app-container') as HTMLElement
    if (appContainer) {
      appContainer.style.setProperty('--default-background', `url('${backgroundUrl}')`)
    }
  } catch (error) {
    console.error('Failed to load default background:', error)
  }
}

// 检查是否需要提示创建桌面快捷方式
const checkDesktopShortcut = async () => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const needed = await invoke<boolean>('check_desktop_shortcut_needed')
    
    if (needed) {
      // 使用新的常驻通知API询问用户
      window.showPersistentNotification?.(
        t('home.notifications.createShortcut').value,
        [
          {
            text: t('home.notifications.yes').value,
            onClick: async () => {
              try {
                await invoke('create_desktop_shortcut')
                window.showNotification?.(t('home.notifications.shortcutCreated').value, 3000)
              } catch (error) {
                console.error('Failed to create shortcut:', error)
                window.showNotification?.(t('home.notifications.shortcutFailed').value, 3000)
              }
            }
          },
          {
            text: t('home.notifications.no').value,
            onClick: () => {
              // 用户选择"否"，不做任何操作
            }
          },
          {
            text: t('home.notifications.noRemind').value,
            onClick: async () => {
              try {
                await invoke('save_skip_shortcut_prompt')
              } catch (error) {
                console.error('Failed to save skip prompt setting:', error)
              }
            }
          }
        ]
      )
    }
  } catch (error) {
    console.error('Failed to check desktop shortcut:', error)
  }
}

// 检查订阅更新
const checkSubscriptionUpdates = async () => {
  try {
    // 获取订阅信息（可能为null）
    const subscription = await invoke<any>('get_subscription')
    
    // 获取远程版本信息
    const remoteData = await invoke<any>('get_remote_versions')
    const latestVersions = remoteData['latest-version']
    if (!latestVersions || !Array.isArray(latestVersions)) return
    
    // 获取当前版本
    const frontendVersion = await invoke<string>('get_frontend_version')
    const backendVersion = await invoke<any>('get_backend_version')
    
    const updates: string[] = []
    
    // 比较版本号函数
    const compareVersions = (v1: string, v2: string): number => {
      const parts1 = v1.split('.').map(Number)
      const parts2 = v2.split('.').map(Number)
      const maxLen = Math.max(parts1.length, parts2.length)
      for (let i = 0; i < maxLen; i++) {
        const p1 = parts1[i] || 0
        const p2 = parts2[i] || 0
        if (p1 > p2) return 1
        if (p1 < p2) return -1
      }
      return 0
    }
    
    // 检查前端更新
    // 规则：没订阅 → 比对stable；订阅了 → 比对订阅的渠道
    const frontendChannel = subscription?.frontend?.channel || 'stable'
    const frontendChannelItem = latestVersions.find((item: any) => item[frontendChannel])
    if (frontendChannelItem && frontendChannelItem[frontendChannel]) {
      const remoteVersion = frontendChannelItem[frontendChannel].frontend?.version
      if (remoteVersion && compareVersions(remoteVersion, frontendVersion) > 0) {
        const channelText = frontendChannel === 'stable' 
          ? t('home.notifications.stable').value 
          : t('home.notifications.beta').value
        updates.push(`${t('home.notifications.frontend').value} ${channelText} ${remoteVersion}`)
      }
    }
    
    // 检查后端更新
    // 规则：没订阅 → 比对stable；订阅了 → 比对订阅的渠道
    const backendChannel = subscription?.backend?.channel || 'stable'
    const backendChannelItem = latestVersions.find((item: any) => item[backendChannel])
    if (backendChannelItem && backendChannelItem[backendChannel]) {
      const remoteVersion = backendChannelItem[backendChannel].backend?.version
      if (remoteVersion && compareVersions(remoteVersion, backendVersion.version) > 0) {
        const channelText = backendChannel === 'stable' 
          ? t('home.notifications.stable').value 
          : t('home.notifications.beta').value
        updates.push(`${t('home.notifications.backend').value} ${channelText} ${remoteVersion}`)
      }
    }
    
    // 如果有更新，显示常驻通知
    if (updates.length > 0) {
      // 判断是否有订阅
      const hasSubscription = subscription?.frontend || subscription?.backend
      const updatesStr = updates.join('、')
      const notificationText = hasSubscription 
        ? t('home.notifications.foundSubscriptionUpdate').value.replace('{updates}', updatesStr)
        : t('home.notifications.foundNewVersion').value.replace('{updates}', updatesStr)
      
      window.showPersistentNotification?.(
        notificationText,
        [
          {
            text: t('home.notifications.updateNow').value,
            onClick: () => {
              // 跳转到版本更新页面
              window.open('/version-update', '_blank')
            }
          },
          {
            text: t('home.notifications.remindLater').value,
            onClick: () => {
              window.logToConsole?.('前端', 'INFO', t('home.notifications.userDelayedUpdate').value)
            }
          }
        ]
      )
      await window.logToConsole?.('前端', 'INFO', `${hasSubscription ? t('home.notifications.foundSubscriptionUpdate').value.replace('{updates}', '') : t('home.notifications.foundNewVersion').value.replace('{updates}', '')}${updates.join(', ')}`)
    }
  } catch (error) {
    console.error('Failed to check subscription updates:', error)
  }
}

// 从 package.json 获取版本号
onMounted(async () => {
  // 加载默认背景
  await loadDefaultBackground()
  
  // 获取前端版本
  try {
    const version = await invoke<string>('get_frontend_version')
    appVersion.value = version
  } catch (error) {
    console.error('Failed to load app version:', error)
  }
  
  // 检查桌面快捷方式
  await checkDesktopShortcut()
  
  // 检查订阅更新
  await checkSubscriptionUpdates()
  
  // 全局监听任务状态变化
  unlistenStatusChange = await listen<string>("sra-status-changed", (event) => {
    const newStatus = event.payload as any;
    
    // 如果之前在执行任务，现在变成running，说明任务完成了
    if (newStatus === "running" && isExecutingTask.value) {
      window.showNotification?.(t('home.notifications.taskCompleted').value, 3000);
      isExecutingTask.value = false;
    } else if (newStatus === "task-running") {
      isExecutingTask.value = true;
    } else if (newStatus === "not-running") {
      isExecutingTask.value = false;
    }
  });
})

onUnmounted(() => {
  if (unlistenStatusChange) {
    unlistenStatusChange();
  }
})
</script>

<style scoped>
.app-container {
  display: grid;
  grid-template-columns: 280px 1fr;
  height: 100vh;
  min-width: 1000px;
  min-height: 600px;
  /* 默认背景，优先级较低 */
  background: var(--default-background, #1a1a2e) no-repeat center center;
  background-size: cover;
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  padding: 0;
  gap: 0;
}

.sidebar {
  background: var(--bg-secondary);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  margin: 15px 0 15px 15px;
}

.sidebar-content {
  padding: 20px;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.app-info {
  text-align: center;
  margin-bottom: 30px;
}

.app-icon {
  width: 80px;
  height: 80px;
  border-radius: 10px;
  margin-bottom: 10px;
  pointer-events: none;
  -webkit-user-drag: none;
}

.app-name {
  margin: 0;
  font-size: 18px;
  font-weight: bold;
  color: var(--text-color);
}

.app-version {
  margin: 5px 0 0 0;
  font-size: 12px;
  color: var(--text-secondary);
}

.nav-menu {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-radius: 8px;
  text-decoration: none;
  color: var(--text-color);
  transition: all 0.2s ease;
  background: var(--bg-tertiary);
  position: relative;
}

.nav-item:hover {
  background: var(--input-bg);
  transform: scale(1.05);
}

.nav-item:active {
  transform: scale(0.98) translateY(2px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.nav-item.active {
  background: rgba(255, 255, 255, 0.25);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  color: rgba(255, 255, 255, 1);
  font-weight: 600;
}

.nav-item.active:active {
  transform: scale(0.98) translateY(2px);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
}

/* 浅色模式下的选中效果 */
@media (prefers-color-scheme: light) {
  .nav-item.active {
    background: rgba(0, 0, 0, 0.12);
    color: rgba(0, 0, 0, 0.95);
    font-weight: 600;
  }
}

.nav-icon {
  width: 20px;
  height: 20px;
  margin-right: 12px;
  color: var(--text-color);
}

.nav-text {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-color);
}

.main-wrapper {
  overflow: hidden;
  padding: 0;
  box-sizing: border-box;
}

.main-content {
  height: 100%;
  background: transparent;
  overflow: hidden;
  border-radius: 12px;
}

/* 所有深色模式样式现在通过CSS变量自动处理 */
</style>