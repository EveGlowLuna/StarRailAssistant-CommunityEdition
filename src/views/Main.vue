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
import { listen } from '@tauri-apps/api/event'
import { House, SquareCheck, Plug, SquareTerminal, Settings } from 'lucide-vue-next'
import NotificationSystem from '../components/NotificationSystem.vue'
import { useTranslation } from '../composables/useTranslation'

const { t } = useTranslation()

const route = useRoute()
const appVersion = ref('0.1.0') // 从 package.json 获取版本号
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

// 从 package.json 获取版本号
onMounted(async () => {
  // 加载默认背景
  await loadDefaultBackground()
  
  fetch('/package.json')
    .then(response => response.json())
    .then(data => {
      appVersion.value = data.version
    })
  
  // 全局监听任务状态变化
  unlistenStatusChange = await listen<string>("sra-status-changed", (event) => {
    const newStatus = event.payload as any;
    
    // 如果之前在执行任务，现在变成running，说明任务完成了
    if (newStatus === "running" && isExecutingTask.value) {
      window.showNotification?.(t('home.notifications.taskCompleted').value, false, 3000);
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
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.1);
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
  color: #000;
}

.app-version {
  margin: 5px 0 0 0;
  font-size: 12px;
  color: rgba(0, 0, 0, 0.7);
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
  color: #000;
  transition: all 0.3s ease;
  background: rgba(0, 0, 0, 0.05);
}

.nav-item:hover {
  background: rgba(0, 0, 0, 0.08);
  transform: scale(1.05);
}

.nav-item.active {
  background: rgba(0, 0, 0, 0.2);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.nav-icon {
  width: 20px;
  height: 20px;
  margin-right: 12px;
  color: #000;
}

.nav-text {
  font-size: 14px;
  font-weight: 500;
  color: #000;
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

/* 确保在暗色模式下也能正常显示 */
@media (prefers-color-scheme: dark) {
  .sidebar {
    background: rgba(0, 0, 0, 0.8);
    border-right: 1px solid rgba(255, 255, 255, 0.2);
  }

  .app-name {
    color: #fff;
  }

  .app-version {
    color: rgba(255, 255, 255, 0.7);
  }

  .nav-item {
    color: #fff;
    background: rgba(255, 255, 255, 0.1);
  }

  .nav-item:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .nav-item.active {
    background: rgba(255, 255, 255, 0.3);
  }

  .nav-icon,
  .nav-text {
    color: #fff;
  }

  .main-content {
    background: transparent;
    color: #fff;
  }
}
</style>