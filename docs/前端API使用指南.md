# 前端 API 使用指南

本文档介绍 SRA-CE 前端可用的全局 API，包括通知系统和日志系统。

---

## 📢 通知系统 API

通知系统用于向用户显示消息提示，支持临时通知和持久通知两种模式。

### `window.showNotification()`

显示一个通知消息。

#### 函数签名

```typescript
window.showNotification(
  message: string,
  persistent?: boolean,
  duration?: number
): number
```

#### 参数说明

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `message` | `string` | ✅ | - | 通知消息内容 |
| `persistent` | `boolean` | ❌ | `false` | `true` 表示持久通知（需手动关闭），`false` 表示临时通知（自动消失） |
| `duration` | `number` | ❌ | `5000` | 通知显示时长（毫秒），仅对临时通知有效 |

#### 返回值

返回通知的 ID（`number`），可用于手动关闭通知。

#### 使用示例

```typescript
// 临时通知（5秒后自动消失）
window.showNotification?.('配置保存成功', false, 5000)

// 临时通知（使用默认时长 5 秒）
window.showNotification?.('操作完成', false)

// 持久通知（需要用户手动关闭）
const notificationId = window.showNotification?.('重要提示：请检查配置', true)

// 手动关闭通知
window.removeNotification?.(notificationId)
```

#### 特性

- ✅ 支持临时通知和持久通知
- ✅ 自动消失（临时通知）
- ✅ 最多同时显示 3 个通知
- ✅ 支持鼠标悬停暂停倒计时
- ✅ 可手动关闭
- ✅ 自动适配深色/浅色模式

#### 注意事项

1. 使用可选链操作符 `?.` 确保 API 可用
2. 通知会自动排队，超过 3 个时最旧的会被移除
3. 通知内容应简洁明了，避免过长
4. 持久通知应谨慎使用，避免干扰用户
5. 临时通知适合大多数场景（操作反馈、状态提示等）

### `window.removeNotification()`

手动关闭指定的通知。

#### 函数签名

```typescript
window.removeNotification(id: number): void
```

#### 参数说明

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | `number` | ✅ | 通知 ID（由 `showNotification` 返回） |

#### 使用示例

```typescript
// 显示持久通知并保存 ID
const id = window.showNotification?.('正在处理...', true)

// 处理完成后手动关闭
setTimeout(() => {
  window.removeNotification?.(id)
  window.showNotification?.('处理完成', false, 3000)
}, 2000)
```

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

### 统一的错误处理模式

```typescript
const performAction = async () => {
  try {
    // 记录开始
    await window.logToConsole?.('前端', 'INFO', '开始执行操作')
    
    // 执行操作
    const result = await someAsyncOperation()
    
    // 记录成功
    await window.logToConsole?.('前端', 'SUCCESS', '操作执行成功')
    window.showNotification?.('操作成功', false, 3000)  // 临时通知
    
    return result
  } catch (error) {
    // 记录错误
    const errorMsg = error instanceof Error ? error.message : '未知错误'
    await window.logToConsole?.('前端', 'ERR', `操作失败: ${errorMsg}`)
    
    // 显示通知
    window.showNotification?.('操作失败', false, 3000)  // 临时通知
    
    // 可选：重新抛出错误
    throw error
  }
}
```

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

### 何时使用通知

✅ **适合使用通知的场景：**
- 用户操作的即时反馈（保存、删除、创建等）
- 重要的状态变更提示
- 操作成功/失败的确认

❌ **不适合使用通知的场景：**
- 频繁的状态更新
- 调试信息
- 详细的错误堆栈

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
    window.showNotification?.('默认配置不能删除', false, 3000)
    return
  }
  
  try {
    // 记录操作开始
    await window.logToConsole?.('前端', 'INFO', `开始删除配置: ${configName}`)
    
    // 执行删除
    await invoke('delete_config', { name: configName })
    
    // 记录成功并通知用户
    await window.logToConsole?.('前端', 'SUCCESS', `配置删除成功: ${configName}`)
    window.showNotification?.('配置删除成功', true, 3000)
  } catch (error) {
    // 记录错误并通知用户
    const errorMsg = error instanceof Error ? error.message : '未知错误'
    await window.logToConsole?.('前端', 'ERR', `配置删除失败: ${errorMsg}`)
    window.showNotification?.('配置删除失败', false, 3000)
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

// 全局 API
interface Window {
  showNotification?: (message: string, isSuccess: boolean, duration?: number) => void
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
