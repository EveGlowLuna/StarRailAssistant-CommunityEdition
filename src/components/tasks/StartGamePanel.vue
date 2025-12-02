<template>
  <div class="start-game-panel">
    <div class="panel-section">
      <div class="section-header">
        <input
          type="checkbox"
          id="enable-start-game"
          v-model="modelValue.EnabledTasks[0]"
          class="section-checkbox"
        />
        <label for="enable-start-game" class="section-title">{{ t('tasks.startGame.title') }}</label>
      </div>

      <div class="section-content">
        <div class="form-group">
          <label class="form-label">{{ t('tasks.startGame.channel') }}</label>
          <CustomDropdown
            v-model="modelValue.StartGameChannel"
            :options="channelOptions"
            placeholder=""
          />
        </div>

        <div class="form-group">
          <label class="form-label">{{ t('tasks.startGame.gamePath') }}</label>
          <div class="path-input-group">
            <input
              v-model="modelValue.StartGamePath"
              type="text"
              class="form-input"
              placeholder=""
              readonly
            />
            <button class="browse-button" @click="browsePath">
              <FolderOpen :size="16" />
              {{ t('tasks.startGame.selectPath') }}
            </button>
          </div>
        </div>

        <div class="form-group checkbox-group">
          <input
            type="checkbox"
            id="auto-login"
            v-model="modelValue.StartGameAutoLogin"
            class="form-checkbox"
          />
          <label for="auto-login" class="checkbox-label">
            {{ t('tasks.startGame.autoLogin') }}
            <span class="hint-text">{{ t('tasks.startGame.autoLoginHint') }}</span>
          </label>
        </div>

        <div class="form-group checkbox-group">
          <input
            type="checkbox"
            id="always-login"
            v-model="modelValue.StartGameAlwaysLogin"
            class="form-checkbox"
          />
          <label for="always-login" class="checkbox-label">{{ t('tasks.startGame.alwaysLogin') }}</label>
        </div>

        <div class="form-group">
          <label class="form-label">{{ t('tasks.startGame.username') }}</label>
          <input
            v-model="modelValue.StartGameUsername"
            type="text"
            class="form-input"
            placeholder=""
          />
        </div>

        <div class="form-group">
          <label class="form-label">{{ t('tasks.startGame.password') }}</label>
          <input
            v-model="modelValue.StartGamePassword"
            type="password"
            class="form-input"
            placeholder=""
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { FolderOpen } from 'lucide-vue-next'
import CustomDropdown from '../CustomDropdown.vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useTranslation } from '../../composables/useTranslation'

const { t } = useTranslation()

const props = defineProps<{
  modelValue: any
}>()

const channelOptions = computed(() => [
  { label: t('tasks.startGame.officialServer').value, value: 0 },
  { label: t('tasks.startGame.bilibiliServer').value, value: 1 }
])

const browsePath = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: '可执行文件',
        extensions: ['exe']
      }],
      title: '选择游戏可执行文件'
    })
    
    if (selected) {
      props.modelValue.StartGamePath = selected
    }
  } catch (error) {
    console.error('Failed to select path:', error)
  }
}
</script>

<style scoped>
.start-game-panel {
  /* 移除固定高度，让内容自然撑开 */
}

.panel-section {
  background: rgba(255, 255, 255, 0.5);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 6px;
  padding: 16px;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 16px;
}

.section-checkbox {
  width: 18px;
  height: 18px;
  cursor: pointer;
  accent-color: #007bff;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: #000;
  cursor: pointer;
}

.section-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding-left: 26px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-label {
  font-size: 14px;
  font-weight: 500;
  color: #000;
}

.form-input {
  padding: 8px 12px;
  border: 1px solid rgba(0, 0, 0, 0.2);
  border-radius: 4px;
  font-size: 14px;
  background: rgba(255, 255, 255, 0.9);
  transition: border-color 0.3s ease;
}

.form-input:focus {
  outline: none;
  border-color: #007bff;
}

.path-input-group {
  display: flex;
  gap: 8px;
}

.path-input-group .form-input {
  flex: 1;
}

.browse-button {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  background: rgba(33, 150, 243, 0.8);
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  white-space: nowrap;
}

.browse-button:hover {
  background: rgba(33, 150, 243, 0.9);
}

.checkbox-group {
  flex-direction: row;
  align-items: flex-start;
  gap: 8px;
}

.form-checkbox {
  width: 16px;
  height: 16px;
  margin-top: 2px;
  cursor: pointer;
  accent-color: #007bff;
  flex-shrink: 0;
}

.checkbox-label {
  font-size: 14px;
  color: #000;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.hint-text {
  font-size: 12px;
  color: #666;
  font-weight: normal;
}

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .panel-section {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .section-title,
  .form-label,
  .checkbox-label {
    color: #fff;
  }

  .form-input {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.3);
    color: #fff;
  }

  .hint-text {
    color: #ccc;
  }
}
</style>
