// 配置验证工具

import { invoke } from '@tauri-apps/api/core'

interface TaskConfig {
  Name: string
  CurrencyWarsEnable?: boolean
  CurrencyWarsUsername?: string
  [key: string]: any
}

/**
 * 验证单个配置的货币战争字段
 * @param config 配置对象
 * @returns 验证结果，如果通过返回null，否则返回错误信息
 */
export function validateCurrencyWars(config: TaskConfig): string | null {
  // 如果货币战争未启用，跳过验证
  if (!config.CurrencyWarsEnable) {
    return null
  }
  
  // 如果启用了货币战争，检查用户名
  if (!config.CurrencyWarsUsername || config.CurrencyWarsUsername.trim() === '') {
    return `配置"${config.Name}"的货币战争功能已启用，但开拓者名称为空，这是必填字段`
  }
  
  return null
}

/**
 * 验证多个配置
 * @param configNames 配置名称列表
 * @returns 验证结果，如果通过返回null，否则返回错误信息
 */
export async function validateConfigs(configNames: string[]): Promise<string | null> {
  for (const configName of configNames) {
    try {
      const config = await invoke<TaskConfig>('load_config', { name: configName })
      const error = validateCurrencyWars(config)
      if (error) {
        return error
      }
    } catch (err) {
      return `无法加载配置"${configName}"`
    }
  }
  return null
}
