import { ref, computed } from 'vue'
import zhCN from './zh-CN'
import enUS from './en-US'

type Language = 'zh-CN' | 'en-US'

const messages = {
  'zh-CN': zhCN,
  'en-US': enUS
}

// 当前语言
const currentLocale = ref<Language>('zh-CN')

// 获取嵌套对象的值
function getNestedValue(obj: any, path: string): any {
  return path.split('.').reduce((current, key) => current?.[key], obj)
}

// 翻译函数
export function t(key: string, params?: Record<string, any>): string {
  const message = getNestedValue(messages[currentLocale.value], key)
  
  if (!message) {
    console.warn(`Translation key not found: ${key}`)
    return key
  }
  
  // 如果有参数，替换占位符
  if (params) {
    return Object.keys(params).reduce((result, paramKey) => {
      return result.replace(`{${paramKey}}`, String(params[paramKey]))
    }, message)
  }
  
  return message
}

// 切换语言
export function setLocale(locale: Language) {
  currentLocale.value = locale
}

// 获取当前语言
export function getLocale() {
  return currentLocale.value
}

// 导出响应式的翻译函数（用于组件中）
export function useI18n() {
  return {
    t: (key: string, params?: Record<string, any>) => {
      // 使用 computed 确保响应式
      return computed(() => t(key, params))
    },
    locale: currentLocale,
    setLocale
  }
}

// 全局挂载
declare global {
  interface Window {
    $t: typeof t
  }
}

export function setupI18n() {
  window.$t = t
}
