<template>
  <div class="after-mission-panel">
    <div class="panel-section">
      <div class="section-header">
        <input
          type="checkbox"
          id="enable-after-mission"
          v-model="modelValue.EnabledTasks[4]"
          class="section-checkbox"
        />
        <label for="enable-after-mission" class="section-title">{{ t('tasks.afterMission.title') }}</label>
      </div>

      <div class="section-content">
        <div class="form-group checkbox-group">
          <input
            type="checkbox"
            id="after-logout"
            v-model="modelValue.AfterLogout"
            class="form-checkbox"
          />
          <label for="after-logout" class="checkbox-label">{{ t('tasks.afterMission.logout') }}</label>
        </div>

        <div class="form-group checkbox-group">
          <input
            type="checkbox"
            id="after-exit-game"
            v-model="modelValue.AfterExitGame"
            class="form-checkbox"
          />
          <label for="after-exit-game" class="checkbox-label">{{ t('tasks.afterMission.exitGame') }}</label>
        </div>

        <div class="form-group checkbox-group">
          <input
            type="checkbox"
            id="after-exit-app"
            v-model="modelValue.AfterExitApp"
            class="form-checkbox"
          />
          <label for="after-exit-app" class="checkbox-label">{{ t('tasks.afterMission.exitApp') }}</label>
        </div>

        <div class="radio-group">
          <div class="radio-item">
            <input
              type="radio"
              id="after-shutdown"
              :checked="modelValue.AfterShutdown"
              @change="handleShutdownChange"
              class="form-radio"
              name="power-action"
            />
            <label for="after-shutdown" class="radio-label">{{ t('tasks.afterMission.shutdown') }}</label>
          </div>

          <div class="radio-item">
            <input
              type="radio"
              id="after-sleep"
              :checked="modelValue.AfterSleep"
              @change="handleSleepChange"
              class="form-radio"
              name="power-action"
            />
            <label for="after-sleep" class="radio-label">{{ t('tasks.afterMission.sleep') }}</label>
          </div>

          <div class="radio-item">
            <input
              type="radio"
              id="after-none"
              :checked="!modelValue.AfterShutdown && !modelValue.AfterSleep"
              @change="handleNoneChange"
              class="form-radio"
              name="power-action"
            />
            <label for="after-none" class="radio-label">{{ t('tasks.afterMission.none') }}</label>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useTranslation } from '../../composables/useTranslation'

const { t } = useTranslation()

const props = defineProps<{
  modelValue: any
}>()

const handleShutdownChange = () => {
  props.modelValue.AfterShutdown = true
  props.modelValue.AfterSleep = false
}

const handleSleepChange = () => {
  props.modelValue.AfterShutdown = false
  props.modelValue.AfterSleep = true
}

const handleNoneChange = () => {
  props.modelValue.AfterShutdown = false
  props.modelValue.AfterSleep = false
}
</script>

<style scoped>
.after-mission-panel {
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

.checkbox-group {
  flex-direction: row;
  align-items: center;
  gap: 8px;
}

.form-checkbox {
  width: 16px;
  height: 16px;
  cursor: pointer;
  accent-color: #007bff;
  flex-shrink: 0;
}

.checkbox-label {
  font-size: 14px;
  color: #000;
  cursor: pointer;
}

.radio-group {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-top: 8px;
  padding: 12px;
  background: rgba(255, 255, 255, 0.3);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 4px;
}

.radio-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.form-radio {
  width: 16px;
  height: 16px;
  cursor: pointer;
  accent-color: #007bff;
  flex-shrink: 0;
}

.radio-label {
  font-size: 14px;
  color: #000;
  cursor: pointer;
}

@media (prefers-color-scheme: dark) {
  .panel-section {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .section-title,
  .checkbox-label,
  .radio-label {
    color: #fff;
  }

  .radio-group {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
}
</style>
