<template>
  <div class="extensions-container">
    <!-- 扩展标题 -->
    <div class="extensions-header">
      <h2 class="extensions-title">{{ t('extensions.title') }}</h2>
    </div>

    <!-- 扩展内容 -->
    <div class="extensions-content">
      <!-- 自动对话扩展 -->
      <div class="extension-card">
        <div class="card-header" @click="toggleCard('autoPlot')">
          <div class="header-left">
            <MessageSquare class="card-icon" :size="20" />
            <h3 class="card-title">{{ t('extensions.autoPlot.title') }}</h3>
          </div>
          <button class="expand-button">
            <ChevronDown v-if="expandedCards.autoPlot" :size="20" />
            <ChevronUp v-else :size="20" />
          </button>
        </div>
        
        <transition name="slide-fade">
          <div class="card-content" v-show="expandedCards.autoPlot">
            <div class="setting-row">
              <div class="setting-info">
                <label class="setting-label">{{ t('extensions.autoPlot.enable') }}</label>
                <p class="setting-description">{{ t('extensions.autoPlot.enableDesc') }}</p>
              </div>
              <label class="toggle-switch">
                <input
                  type="checkbox"
                  v-model="enableAutoPlot"
                  @change="onEnableAutoPlotChange"
                />
                <span class="toggle-slider"></span>
              </label>
            </div>

            <div class="setting-row">
              <div class="setting-info">
                <label class="setting-label">{{ t('extensions.autoPlot.skipPlot') }}</label>
                <p class="setting-description">{{ t('extensions.autoPlot.skipPlotDesc') }}</p>
              </div>
              <label class="toggle-switch">
                <input
                  type="checkbox"
                  v-model="skipPlot"
                  @change="onSkipPlotChange"
                />
                <span class="toggle-slider"></span>
              </label>
            </div>
          </div>
        </transition>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { MessageSquare, ChevronDown, ChevronUp } from 'lucide-vue-next';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from '../composables/useTranslation';

const { t } = useTranslation();

// 响应式数据
const expandedCards = ref({
  autoPlot: true,
});

const enableAutoPlot = ref(false);
const skipPlot = ref(false);

// 方法
const toggleCard = (cardName: keyof typeof expandedCards.value) => {
  expandedCards.value[cardName] = !expandedCards.value[cardName];
};

const onEnableAutoPlotChange = async () => {
  try {
    const command = enableAutoPlot.value
      ? 'trigger enable AutoPlotTrigger'
      : 'trigger disable AutoPlotTrigger';
    
    await invoke('send_input_to_sra', { input: command });
    
    // 显示通知
    if (window.showNotification) {
      window.showNotification(
        enableAutoPlot.value ? t('extensions.notifications.autoPlotEnabled').value : t('extensions.notifications.autoPlotDisabled').value,
        3000
      );
    }
  } catch (error) {
    console.error('Failed to toggle auto plot:', error);
    if (window.showNotification) {
      window.showNotification(t('extensions.notifications.toggleFailed').value, 3000);
    }
    // 回滚状态
    enableAutoPlot.value = !enableAutoPlot.value;
  }
};

const onSkipPlotChange = async () => {
  try {
    const command = `trigger set-bool AutoPlotTrigger skip_plot ${skipPlot.value}`;
    
    await invoke('send_input_to_sra', { input: command });
    
    // 显示通知
    if (window.showNotification) {
      window.showNotification(
        skipPlot.value ? t('extensions.notifications.skipPlotEnabled').value : t('extensions.notifications.skipPlotDisabled').value,
        3000
      );
    }
  } catch (error) {
    console.error('Failed to toggle skip plot:', error);
    if (window.showNotification) {
      window.showNotification(t('extensions.notifications.toggleFailed').value, 3000);
    }
    // 回滚状态
    skipPlot.value = !skipPlot.value;
  }
};
</script>

<style scoped>
.extensions-container {
  height: 100%;
  padding: 16px;
  overflow-y: auto;
}

/* 美化滚动条 */
.extensions-container::-webkit-scrollbar {
  width: 8px;
}

.extensions-container::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 4px;
}

.extensions-container::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
  transition: background-color 0.3s ease;
}

.extensions-container::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

.extensions-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding: 20px 24px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.extensions-title {
  margin: 0;
  font-size: 28px;
  font-weight: 600;
  color: #000;
}

.extensions-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding-bottom: 16px;
}

.extension-card {
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  border-left: 4px solid #2196f3;
}

.card-header {
  padding: 16px 20px;
  background: rgba(33, 150, 243, 0.1);
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: background-color 0.3s ease;
}

.card-header:hover {
  background: rgba(33, 150, 243, 0.15);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.card-icon {
  color: #2196f3;
}

.card-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #000;
}

.expand-button {
  background: none;
  border: none;
  color: #666;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.expand-button:hover {
  background: rgba(0, 0, 0, 0.1);
  color: #000;
}

/* 卡片折叠动画 */
.slide-fade-enter-active {
  transition: all 0.3s ease;
}

.slide-fade-leave-active {
  transition: all 0.2s ease;
}

.slide-fade-enter-from {
  opacity: 0;
  transform: translateY(-10px);
}

.slide-fade-enter-to {
  opacity: 1;
  transform: translateY(0);
}

.slide-fade-leave-from {
  opacity: 1;
  transform: translateY(0);
}

.slide-fade-leave-to {
  opacity: 0;
  transform: translateY(-5px);
}

.card-content {
  padding: 20px;
}

.setting-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 0;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.setting-row:last-child {
  border-bottom: none;
}

.setting-info {
  flex: 1;
}

.setting-label {
  display: block;
  font-size: 16px;
  font-weight: 500;
  color: #000;
  margin-bottom: 4px;
}

.setting-description {
  margin: 0;
  font-size: 14px;
  color: #666;
}

/* Toggle Switch 样式 */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 48px;
  height: 24px;
  cursor: pointer;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.2);
  border-radius: 24px;
  transition: all 0.3s ease;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  border-radius: 50%;
  transition: all 0.3s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle-switch input:checked + .toggle-slider {
  background-color: #2196f3;
}

.toggle-switch input:checked + .toggle-slider:before {
  transform: translateX(24px);
}

.toggle-switch:hover .toggle-slider {
  box-shadow: 0 0 0 4px rgba(33, 150, 243, 0.1);
}

/* 深色模式样式 */
@media (prefers-color-scheme: dark) {
  .extensions-container {
    color: #fff;
  }

  .extensions-header {
    background: rgba(0, 0, 0, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.2);
    padding: 20px 24px;
  }

  .extensions-title {
    color: #fff;
    font-size: 28px;
  }

  .extension-card {
    background: rgba(0, 0, 0, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .card-header {
    background: rgba(33, 150, 243, 0.2);
    border-bottom: 1px solid rgba(255, 255, 255, 0.2);
  }

  .card-header:hover {
    background: rgba(33, 150, 243, 0.3);
  }

  .card-title {
    color: #fff;
  }

  .expand-button {
    color: #ccc;
  }

  .expand-button:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
  }

  .setting-row {
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .setting-label {
    color: #fff;
  }

  .setting-description {
    color: #ccc;
  }

  .toggle-slider {
    background-color: rgba(255, 255, 255, 0.2);
  }

  .toggle-slider:before {
    background-color: #fff;
  }

  .toggle-switch input:checked + .toggle-slider {
    background-color: #2196f3;
  }
}
</style>
