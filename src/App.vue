<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRoute } from 'vue-router'
import Main from './views/Main.vue'
import { setLocale } from './locales'
import { useTranslation } from './composables/useTranslation'

const { t } = useTranslation()
const route = useRoute()

// 判断是否是独立窗口路由
const isStandaloneWindow = computed(() => {
  return route.path === '/announcement-window' || route.path === '/version-update'
})

interface AppSettings {
  language: number
  [key: string]: any
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
  }
}

// 加载壁纸
const loadWallpaper = async () => {
  try {
    window.logToConsole?.('前端', 'DEBUG', t('console.logs.startGetWallpaper').value)
    const base64Data = await invoke<string | null>('get_wallpaper_base64')
    const result = base64Data ? t('console.logs.hasData').value : t('console.logs.noData').value
    window.logToConsole?.('前端', 'DEBUG', t('console.logs.getWallpaperResult', { result }).value)
    
    if (base64Data) {
      window.logToConsole?.('前端', 'DEBUG', t('console.logs.wallpaperDataLength', { length: base64Data.length }).value)
      // 立即应用壁纸
      applyWallpaperToBackground(base64Data)
      window.logToConsole?.('前端', 'INFO', t('console.logs.loadWallpaperSuccess').value)
    } else {
      window.logToConsole?.('前端', 'DEBUG', t('console.logs.noCustomWallpaper').value)
    }
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : t('settings.notifications.unknownError').value
    window.logToConsole?.('前端', 'ERR', t('console.logs.loadWallpaperFailed', { error: errorMsg }).value)
  }
}

// 初始化语言
const initLanguage = async () => {
  try {
    const settings = await invoke<AppSettings>('load_app_settings')
    setLocale(settings.language === 0 ? 'en-US' : 'zh-CN')
  } catch (error) {
    // 如果加载失败，使用默认语言（中文）
    setLocale('zh-CN')
  }
}

onMounted(async () => {
  await initLanguage()
  loadWallpaper()
})
</script>

<template>
  <Main v-if="!isStandaloneWindow" />
  <router-view v-else />
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>