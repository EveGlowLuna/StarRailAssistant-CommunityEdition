<template>
  <div class="trailblaze-power-panel">
    <div class="panel-layout">
      <!-- 左侧配置区域 -->
      <div class="left-panel">
        <div class="panel-section">
          <div class="section-header">
            <input
              type="checkbox"
              id="enable-trailblaze-power"
              v-model="modelValue.EnabledTasks[1]"
              class="section-checkbox"
            />
            <label for="enable-trailblaze-power" class="section-title">{{ t('tasks.trailblazePower.title') }}</label>
          </div>

          <div class="section-content">
            <!-- 补充体力 -->
            <div class="subsection">
              <div class="subsection-header">
                <input
                  type="checkbox"
                  id="replenish-stamina"
                  v-model="modelValue.TrailblazePowerReplenishStamina"
                  class="subsection-checkbox"
                />
                <label for="replenish-stamina" class="subsection-title">{{ t('tasks.trailblazePower.replenishPower') }}</label>
              </div>

              <div class="subsection-content">
                <div class="form-group">
                  <label class="form-label">{{ t('tasks.trailblazePower.replenishMethod') }}</label>
                  <CustomDropdown
                    v-model="modelValue.TrailblazePowerReplenishWay"
                    :options="replenishWayOptions"
                    placeholder=""
                  />
                </div>

                <div class="form-group">
                  <label class="form-label">{{ t('tasks.trailblazePower.replenishTimes') }}</label>
                  <input
                    v-model.number="modelValue.TrailblazePowerReplenishTimes"
                    type="number"
                    min="0"
                    max="99"
                    class="form-input"
                  />
                </div>

                <div class="form-group checkbox-group">
                  <input
                    type="checkbox"
                    id="use-assistant"
                    v-model="modelValue.TrailblazePowerUseAssistant"
                    class="form-checkbox"
                  />
                  <label for="use-assistant" class="checkbox-label">{{ t('tasks.trailblazePower.useSupport') }}</label>
                </div>

                <div class="form-group checkbox-group">
                  <input
                    type="checkbox"
                    id="change-lineup"
                    v-model="modelValue.TrailblazePowerChangeLineup"
                    class="form-checkbox"
                  />
                  <label for="change-lineup" class="checkbox-label">{{ t('tasks.trailblazePower.reLineup') }}</label>
                </div>

                <div class="form-group checkbox-group">
                  <input
                    type="checkbox"
                    id="lineup-check"
                    v-model="modelValue.TrailblazePowerLineupCheck"
                    class="form-checkbox"
                  />
                  <label for="lineup-check" class="checkbox-label">{{ t('tasks.trailblazePower.lineupCheck') }}</label>
                </div>

                <div class="form-group checkbox-group">
                  <input
                    type="checkbox"
                    id="use-skill"
                    v-model="modelValue.TrailblazePowerUseSkill"
                    class="form-checkbox"
                  />
                  <label for="use-skill" class="checkbox-label">{{ t('tasks.trailblazePower.useSkillEntry') }}</label>
                </div>
              </div>
            </div>

            <!-- 饰品提取 -->
            <TaskTypeSection
              :title="t('tasks.trailblazePower.categories.ornament').value"
              :task-name="t('tasks.taskTypes.ornament').value"
              :levels="ornamentLevels"
              :max-count="6"
              @add-task="addTask"
            />

            <!-- 拟造花萼（金） -->
            <TaskTypeSection
              :title="t('tasks.trailblazePower.categories.calyxGolden').value"
              :task-name="t('tasks.taskTypes.calyxGolden').value"
              :levels="calyxGoldLevels"
              :max-count="24"
              @add-task="addTask"
            />

            <!-- 拟造花萼（赤） -->
            <TaskTypeSection
              :title="t('tasks.trailblazePower.categories.calyxCrimson').value"
              :task-name="t('tasks.taskTypes.calyxCrimson').value"
              :levels="calyxCrimsonLevels"
              :max-count="24"
              @add-task="addTask"
            />

            <!-- 凝滞虚影 -->
            <TaskTypeSection
              :title="t('tasks.trailblazePower.categories.stagnantShadow').value"
              :task-name="t('tasks.taskTypes.stagnantShadow').value"
              :levels="stagnantShadowLevels"
              :max-count="8"
              @add-task="addTask"
            />

            <!-- 侵蚀隧洞 -->
            <TaskTypeSection
              :title="t('tasks.trailblazePower.categories.cavern').value"
              :task-name="t('tasks.taskTypes.cavern').value"
              :levels="cavernLevels"
              :max-count="6"
              @add-task="addTask"
            />

            <!-- 历战余响 -->
            <TaskTypeSection
              :title="t('tasks.trailblazePower.categories.echoOfWar').value"
              :task-name="t('tasks.taskTypes.echoOfWar').value"
              :levels="echoLevels"
              :max-count="3"
              @add-task="addTask"
            />
          </div>
        </div>
      </div>

      <!-- 右侧任务清单 -->
      <div class="right-panel">
        <div class="task-list-container">
          <h3 class="task-list-title">{{ t('tasks.trailblazePower.taskList') }}</h3>
          <div class="task-list">
            <div
              v-for="(task, index) in modelValue.TrailblazePowerTaskList"
              :key="task.id || task.Name + task.Level + index"
              class="task-item"
              @mouseenter="hoveredTask = index"
              @mouseleave="hoveredTask = null"
            >
              <div class="task-info">
                <div class="task-type">{{ task.Name }}</div>
                <div class="task-details">
                  <span class="task-level">{{ task.LevelName }}</span>
                  <span class="task-count">{{ t('tasks.taskList.singleTime').value }}: {{ task.Count }}</span>
                  <span class="task-runtimes">{{ t('tasks.taskList.execution').value }}: {{ task.RunTimes }}</span>
                </div>
              </div>
              <div v-if="hoveredTask === index" class="task-actions">
                <button
                  class="move-button"
                  @click="moveTaskUp(index)"
                  :disabled="index === 0"
                  :title="t('tasks.taskList.moveUp').value"
                >
                  <ChevronUp :size="14" :color="'white'" />
                </button>
                <button
                  class="move-button"
                  @click="moveTaskDown(index)"
                  :disabled="index === modelValue.TrailblazePowerTaskList.length - 1"
                  :title="t('tasks.taskList.moveDown').value"
                >
                  <ChevronDown :size="14" :color="'white'" />
                </button>
                <button
                  class="delete-task-button"
                  @click="removeTask(index)"
                  :title="t('tasks.taskList.deleteTask').value"
                >
                  <Trash2 :size="14" :color="'white'" />
                </button>
              </div>
            </div>
            <div v-if="modelValue.TrailblazePowerTaskList.length === 0" class="empty-task-list">
              {{ t('common.loading') }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Trash2, ChevronUp, ChevronDown } from 'lucide-vue-next'
import CustomDropdown from '../CustomDropdown.vue'
import TaskTypeSection from './TaskTypeSection.vue'
import { useTranslation } from '../../composables/useTranslation'

const { t } = useTranslation()
const props = defineProps<{
  modelValue: any
}>()

const hoveredTask = ref<number | null>(null)

const replenishWayOptions = computed(() => [
  { label: t('tasks.trailblazePower.reservedPower').value, value: 0 },
  { label: t('tasks.trailblazePower.fuel').value, value: 1 },
  { label: t('tasks.trailblazePower.stellarJade').value, value: 2 }
])

const ornamentLevels = computed(() => [
  { label: t('tasks.trailblazePower.selectInstance').value, value: 0 },
  { label: t('tasks.trailblazePowerLevels.ornament.moonlitBlood').value, value: 1 },
  { label: t('tasks.trailblazePowerLevels.ornament.unceasingStrife').value, value: 2 },
  { label: t('tasks.trailblazePowerLevels.ornament.famishedWorker').value, value: 3 },
  { label: t('tasks.trailblazePowerLevels.ornament.eternalComedy').value, value: 4 },
  { label: t('tasks.trailblazePowerLevels.ornament.toSweetDreams').value, value: 5 },
  { label: t('tasks.trailblazePowerLevels.ornament.pouringBlades').value, value: 6 },
  { label: t('tasks.trailblazePowerLevels.ornament.fruitOfEvil').value, value: 7 },
  { label: t('tasks.trailblazePowerLevels.ornament.permafrost').value, value: 8 },
  { label: t('tasks.trailblazePowerLevels.ornament.gentleWords').value, value: 9 },
  { label: t('tasks.trailblazePowerLevels.ornament.smeltedHeart').value, value: 10 },
  { label: t('tasks.trailblazePowerLevels.ornament.untoppledWalls').value, value: 11 }
])

const calyxGoldLevels = computed(() => [
  { label: t('tasks.trailblazePower.selectInstance').value, value: 0 },
  { label: t('tasks.trailblazePowerLevels.calyxGolden.budOfMemoriesAmphoreus').value, value: 1 },
  { label: t('tasks.trailblazePowerLevels.calyxGolden.budOfAetherAmphoreus').value, value: 2 },
  { label: t('tasks.trailblazePowerLevels.calyxGolden.budOfTreasuresAmphoreus').value, value: 3 },
  { label: t('tasks.trailblazePowerLevels.calyxGolden.budOfMemoriesPenacony').value, value: 4 },
  { label: t('tasks.trailblazePowerLevels.calyxGolden.budOfAetherPenacony').value, value: 5 },
  { label: t('tasks.trailblazePowerLevels.calyxGolden.budOfTreasuresPenacony').value, value: 6 },
  { label: t('tasks.trailblazePowerLevels.calyxGolden.budOfMemoriesLuofu').value, value: 7 },
  { label: t('tasks.trailblazePowerLevels.calyxGolden.budOfAetherLuofu').value, value: 8 },
  { label: t('tasks.trailblazePowerLevels.calyxGolden.budOfTreasuresLuofu').value, value: 9 },
  { label: t('tasks.trailblazePowerLevels.calyxGolden.budOfMemoriesJarilo').value, value: 10 },
  { label: t('tasks.trailblazePowerLevels.calyxGolden.budOfAetherJarilo').value, value: 11 },
  { label: t('tasks.trailblazePowerLevels.calyxGolden.budOfTreasuresJarilo').value, value: 12 }
])

const calyxCrimsonLevels = computed(() => [
  { label: t('tasks.trailblazePower.selectInstance').value, value: 0 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.moonRageFang').value, value: 1 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.worldbreakerBlade').value, value: 2 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.divineAmber').value, value: 3 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.safeguardOfAmber').value, value: 4 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.countertemporalShot').value, value: 5 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.arrowOfTheStarchaser').value, value: 6 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.myriadFruit').value, value: 7 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.flowerOfEternity').value, value: 8 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.exquisiteColoredDraft').value, value: 9 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.keyOfWisdom').value, value: 10 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.heavenlyMelody').value, value: 11 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.stellarisSymphony').value, value: 12 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.heavenIncinerator').value, value: 13 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.obsidianOfObsession').value, value: 14 },
  { label: t('tasks.trailblazePowerLevels.calyxCrimson.flowerOfAlaya').value, value: 15 }
])

const stagnantShadowLevels = computed(() => [
  { label: t('tasks.trailblazePower.selectInstance').value, value: 0 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.invasiveClot').value, value: 1 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.ipcWorkPermit').value, value: 2 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.netherworldToken').value, value: 3 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.brokenTeethOfIronWolf').value, value: 4 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.radiantProminence').value, value: 5 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.ragingHeart').value, value: 6 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.searingSteelBlade').value, value: 7 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.endothermChitin').value, value: 8 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.seaSirensTornFin').value, value: 9 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.dreamFridge').value, value: 10 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.gelidChitin').value, value: 11 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.hornOfSnow').value, value: 12 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.nailOfTheBeastCoffin').value, value: 13 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.shapeShiffersLightningStaff').value, value: 14 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.lightningCrownOfThePastShadow').value, value: 15 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.charredBudOfTwilight').value, value: 16 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.aGlassOfTheBesottedEra').value, value: 17 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.ascendantDebirs').value, value: 18 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.stormEye').value, value: 19 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.darkveilMoonlight').value, value: 20 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.dreamFlamer').value, value: 21 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.nailOfTheApe').value, value: 22 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.voidCastIron').value, value: 23 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.harbingerOfStrife').value, value: 24 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.chordalMirage').value, value: 25 },
  { label: t('tasks.trailblazePowerLevels.stagnantShadow.suppressingEdict').value, value: 26 }
])

const cavernLevels = computed(() => [
  { label: t('tasks.trailblazePower.selectInstance').value, value: 0 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfHiddenSalvation').value, value: 1 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfThundersurge').value, value: 2 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfAria').value, value: 3 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfUncertainty').value, value: 4 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfCavalier').value, value: 5 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfDreamdive').value, value: 6 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfDarkness').value, value: 7 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfElixirSeekers').value, value: 8 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfConflagration').value, value: 9 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfHolyHymn').value, value: 10 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfProvidence').value, value: 11 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfDrifting').value, value: 12 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfJabbingPunch').value, value: 13 },
  { label: t('tasks.trailblazePowerLevels.cavern.pathOfGelidWind').value, value: 14 }
])

const echoLevels = computed(() => [
  { label: t('tasks.trailblazePower.selectInstance').value, value: 0 },
  { label: t('tasks.trailblazePowerLevels.echoOfWar.glanceOfTwilight').value, value: 1 },
  { label: t('tasks.trailblazePowerLevels.echoOfWar.innerBeastsBattlefield').value, value: 2 },
  { label: t('tasks.trailblazePowerLevels.echoOfWar.salutationsOfAshenDreams').value, value: 3 },
  { label: t('tasks.trailblazePowerLevels.echoOfWar.boreholePlanetsPastNightmares').value, value: 4 },
  { label: t('tasks.trailblazePowerLevels.echoOfWar.divineSeed').value, value: 5 },
  { label: t('tasks.trailblazePowerLevels.echoOfWar.endOfTheEternalFreeze').value, value: 6 },
  { label: t('tasks.trailblazePowerLevels.echoOfWar.destructionsBeginning').value, value: 7 },
  { label: t('tasks.trailblazePowerLevels.echoOfWar.rustedCryptOfTheIronCarcass').value, value: 8 }
])

const addTask = (task: any) => {
  if (!props.modelValue.TrailblazePowerTaskList) {
    props.modelValue.TrailblazePowerTaskList = []
  }
  props.modelValue.TrailblazePowerTaskList.push({
    ...task,
    id: Date.now() + Math.random()
  })
}

const removeTask = (index: number) => {
  props.modelValue.TrailblazePowerTaskList.splice(index, 1)
}

const moveTaskUp = (index: number) => {
  if (index > 0) {
    const task = props.modelValue.TrailblazePowerTaskList[index]
    props.modelValue.TrailblazePowerTaskList.splice(index, 1)
    props.modelValue.TrailblazePowerTaskList.splice(index - 1, 0, task)
  }
}

const moveTaskDown = (index: number) => {
  if (index < props.modelValue.TrailblazePowerTaskList.length - 1) {
    const task = props.modelValue.TrailblazePowerTaskList[index]
    props.modelValue.TrailblazePowerTaskList.splice(index, 1)
    props.modelValue.TrailblazePowerTaskList.splice(index + 1, 0, task)
  }
}
</script>

<style scoped>
.trailblaze-power-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.panel-layout {
  display: flex;
  gap: 16px;
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.left-panel {
  flex: 1;
  overflow-y: auto;
  padding-bottom: 20px;
}

.right-panel {
  width: 45%;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
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
  gap: 16px;
  padding-left: 26px;
}

.subsection {
  background: rgba(255, 255, 255, 0.3);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  padding: 12px;
}

.subsection-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}

.subsection-checkbox {
  width: 16px;
  height: 16px;
  cursor: pointer;
  accent-color: #007bff;
  flex-shrink: 0;
}

.subsection-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #000;
  cursor: pointer;
}

.subsection-content {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding-left: 24px;
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
  transition: all 0.3s ease;
}

.form-input:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.1);
}

.form-input[type="number"] {
  -moz-appearance: textfield;
  appearance: textfield;
}

.form-input[type="number"]::-webkit-inner-spin-button,
.form-input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
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

.task-list-container {
  background: rgba(255, 255, 255, 0.5);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 6px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
}

.task-list-title {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: 600;
  color: #000;
  flex-shrink: 0;
}

.task-list {
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.task-item {
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 6px;
  padding: 12px;
  margin-bottom: 8px;
  transition: all 0.3s ease;
  display: grid;
  grid-template-columns: 1fr auto;
  gap: 12px;
  align-items: center;
  position: relative;
}

.task-item:hover {
  background: rgba(255, 255, 255, 0.95);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border-color: rgba(0, 123, 255, 0.3);
}

.task-info {
  flex: 1;
  min-width: 0;
}

.task-type {
  font-size: 15px;
  font-weight: 600;
  color: #000;
  margin-bottom: 6px;
}

.task-details {
  display: flex;
  gap: 16px;
  font-size: 13px;
  color: #666;
}

.task-level {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.task-count,
.task-runtimes {
  flex-shrink: 0;
}

.task-actions {
  display: flex;
  gap: 8px;
  align-items: center;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.task-item:hover .task-actions {
  opacity: 1;
}

.move-button {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 6px;
  background: rgba(33, 150, 243, 0.9);
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  flex-shrink: 0;
  padding: 0;
}

.move-button:hover:not(:disabled) {
  background: rgba(33, 150, 243, 1);
  box-shadow: 0 2px 8px rgba(33, 150, 243, 0.4);
}

.move-button:disabled {
  opacity: 0.4;
  cursor: not-allowed;
  background: rgba(33, 150, 243, 0.5);
}

.move-button svg {
  color: white;
  width: 16px;
  height: 16px;
}

.delete-task-button {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 6px;
  background: rgba(244, 67, 54, 0.9);
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  flex-shrink: 0;
  padding: 0;
}

.delete-task-button:hover {
  background: rgba(244, 67, 54, 1);
  box-shadow: 0 2px 8px rgba(244, 67, 54, 0.4);
}

.delete-task-button svg {
  color: white;
  width: 16px;
  height: 16px;
}

.empty-task-list {
  text-align: center;
  padding: 40px 20px;
  color: #999;
  font-size: 14px;
}

.left-panel::-webkit-scrollbar,
.task-list::-webkit-scrollbar {
  width: 6px;
}

.left-panel::-webkit-scrollbar-track,
.task-list::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 3px;
}

.left-panel::-webkit-scrollbar-thumb,
.task-list::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 3px;
}

.left-panel::-webkit-scrollbar-thumb:hover,
.task-list::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

@media (prefers-color-scheme: dark) {
  .panel-section,
  .task-list-container {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .subsection {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .section-title,
  .subsection-title,
  .form-label,
  .checkbox-label,
  .task-list-title,
  .task-type {
    color: #fff;
  }

  .move-button svg,
  .delete-task-button svg {
    color: white;
  }

  .form-input {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.3);
    color: #fff;
  }

  .task-item {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .task-item:hover {
    background: rgba(255, 255, 255, 0.15);
    border-color: rgba(0, 123, 255, 0.5);
  }

  .task-details {
    color: #ccc;
  }

  .empty-task-list {
    color: #666;
  }
}
</style>
