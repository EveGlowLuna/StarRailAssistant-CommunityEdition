import { ref } from 'vue'

export interface NotificationButton {
  text: string
  onClick: () => void
}

interface Notification {
  id: number
  message: string
  type: 'temporary' | 'persistent'
  duration: number
  paused?: boolean
  remainingTime?: number
  timeoutId?: number
  buttons?: NotificationButton[]
}

class NotificationManager {
  private notifications = ref<Notification[]>([])
  private notificationId = 0

  getNotifications() {
    return this.notifications
  }

  addNotification(message: string, duration: number = 5000) {
    const id = ++this.notificationId
    const notification: Notification = {
      id,
      message,
      type: 'temporary',
      duration,
      remainingTime: duration,
      paused: false
    }

    this.notifications.value.push(notification)

    // 限制最多5个临时通知（不包括常驻通知）
    const temporaryNotifications = this.notifications.value.filter(n => n.type === 'temporary')
    if (temporaryNotifications.length > 5) {
      const removed = temporaryNotifications[0]
      const index = this.notifications.value.findIndex(n => n.id === removed.id)
      if (index > -1) {
        if (removed.timeoutId) {
          clearTimeout(removed.timeoutId)
        }
        this.notifications.value.splice(index, 1)
      }
    }

    this.startTimer(notification)

    return id
  }

  addPersistentNotification(message: string, buttons: NotificationButton[]) {
    const id = ++this.notificationId
    const notification: Notification = {
      id,
      message,
      type: 'persistent',
      duration: 0,
      buttons
    }

    this.notifications.value.push(notification)

    return id
  }

  private startTimer(notification: Notification) {
    if (notification.type === 'persistent' || notification.paused) return

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
    if (notification && notification.type === 'temporary') {
      notification.paused = true
      notification.remainingTime = notification.remainingTime || notification.duration
      this.stopTimer(notification)
    }
  }

  resumeTimer(id: number) {
    const notification = this.notifications.value.find(n => n.id === id)
    if (notification && notification.type === 'temporary') {
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
    addPersistentNotification: notificationManager.addPersistentNotification.bind(notificationManager),
    removeNotification: notificationManager.removeNotification.bind(notificationManager),
    pauseTimer: notificationManager.pauseTimer.bind(notificationManager),
    resumeTimer: notificationManager.resumeTimer.bind(notificationManager),
    clearAll: notificationManager.clearAll.bind(notificationManager)
  }
}

// 全局通知函数，供所有组件和 Rust 后端调用
declare global {
  interface Window {
    showNotification: (message: string, duration?: number) => number
    showPersistentNotification: (message: string, buttons: NotificationButton[]) => number
    removeNotification: (id: number) => void
  }
}

// 在应用启动时挂载全局函数
export const setupGlobalNotifications = () => {
  const { addNotification, addPersistentNotification, removeNotification } = useNotifications()

  window.showNotification = addNotification
  window.showPersistentNotification = addPersistentNotification
  window.removeNotification = removeNotification
}