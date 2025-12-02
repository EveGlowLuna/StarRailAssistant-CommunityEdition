import { ref } from 'vue'

interface Notification {
  id: number
  message: string
  persistent: boolean
  duration: number
  paused?: boolean
  remainingTime?: number
  timeoutId?: number
}

class NotificationManager {
  private notifications = ref<Notification[]>([])
  private notificationId = 0

  getNotifications() {
    return this.notifications
  }

  addNotification(message: string, persistent: boolean = false, duration: number = 5000) {
    const id = ++this.notificationId
    const notification: Notification = {
      id,
      message,
      persistent,
      duration,
      remainingTime: duration,
      paused: false
    }

    this.notifications.value.push(notification)

    // 限制最多3个通知
    if (this.notifications.value.length > 3) {
      const removed = this.notifications.value.shift()
      if (removed?.timeoutId) {
        clearTimeout(removed.timeoutId)
      }
    }

    if (!persistent) {
      this.startTimer(notification)
    }

    return id
  }

  private startTimer(notification: Notification) {
    if (notification.persistent || notification.paused) return

    notification.timeoutId = setTimeout(() => {
      this.removeNotification(notification.id)
    }, notification.remainingTime)
  }

  private stopTimer(notification: Notification) {
    if (notification.timeoutId) {
      clearTimeout(notification.timeoutId)
      notification.timeoutId = undefined
    }
  }

  pauseTimer(id: number) {
    const notification = this.notifications.value.find(n => n.id === id)
    if (notification && !notification.persistent) {
      notification.paused = true
      notification.remainingTime = notification.remainingTime || notification.duration
      this.stopTimer(notification)
    }
  }

  resumeTimer(id: number) {
    const notification = this.notifications.value.find(n => n.id === id)
    if (notification && !notification.persistent) {
      notification.paused = false
      this.startTimer(notification)
    }
  }

  removeNotification(id: number) {
    const index = this.notifications.value.findIndex(n => n.id === id)
    if (index > -1) {
      const notification = this.notifications.value[index]
      this.stopTimer(notification)
      this.notifications.value.splice(index, 1)
    }
  }

  clearAll() {
    this.notifications.value.forEach(notification => {
      this.stopTimer(notification)
    })
    this.notifications.value = []
  }
}

const notificationManager = new NotificationManager()

export const useNotifications = () => {
  return {
    notifications: notificationManager.getNotifications(),
    addNotification: notificationManager.addNotification.bind(notificationManager),
    removeNotification: notificationManager.removeNotification.bind(notificationManager),
    pauseTimer: notificationManager.pauseTimer.bind(notificationManager),
    resumeTimer: notificationManager.resumeTimer.bind(notificationManager),
    clearAll: notificationManager.clearAll.bind(notificationManager)
  }
}

// 全局通知函数，供所有组件和 Rust 后端调用
declare global {
  interface Window {
    showNotification: (message: string, persistent?: boolean, duration?: number) => number
    removeNotification: (id: number) => void
  }
}

// 在应用启动时挂载全局函数
export const setupGlobalNotifications = () => {
  const { addNotification, removeNotification } = useNotifications()

  window.showNotification = addNotification
  window.removeNotification = removeNotification
}