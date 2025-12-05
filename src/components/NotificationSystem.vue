<template>
  <div class="notification-container">
    <TransitionGroup name="notification" tag="div">
      <div
        v-for="notification in notifications"
        :key="notification.id"
        class="notification-item"
        :class="{ persistent: notification.type === 'persistent' }"
        @mouseenter="handleMouseEnter(notification.id)"
        @mouseleave="handleMouseLeave(notification.id)"
      >
        <div class="notification-content">
          <div class="notification-text">{{ notification.message }}</div>
          <button
            v-if="notification.type === 'temporary' && hoveredNotificationId === notification.id"
            class="notification-close"
            @click="removeNotification(notification.id)"
          >
            ×
          </button>
        </div>
        
        <!-- 常驻通知的按钮 -->
        <div v-if="notification.type === 'persistent' && notification.buttons" class="notification-buttons">
          <button
            v-for="(button, index) in notification.buttons"
            :key="index"
            class="notification-button"
            @click="handleButtonClick(notification.id, button)"
          >
            {{ button.text }}
          </button>
        </div>
        
        <!-- 临时通知的进度条 -->
        <div
          v-if="notification.type === 'temporary' && hoveredNotificationId !== notification.id"
          class="notification-progress"
          :style="{ animationDuration: `${notification.remainingTime || notification.duration}ms` }"
        ></div>
      </div>
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useNotifications, type NotificationButton } from '../composables/useNotifications'

const { notifications, removeNotification, pauseTimer, resumeTimer } = useNotifications()

const hoveredNotificationId = ref<number | null>(null)

const handleMouseEnter = (notificationId: number) => {
  hoveredNotificationId.value = notificationId
  pauseTimer(notificationId)
}

const handleMouseLeave = (notificationId: number) => {
  hoveredNotificationId.value = null
  resumeTimer(notificationId)
}

const handleButtonClick = (notificationId: number, button: NotificationButton) => {
  button.onClick()
  removeNotification(notificationId)
}
</script>

<style scoped>
.notification-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 1000;
  pointer-events: none;
}

.notification-item {
  margin-bottom: 10px;
  pointer-events: auto;
  border-radius: 8px;
  overflow: hidden;
  background: rgba(255, 255, 255, 0.15);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  min-width: 300px;
  max-width: 400px;
}

.notification-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
}

.notification-text {
  color: #fff;
  font-size: 14px;
  font-weight: 500;
  flex: 1;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

.notification-close {
  background: none;
  border: none;
  color: rgba(255, 255, 255, 0.8);
  font-size: 18px;
  cursor: pointer;
  padding: 0;
  margin-left: 8px;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: background-color 0.2s;
}

.notification-close:hover {
  background: rgba(255, 255, 255, 0.15);
}

.notification-buttons {
  display: flex;
  gap: 8px;
  padding: 0 16px 12px 16px;
  justify-content: flex-end;
}

.notification-button {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: #4CAF50;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.2s;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.notification-button:hover {
  background: rgba(76, 175, 80, 0.1);
}

.notification-button:active {
  background: rgba(76, 175, 80, 0.2);
}

.notification-progress {
  height: 3px;
  background: linear-gradient(to left, #4CAF50, #81C784);
  animation: progress linear;
}

@keyframes progress {
  from {
    width: 100%;
  }
  to {
    width: 0%;
  }
}

/* 深色模式样式 */
@media (prefers-color-scheme: dark) {
  .notification-item {
    background: rgba(0, 0, 0, 0.25);
    border: 1px solid rgba(255, 255, 255, 0.15);
  }

  .notification-text {
    color: #fff;
  }

  .notification-close {
    color: rgba(255, 255, 255, 0.8);
  }

  .notification-close:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .notification-button {
    color: #81C784;
  }

  .notification-button:hover {
    background: rgba(129, 199, 132, 0.15);
  }

  .notification-button:active {
    background: rgba(129, 199, 132, 0.25);
  }
}

/* 动画效果 */
.notification-enter-active {
  transition: all 0.4s ease-out;
}

.notification-leave-active {
  transition: all 0.3s ease-in;
  position: absolute;
  width: 100%;
}

.notification-enter-from {
  transform: translateX(100%);
  opacity: 0;
}

.notification-enter-to {
  transform: translateX(0);
  opacity: 1;
}

.notification-leave-from {
  transform: translateX(0);
  opacity: 1;
}

.notification-leave-to {
  transform: translateX(100%);
  opacity: 0;
}

/* 离开动画时隐藏进度条 */
.notification-leave-active .notification-progress {
  display: none;
}

/* 其他通知向上移动的动画 */
.notification-move {
  transition: all 0.5s ease-out;
}
</style>