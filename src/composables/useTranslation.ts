import { computed, unref } from 'vue'
import { useI18n } from '../locales'

// 简化的翻译 hook
export function useTranslation() {
  const { t: tComputed, locale, setLocale } = useI18n()
  
  // 创建一个简单的 t 函数，返回响应式的字符串
  const t = (key: string, params?: Record<string, any>) => {
    const computedValue = tComputed(key, params)
    // 返回 computed 的值，这样在模板中可以直接使用
    return computed(() => unref(computedValue))
  }
  
  return {
    t,
    locale,
    setLocale
  }
}
