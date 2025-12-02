<template>
  <div class="receive-reward-panel">
    <div class="panel-section">
      <div class="section-header">
        <input
          type="checkbox"
          id="enable-receive-reward"
          v-model="modelValue.EnabledTasks[2]"
          class="section-checkbox"
        />
        <label for="enable-receive-reward" class="section-title">{{ t('tasks.receiveReward.title') }}</label>
      </div>

      <div class="section-content">
        <div class="form-group checkbox-group">
          <input
            type="checkbox"
            id="reward-0"
            v-model="modelValue.ReceiveRewards[0]"
            class="form-checkbox"
          />
          <label for="reward-0" class="checkbox-label">{{ t('tasks.receiveReward.guide') }}</label>
        </div>

        <div class="form-group checkbox-group">
          <input
            type="checkbox"
            id="reward-1"
            v-model="modelValue.ReceiveRewards[1]"
            class="form-checkbox"
          />
          <label for="reward-1" class="checkbox-label">{{ t('tasks.receiveReward.assignments') }}</label>
        </div>

        <div class="form-group checkbox-group">
          <input
            type="checkbox"
            id="reward-2"
            v-model="modelValue.ReceiveRewards[2]"
            class="form-checkbox"
          />
          <label for="reward-2" class="checkbox-label">{{ t('tasks.receiveReward.mailbox') }}</label>
        </div>

        <div class="form-group checkbox-group">
          <input
            type="checkbox"
            id="reward-3"
            v-model="modelValue.ReceiveRewards[3]"
            class="form-checkbox"
          />
          <label for="reward-3" class="checkbox-label">{{ t('tasks.receiveReward.dailyTraining') }}</label>
        </div>

        <div class="form-group checkbox-group">
          <input
            type="checkbox"
            id="reward-4"
            v-model="modelValue.ReceiveRewards[4]"
            class="form-checkbox"
          />
          <label for="reward-4" class="checkbox-label">{{ t('tasks.receiveReward.namelessHonor') }}</label>
        </div>

        <div class="form-group checkbox-group">
          <input
            type="checkbox"
            id="reward-5"
            v-model="modelValue.ReceiveRewards[5]"
            class="form-checkbox"
          />
          <label for="reward-5" class="checkbox-label">{{ t('tasks.receiveReward.giftOfOdyssey') }}</label>
        </div>

        <div class="form-group checkbox-group">
          <input
            type="checkbox"
            id="reward-6"
            v-model="modelValue.ReceiveRewards[6]"
            class="form-checkbox"
          />
          <label for="reward-6" class="checkbox-label">{{ t('tasks.receiveReward.redemptionCode') }}</label>
        </div>

        <div class="form-group">
          <label class="form-label">{{ t('tasks.receiveReward.redeemCodes') }}</label>
          <textarea
            v-model="modelValue.ReceiveRewardRedeemCodes"
            class="form-textarea"
            placeholder=""
            rows="5"
          ></textarea>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useTranslation } from '../../composables/useTranslation'

const { t } = useTranslation()

defineProps<{
  modelValue: any
}>()
</script>

<style scoped>
.receive-reward-panel {
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

.form-textarea {
  padding: 8px 12px;
  border: 1px solid rgba(0, 0, 0, 0.2);
  border-radius: 4px;
  font-size: 14px;
  background: rgba(255, 255, 255, 0.9);
  transition: border-color 0.3s ease;
  resize: vertical;
  font-family: inherit;
}

.form-textarea:focus {
  outline: none;
  border-color: #007bff;
}

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

  .form-textarea {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.3);
    color: #fff;
  }
}
</style>
