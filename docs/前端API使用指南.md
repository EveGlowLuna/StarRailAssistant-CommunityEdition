# 前端 API 使用指南

本文档介绍 SRA-CE 前端可用的全局 API，包括通知系统和日志系统。

---

## 📢 通知系统 API

通知系统用于向用户显示消息提示，支持临时通知和常驻通知两种模式。

### `window.showNotification()`

显示一个临时通知消息（自动消失）。

#### 函数签名

```typescript
window.showNotification(
  message: string,
  duration?: number
): number
```

#### 参数说明

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `message` | `string` | ✅ | - | 通知消息内容 |
| `duration` | `number` | ❌ | `5000` | 通知显示时长（毫秒） |

#### 返回值

返回通知的 ID（`number`），可用于手动关闭通知。

#### 特性

- ✅ 自动消失
- ✅ 最多同时显示 5 个临时通知
- ✅ 支持鼠标悬停暂停倒计时
- ✅ 可手动关闭（悬停时显示关闭按钮）
- ✅ 自动适配深色/浅色模式

#### 注意事项

1. 使用可选链操作符 `?.` 确保 API 可用
2. 通知会自动排队，超过 5 个时最旧的会被移除
3. 通知内容应简洁明了，避免过长
4. 临时通知适合大多数场景（操作反馈、状态提示等）

---

### `window.showPersistentNotification()`

显示一个常驻通知消息（带自定义按钮，需要用户操作）。

#### 函数签名

```typescript
window.showPersistentNotification(
  message: string,
  buttons: NotificationButton[]
): number

interface NotificationButton {
  text: string
  onClick: () => void
}
```

#### 参数说明

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `message` | `string` | ✅ | 通知消息内容 |
| `buttons` | `NotificationButton[]` | ✅ | 按钮数组，每个按钮包含文本和点击回调 |

#### 返回值

返回通知的 ID（`number`）。

#### 特性

- ✅ 不会自动消失，需要用户点击按钮
- ✅ 支持自定义多个按钮
- ✅ 点击按钮后自动关闭通知
- ✅ 不计入临时通知的数量限制
- ✅ 不会强制临时通知被移除
- ✅ 自动适配深色/浅色模式

#### 注意事项

1. 使用可选链操作符 `?.` 确保 API 可用
2. 常驻通知应谨慎使用，避免干扰用户
3. 按钮文本应简洁明了
4. 点击任何按钮后通知会自动关闭
5. 适合需要用户确认或选择的场景

### `window.removeNotification()`

手动关闭指定的通知。

#### 函数签名

```typescript
window.removeNotification(id: number): void
```

#### 参数说明

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | `number` | ✅ | 通知 ID（由 `showNotification` 或 `showPersistentNotification` 返回） |

---

## 📝 日志系统 API

日志系统用于记录应用运行时的各种信息，支持多种日志级别和来源。

### `window.logToConsole()`

向控制台记录一条日志。

#### 函数签名

```typescript
window.logToConsole(
  source: MessageSource,
  level: MessageLevel,
  message: string
): Promise<void>
```

#### 参数说明

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `source` | `MessageSource` | ✅ | 日志来源：`'前端'` \| `'后端'` \| `'进程端'` |
| `level` | `MessageLevel` | ✅ | 日志级别（见下表） |
| `message` | `string` | ✅ | 日志消息内容 |

#### 日志级别 (MessageLevel)

| 级别 | 说明 | 颜色 | 使用场景 |
|------|------|------|----------|
| `'INFO'` | 信息 | 白色 | 一般信息提示 |
| `'WARN'` | 警告 | 黄色 | 警告信息，不影响运行 |
| `'ERR'` | 错误 | 红色 | 错误信息，需要关注 |
| `'DEBUG'` | 调试 | 蓝色 | 调试信息，开发时使用 |
| `'TRACE'` | 追踪 | 紫色 | 详细追踪信息 |
| `'SUCCESS'` | 成功 | 绿色 | 操作成功提示 |
| `'MSG'` | 消息 | 灰色 | 用户输入/输出消息 |

#### 使用示例

```typescript
// 记录信息日志
await window.logToConsole?.('前端', 'INFO', '应用已启动')

// 记录警告日志
await window.logToConsole?.('前端', 'WARN', '配置文件不存在，使用默认配置')

// 记录错误日志
await window.logToConsole?.('前端', 'ERR', '网络请求失败: ' + error.message)

// 记录调试日志
await window.logToConsole?.('前端', 'DEBUG', '当前状态: ' + JSON.stringify(state))

// 记录成功日志
await window.logToConsole?.('前端', 'SUCCESS', '数据加载成功')
```

#### 实际应用示例

```typescript
// 在 Vue 组件中使用
const saveConfig = async () => {
  try {
    await window.logToConsole?.('前端', 'INFO', '开始保存配置')
    
    await invoke('save_config', { config: config.value })
    
    await window.logToConsole?.('前端', 'SUCCESS', '配置保存成功')
    window.showNotification?.('配置保存成功', false, 3000)  // 临时通知
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : '未知错误'
    await window.logToConsole?.('前端', 'ERR', `配置保存失败: ${errorMsg}`)
    window.showNotification?.('配置保存失败', false, 3000)  // 临时通知
  }
}
```

#### 特性

- ✅ 自动添加时间戳
- ✅ 支持日志级别过滤
- ✅ 支持来源过滤
- ✅ 自动持久化到文件
- ✅ 支持导出日志
- ✅ 实时显示在控制台页面

#### 注意事项

1. 使用可选链操作符 `?.` 确保 API 可用
2. 日志会自动保存到文件：`%APPDATA%/SRA/SRA-CE-Logs/`
3. 避免在循环中频繁记录日志，可能影响性能
4. 敏感信息（密码、密钥等）不应记录到日志

---

## 🔧 错误处理最佳实践

### 全局错误捕获

在 `main.ts` 中已配置全局错误捕获：

```typescript
// 全局错误处理
window.addEventListener('error', (event) => {
  window.logToConsole?.('前端', 'ERR', `全局错误: ${event.message}`)
})

// 全局未处理的 Promise 拒绝
window.addEventListener('unhandledrejection', (event) => {
  window.logToConsole?.('前端', 'ERR', `未处理的 Promise 拒绝: ${event.reason}`)
})
```

---

## 📊 日志查看

### 控制台页面

用户可以在"控制台"页面查看所有日志：

1. **过滤功能**
   - 按日志级别过滤（INFO/WARN/ERR/DEBUG/TRACE/SUCCESS/MSG）
   - 按来源过滤（前端/后端/进程端）
   - 过滤状态会自动保存

2. **导出功能**
   - 点击"导出日志"按钮
   - 选择保存位置
   - 导出为 `.txt` 文本文件

3. **清空功能**
   - 点击"清空"按钮清空当前显示的日志
   - 不影响已保存到文件的日志

### 日志文件位置

日志文件自动保存在：
```
Windows: C:\Users\<用户名>\AppData\Roaming\SRA\SRA-CE-Logs\
文件名格式: log-YYYY-MM-DD_HH-MM-SS.log
```

---

## 💡 使用建议

### 何时使用临时通知

✅ **适合使用临时通知的场景：**
- 用户操作的即时反馈（保存、删除、创建等）
- 操作成功/失败的确认
- 一般的状态提示

❌ **不适合使用临时通知的场景：**
- 频繁的状态更新
- 调试信息
- 详细的错误堆栈

### 何时使用常驻通知

✅ **适合使用常驻通知的场景：**
- 需要用户确认的重要操作
- 需要用户选择的场景（如版本更新）
- 任务完成后需要用户查看结果
- 需要用户注意的重要提示

❌ **不适合使用常驻通知的场景：**
- 一般的操作反馈（使用临时通知）
- 频繁出现的提示
- 不需要用户操作的信息

### 何时使用日志

✅ **适合记录日志的场景：**
- 所有用户操作
- 重要的状态变更
- 错误和异常
- 调试信息
- 性能关键点

❌ **不适合记录日志的场景：**
- 高频率的事件（如鼠标移动）
- 敏感信息（密码、密钥）
- 过于详细的内部状态

### 组合使用示例

```typescript
const deleteConfig = async (configName: string) => {
  // 检查是否为默认配置
  if (configName === 'Default') {
    // 只显示通知，不记录日志（这是预期的用户行为）
    window.showNotification?.('默认配置不能删除', 3000)
    return
  }
  
  try {
    // 记录操作开始
    await window.logToConsole?.('前端', 'INFO', `开始删除配置: ${configName}`)
    
    // 执行删除
    await invoke('delete_config', { name: configName })
    
    // 记录成功并通知用户
    await window.logToConsole?.('前端', 'SUCCESS', `配置删除成功: ${configName}`)
    window.showNotification?.('配置删除成功', 3000)
  } catch (error) {
    // 记录错误并通知用户
    const errorMsg = error instanceof Error ? error.message : '未知错误'
    await window.logToConsole?.('前端', 'ERR', `配置删除失败: ${errorMsg}`)
    window.showNotification?.('配置删除失败', 3000)
  }
}

// 使用常驻通知的示例
const checkForUpdates = async () => {
  try {
    const updateInfo = await invoke('check_updates')
    
    if (updateInfo.hasUpdate) {
      // 显示常驻通知，等待用户操作
      window.showPersistentNotification?.(
        `发现新版本 ${updateInfo.version}，是否立即更新？`,
        [
          {
            text: '立即更新',
            onClick: async () => {
              await window.logToConsole?.('前端', 'INFO', '用户选择立即更新')
              await invoke('start_update')
              window.showNotification?.('开始下载更新...', 3000)
            }
          },
          {
            text: '稍后提醒',
            onClick: () => {
              window.logToConsole?.('前端', 'INFO', '用户选择稍后更新')
            }
          }
        ]
      )
    } else {
      window.showNotification?.('当前已是最新版本', 3000)
    }
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : '未知错误'
    await window.logToConsole?.('前端', 'ERR', `检查更新失败: ${errorMsg}`)
    window.showNotification?.('检查更新失败', 3000)
  }
}
```

---

## 🔍 TypeScript 类型定义

如果需要在 TypeScript 中使用这些 API，可以参考以下类型定义：

```typescript
// 消息来源
type MessageSource = '前端' | '后端' | '进程端'

// 消息级别
type MessageLevel = 'INFO' | 'WARN' | 'ERR' | 'DEBUG' | 'TRACE' | 'SUCCESS' | 'MSG'

// 通知按钮
interface NotificationButton {
  text: string
  onClick: () => void
}

// 全局 API
interface Window {
  showNotification?: (message: string, duration?: number) => number
  showPersistentNotification?: (message: string, buttons: NotificationButton[]) => number
  removeNotification?: (id: number) => void
  logToConsole?: (source: MessageSource, level: MessageLevel, message: string) => Promise<void>
}
```

这些类型定义已在 `src/composables/useNotifications.ts` 和 `src/composables/useConsoleMessages.ts` 中声明。

---

## 📅 更新日期

2024-11-29

---

## 📝 相关文档

- [功能实现情况](./功能实现情况.md)
- [日志系统实现说明](./日志系统实现说明.md)
- [SRA日志解析器说明](./SRA日志解析器说明.md)
