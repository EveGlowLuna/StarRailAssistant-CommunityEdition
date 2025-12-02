<template>
  <div class="notification-container">
    <TransitionGroup name="notification" tag="div">
      <div
        v-for="notification in notifications"
        :key="notification.id"
        class="notification-item"
        :class="{ persistent: notification.persistent }"
        @mouseenter="handleMouseEnter(notification.id)"
        @mouseleave="handleMouseLeave(notification.id)"
      >
        <div class="notification-content">
          <div class="notification-text">{{ notification.message }}</div>
          <button
            v-if="notification.persistent || hoveredNotificationId === notification.id"
            class="notification-close"
            @click="removeNotification(notification.id)"
          >
            ×
          </button>
        </div>
        <div
          v-if="!notification.persistent && hoveredNotificationId !== notification.id"
          class="notification-progress"
          :style="{ animationDuration: `${notification.remainingTime || notification.duration}ms` }"
        ></div>
      </div>
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useNotifications } from '../composables/useNotifications'

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
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.1);
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
  color: #000;
  font-size: 14px;
  font-weight: 500;
  flex: 1;
}

.notification-close {
  background: none;
  border: none;
  color: #666;
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
  background: rgba(0, 0, 0, 0.1);
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
    background: rgba(0, 0, 0, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .notification-text {
    color: #fff;
  }

  .notification-close {
    color: #ccc;
  }

  .notification-close:hover {
    background: rgba(255, 255, 255, 0.1);
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