<template>
  <div class="task-type-section">
    <h4 class="section-title">{{ title }}</h4>
    <div class="section-form">
      <div class="form-row">
        <label class="form-label">{{ t('tasks.trailblazePower.selectLevel').value }}</label>
        <CustomDropdown
          v-model="selectedLevel"
          :options="levels"
          :placeholder="t('tasks.trailblazePower.selectLevel').value"
        />
      </div>

      <div class="form-row">
        <label class="form-label">{{ t('tasks.trailblazePower.continuousBattle').value }}</label>
        <input
          v-model.number="count"
          type="number"
          min="1"
          :max="maxCount"
          class="form-input"
        />
      </div>

      <div class="form-row">
        <label class="form-label">{{ t('tasks.trailblazePower.runTimes').value }}</label>
        <input
          v-model.number="runTimes"
          type="number"
          min="1"
          max="99"
          class="form-input"
        />
      </div>

      <button class="add-button" @click="handleAddTask">
        <Plus :size="16" />
        {{ t('tasks.trailblazePower.add').value }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Plus } from 'lucide-vue-next'
import CustomDropdown from '../CustomDropdown.vue'
import { useTranslation } from '../../composables/useTranslation'

const { t } = useTranslation()

const props = defineProps<{
  title: string
  taskName: string
  levels: Array<{ label: string; value: number }>
  maxCount: number
}>()

const emit = defineEmits<{
  addTask: [task: any]
}>()

const selectedLevel = ref(0)
const count = ref(1)
const runTimes = ref(1)

const handleAddTask = () => {
  if (selectedLevel.value === 0) {
    window.showNotification?.(t('tasks.notifications.selectLevel').value, 2000)
    return
  }

  const levelInfo = props.levels.find(l => l.value === selectedLevel.value)
  if (!levelInfo) return

  emit('addTask', {
    Name: props.taskName,
    Level: selectedLevel.value,
    LevelName: levelInfo.label,
    Count: count.value,
    RunTimes: runTimes.value
  })

  // 重置表单
  selectedLevel.value = 0
  count.value = 1
  runTimes.value = 1
}
</script>

<style scoped>
.task-type-section {
  background: rgba(255, 255, 255, 0.3);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  padding: 12px;
}

.section-title {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
  color: #000;
}

.section-form {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.form-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.form-label {
  width: 80px;
  font-size: 13px;
  font-weight: 500;
  color: #000;
  flex-shrink: 0;
}

.form-input {
  flex: 1;
  padding: 6px 10px;
  border: 1px solid rgba(0, 0, 0, 0.2);
  border-radius: 4px;
  font-size: 13px;
  background: rgba(255, 255, 255, 0.9);
  transition: all 0.3s ease;
}

.form-input:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.1);
}

.form-input[type="number"] {
  -moz-appearance: textfield;
}

.form-input[type="number"]::-webkit-inner-spin-button,
.form-input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.add-button {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
  background: rgba(33, 150, 243, 0.8);
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  margin-top: 4px;
}

.add-button:hover {
  background: rgba(33, 150, 243, 0.9);
  transform: translateY(-1px);
}

@media (prefers-color-scheme: dark) {
  .task-type-section {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .section-title,
  .form-label {
    color: #fff;
  }

  .form-input {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.3);
    color: #fff;
  }
}
</style>
