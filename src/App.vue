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

// 初始化应用设置
const initAppSettings = async () => {
  try {
    const settings = await invoke<AppSettings>('load_app_settings')
    // 设置语言 (0=中文, 1=英文)
    setLocale(settings.language === 0 ? 'zh-CN' : 'en-US')
  } catch (error) {
    // 如果加载失败，使用默认设置
    setLocale('zh-CN')
  }
}

onMounted(async () => {
  await initAppSettings()
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

  /* 浅色模式变量 */
  --text-color: #000;
  --text-secondary: #666;
  --bg-color: #f6f6f6;
  --bg-secondary: rgba(255, 255, 255, 0.8);
  --bg-tertiary: rgba(255, 255, 255, 0.9);
  --border-color: rgba(0, 0, 0, 0.1);
  --border-secondary: rgba(0, 0, 0, 0.2);
  --input-bg: #ffffff;
  --input-text: #0f0f0f;
  --button-active-bg: #e8e8e8;
  --scrollbar-track: rgba(0, 0, 0, 0.05);
  --scrollbar-thumb: rgba(0, 0, 0, 0.2);
  --scrollbar-thumb-hover: rgba(0, 0, 0, 0.3);

  color: var(--text-color);
  background-color: var(--bg-color);

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

/* 深色模式（跟随系统） */
@media (prefers-color-scheme: dark) {
  :root {
    --text-color: #fff;
    --text-secondary: #ccc;
    --bg-color: #2f2f2f;
    --bg-secondary: rgba(0, 0, 0, 0.8);
    --bg-tertiary: rgba(255, 255, 255, 0.1);
    --border-color: rgba(255, 255, 255, 0.2);
    --border-secondary: rgba(255, 255, 255, 0.2);
    --input-bg: rgba(255, 255, 255, 0.1);
    --input-text: #ffffff;
    --button-active-bg: #0f0f0f69;
    --scrollbar-track: rgba(255, 255, 255, 0.05);
    --scrollbar-thumb: rgba(255, 255, 255, 0.2);
    --scrollbar-thumb-hover: rgba(255, 255, 255, 0.3);
  }
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
  color: var(--input-text);
  background-color: var(--input-bg);
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
  background-color: var(--button-active-bg);
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

</style>