<script setup lang="ts">
import { useRobotState } from '../composables/useRobotState'

const emit = defineEmits<{ close: [] }>()

const { robotList, descriptors, selectedId, selectRobot } = useRobotState()

function onSelect(id: number) {
  selectRobot(id)
  emit('close')
}

// Считаем робота живым если state не старше 3 секунд
function isAlive(ts: number) {
  return Date.now() / 1000 - ts < 3
}

// IMU roll/pitch в градусах, округлённо
function imuDeg(v: number) {
  return (v * 180 / Math.PI).toFixed(1)
}
</script>

<template>
  <Transition name="overlay">
    <div class="robot-overlay" @click.self="emit('close')">
      <div class="overlay-panel glass-card">
        <!-- Header -->
        <div class="overlay-header">
          <span class="overlay-title">Роботы</span>
          <button class="close-btn" @click="emit('close')">
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
              <path d="M1 1l12 12M13 1L1 13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
            </svg>
          </button>
        </div>

        <!-- Empty state -->
        <div v-if="robotList.length === 0" class="empty-state">
          <div class="empty-icon">
            <svg width="28" height="28" viewBox="0 0 28 28" fill="none">
              <rect x="9" y="4" width="10" height="8" rx="2" stroke="#5F5E5A" stroke-width="1.2"/>
              <rect x="5" y="12" width="18" height="10" rx="2" stroke="#5F5E5A" stroke-width="1.2"/>
              <circle cx="10" cy="7" r="1.2" fill="#5F5E5A"/>
              <circle cx="14" cy="7" r="1.2" fill="#5F5E5A"/>
              <line x1="8" y1="22" x2="8" y2="26" stroke="#5F5E5A" stroke-width="1.2" stroke-linecap="round"/>
              <line x1="20" y1="22" x2="20" y2="26" stroke="#5F5E5A" stroke-width="1.2" stroke-linecap="round"/>
            </svg>
          </div>
          <span class="empty-label">Нет подключённых роботов</span>
          <span class="empty-hint">Запустите virtual_robot и подключитесь к Zenoh</span>
        </div>

        <!-- Robot cards -->
        <div v-else class="card-list">
          <button
            v-for="robot in robotList"
            :key="robot.id"
            class="robot-card"
            :class="{ selected: robot.id === selectedId, dead: !isAlive(robot.ts) }"
            @click="onSelect(robot.id)"
          >
            <!-- Active indicator -->
            <div class="card-indicator" :class="isAlive(robot.ts) ? 'alive' : 'dead'" />

            <div class="card-body">
              <div class="card-row-top">
                <span class="card-id">Robot #{{ robot.id }}</span>
                <span class="card-model">{{ descriptors.get(robot.id)?.model ?? '—' }}</span>
              </div>

              <div class="card-row-imu">
                <span class="imu-label">R</span><span class="imu-val">{{ imuDeg(robot.imu?.roll ?? 0) }}°</span>
                <span class="imu-label">P</span><span class="imu-val">{{ imuDeg(robot.imu?.pitch ?? 0) }}°</span>
                <span class="imu-label">Y</span><span class="imu-val">{{ imuDeg(robot.imu?.yaw ?? 0) }}°</span>
              </div>

              <div class="card-row-joints">
                <span class="joints-count">{{ descriptors.get(robot.id)?.joints.length ?? '?' }} joints</span>
                <span class="card-status" :class="isAlive(robot.ts) ? 'online' : 'offline'">
                  {{ isAlive(robot.ts) ? 'online' : 'timeout' }}
                </span>
              </div>
            </div>

            <!-- Selected checkmark -->
            <div v-if="robot.id === selectedId" class="selected-mark">
              <svg width="10" height="8" viewBox="0 0 10 8" fill="none">
                <path d="M1 4l3 3 5-6" stroke="#E8784A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.robot-overlay {
  position: absolute;
  inset: 0;
  z-index: 40;
  pointer-events: all;
}

.overlay-panel {
  position: absolute;
  left: 12px;
  top: 12px;
  width: 240px;
  max-height: calc(100% - 24px);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: rgba(20, 18, 17, 0.92);
  border: 0.5px solid rgba(255,255,255,0.09);
  border-radius: 14px;
  backdrop-filter: blur(16px);
}

.overlay-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 14px 10px;
  border-bottom: 0.5px solid rgba(255,255,255,0.06);
  flex-shrink: 0;
}

.overlay-title {
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.04em;
  color: #F5F0EB;
}

.close-btn {
  width: 22px;
  height: 22px;
  border-radius: 6px;
  border: none;
  background: rgba(255,255,255,0.06);
  color: #9E9A93;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 120ms, color 120ms;
}
.close-btn:hover { background: rgba(255,255,255,0.11); color: #F5F0EB; }

.card-list {
  overflow-y: auto;
  padding: 8px;
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.robot-card {
  position: relative;
  display: flex;
  align-items: stretch;
  gap: 10px;
  padding: 10px 10px 10px 12px;
  border-radius: 10px;
  border: 0.5px solid rgba(255,255,255,0.06);
  background: rgba(255,255,255,0.03);
  text-align: left;
  transition: background 120ms, border-color 120ms;
  width: 100%;
}
.robot-card:hover {
  background: rgba(255,255,255,0.06);
  border-color: rgba(255,255,255,0.10);
}
.robot-card.selected {
  background: rgba(232,120,74,0.08);
  border-color: rgba(232,120,74,0.28);
}
.robot-card.dead { opacity: 0.5; }

.card-indicator {
  width: 3px;
  border-radius: 2px;
  flex-shrink: 0;
  align-self: stretch;
}
.card-indicator.alive { background: #7AB87A; }
.card-indicator.dead  { background: #5F5E5A; }

.card-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 5px;
  min-width: 0;
}

.card-row-top {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 6px;
}

.card-id {
  font-size: 13px;
  font-weight: 600;
  color: #F5F0EB;
}

.card-model {
  font-size: 9px;
  color: #5F5E5A;
  letter-spacing: 0.05em;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100px;
}

.card-row-imu {
  display: flex;
  align-items: center;
  gap: 4px;
}

.imu-label {
  font-size: 9px;
  color: #5F5E5A;
  letter-spacing: 0.06em;
}

.imu-val {
  font-size: 10px;
  color: #9E9A93;
  margin-right: 5px;
  font-variant-numeric: tabular-nums;
}

.card-row-joints {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.joints-count {
  font-size: 10px;
  color: #5F5E5A;
}

.card-status {
  font-size: 9px;
  letter-spacing: 0.07em;
  text-transform: uppercase;
}
.card-status.online { color: #7AB87A; }
.card-status.offline { color: #5F5E5A; }

.selected-mark {
  align-self: center;
  margin-left: 2px;
  flex-shrink: 0;
}

/* Empty state */
.empty-state {
  padding: 24px 16px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  text-align: center;
}
.empty-icon { opacity: 0.4; margin-bottom: 4px; }
.empty-label { font-size: 12px; color: #9E9A93; }
.empty-hint  { font-size: 10px; color: #5F5E5A; line-height: 1.5; }

/* Transition */
.overlay-enter-active, .overlay-leave-active { transition: opacity 160ms ease; }
.overlay-enter-from, .overlay-leave-to { opacity: 0; }

.overlay-enter-active .overlay-panel,
.overlay-leave-active .overlay-panel {
  transition: transform 160ms ease, opacity 160ms ease;
}
.overlay-enter-from .overlay-panel,
.overlay-leave-to .overlay-panel {
  transform: translateX(-10px);
  opacity: 0;
}
</style>
