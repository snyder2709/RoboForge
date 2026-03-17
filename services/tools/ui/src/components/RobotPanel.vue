<template>
  <n-card
    class="robot-panel"
    :bordered="true"
    size="small"
    content-style="padding: 0;"
  >
    <!-- Header -->
    <template #header>
      <div class="panel-header">
        <div class="panel-left">
          <span class="robot-title">ROBOT {{ robot.id }}</span>
          <n-badge
            :type="robot.live ? 'success' : 'warning'"
            :value="robot.live ? 'live' : 'idle'"
            :processing="robot.live"
          />
        </div>
        <div class="panel-right">
          <n-select
            v-model:value="selectedScenario"
            :options="scenarioOptions"
            placeholder="сценарий"
            size="small"
            style="width: 160px"
            :consistent-menu-width="false"
          />
          <n-button
            size="small"
            :type="scenarioRunning ? 'warning' : 'primary'"
            :disabled="!selectedScenario && !scenarioRunning"
            @click="scenarioRunning ? onStop() : onExecute()"
          >
            {{ scenarioRunning ? '■' : '▶' }}
          </n-button>
          <span class="ts-label">{{ tsStr }}</span>
        </div>
      </div>
    </template>

    <!-- 3D canvas -->
    <RobotCanvas :servos="robot.servos" :imu="robot.imu" :live="robot.live" />

    <!-- Servos (collapsible) -->
    <n-collapse class="servo-collapse">
      <n-collapse-item title="Сервоприводы" name="servos">
        <ServoGrid
          :servos="localServos[robot.id]"
          @servo-change="onServoChange"
        />
      </n-collapse-item>
    </n-collapse>

    <!-- Controls -->
    <div class="controls">
      <n-button size="small" type="error" ghost @click="sendCmd(robot.id, 'stop')">
        STOP
      </n-button>
      <n-button size="small" ghost @click="sendCmd(robot.id, 'reset')">
        RESET
      </n-button>
    </div>
  </n-card>
</template>

<script setup>
import { computed, ref } from 'vue'
import { NCard, NButton, NSelect, NCollapse, NCollapseItem, NBadge } from 'naive-ui'
import RobotCanvas from './RobotCanvas.vue'
import ServoGrid   from './ServoGrid.vue'
import { useRobotStore }    from '../viewmodels/useRobotStore'
import { useScenarioStore } from '../viewmodels/useScenarioStore'

const props = defineProps({
  robotId: { type: Number, required: true },
})

const { robots, localServos, sendCmd, setServo } = useRobotStore()
const { scenarioOptions, running, execute, stop } = useScenarioStore()

const robot = computed(() => robots[props.robotId])
const selectedScenario = ref(null)
const scenarioRunning = computed(() => !!running.value[props.robotId])

const tsStr = computed(() => {
  if (!robot.value.ts) return '—'
  return new Date(robot.value.ts * 1000).toLocaleTimeString()
})

function onExecute() {
  if (selectedScenario.value) execute(props.robotId, selectedScenario.value)
}

function onStop() {
  stop(props.robotId)
  sendCmd(props.robotId, 'stop')
}

function onServoChange(idx, value) {
  setServo(props.robotId, idx, value)
}
</script>

<style scoped>
.robot-panel {
  background: #0f0f1e;
  border-color: #1e1e3a !important;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  flex-wrap: wrap;
}

.panel-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.panel-right {
  display: flex;
  align-items: center;
  gap: 6px;
}

.robot-title {
  color: #7eb8f7;
  font-size: 12px;
  letter-spacing: 1px;
  font-family: 'Courier New', monospace;
}

.ts-label {
  color: #445;
  font-size: 10px;
  min-width: 60px;
  font-family: 'Courier New', monospace;
}

.servo-collapse {
  padding: 0 12px;
  border-top: 1px solid #1a1a2e;
}

.controls {
  display: flex;
  gap: 8px;
  padding: 8px 12px;
  border-top: 1px solid #1a1a2e;
}
</style>
