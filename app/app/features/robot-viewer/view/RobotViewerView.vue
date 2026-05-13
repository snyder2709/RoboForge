<script setup lang="ts">
import { useRobotState } from '../composables/useRobotState'

const { robotList, descriptors, selectedId, selectedRobot } = useRobotState()

const showSelect = ref(false)

const mode = computed<'solo' | 'swarm'>(() =>
  robotList.value.length > 1 ? 'swarm' : 'solo'
)

// dev override
const devMode = ref<'solo' | 'swarm' | 'auto'>('auto')
const effectiveMode = computed<'solo' | 'swarm'>(() =>
  devMode.value === 'auto' ? mode.value : devMode.value
)

// Список роботов для передачи во viewer:
// в solo-режиме — только выбранный, в swarm — все
const viewerRobots = computed(() => {
  if (effectiveMode.value === 'solo' && selectedRobot.value) {
    return [selectedRobot.value]
  }
  return robotList.value
})
</script>

<template>
  <div class="viewer-screen">
    <div class="viewer-vignette" />

    <RobotViewer :mode="effectiveMode" :robots="viewerRobots" :descriptors="descriptors" :selected-id="selectedId" />

    <!-- Robot select button -->
    <button
      class="select-btn"
      :class="{ 'select-btn--active': showSelect, 'select-btn--has-robots': robotList.length > 0 }"
      @click="showSelect = !showSelect"
    >
      <svg width="18" height="18" viewBox="0 0 18 18" fill="none">
        <rect x="5.5" y="2" width="7" height="5.5" rx="1.5" stroke="currentColor" stroke-width="1.2"/>
        <rect x="2" y="7.5" width="14" height="7" rx="1.5" stroke="currentColor" stroke-width="1.2"/>
        <line x1="5.5" y1="16" x2="5.5" y2="14.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
        <line x1="12.5" y1="16" x2="12.5" y2="14.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
        <circle cx="7.2" cy="4.6" r="0.9" fill="currentColor"/>
        <circle cx="10.8" cy="4.6" r="0.9" fill="currentColor"/>
      </svg>
      <span v-if="robotList.length > 0" class="select-badge">{{ robotList.length }}</span>
    </button>

    <!-- Robot selection overlay -->
    <RobotSelectOverlay v-if="showSelect" @close="showSelect = false" />

    <!-- Selected robot indicator (solo mode) -->
    <div v-if="selectedRobot && effectiveMode === 'solo'" class="selected-indicator">
      <span class="indicator-dot" />
      Robot #{{ selectedRobot.id }}
    </div>

    <!-- Dev toggle -->
    <div class="dev-toggle">
      <button :class="{ active: devMode === 'solo' }" @click="devMode = 'solo'">Solo</button>
      <button :class="{ active: devMode === 'auto' }" @click="devMode = 'auto'">Auto</button>
      <button :class="{ active: devMode === 'swarm' }" @click="devMode = 'swarm'">Swarm</button>
    </div>

    <!-- Robot count badge -->
    <div v-if="robotList.length > 0" class="robot-count">
      {{ robotList.length }} online
    </div>
  </div>
</template>

<style scoped>
.viewer-screen {
  position: relative;
  width: 100%;
  height: 100%;
  background: #0F0E0D;
  overflow: hidden;
}

.viewer-vignette {
  position: absolute;
  inset: 0;
  pointer-events: none;
  z-index: 5;
  background: radial-gradient(ellipse at center, transparent 50%, rgba(0,0,0,0.55) 100%);
}

/* Robot select button */
.select-btn {
  position: absolute;
  top: 12px;
  left: 12px;
  z-index: 50;
  width: 36px;
  height: 36px;
  border-radius: 10px;
  border: 0.5px solid rgba(255,255,255,0.08);
  background: rgba(28,25,23,0.88);
  color: #5F5E5A;
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(8px);
  transition: background 120ms, color 120ms, border-color 120ms;
  position: absolute;
}
.select-btn:hover {
  background: rgba(42,39,36,0.95);
  color: #9E9A93;
}
.select-btn--active {
  background: rgba(232,120,74,0.12);
  border-color: rgba(232,120,74,0.3);
  color: #E8784A;
}
.select-btn--has-robots { color: #9E9A93; }

.select-badge {
  position: absolute;
  top: -4px;
  right: -4px;
  min-width: 16px;
  height: 16px;
  padding: 0 4px;
  border-radius: 8px;
  background: #E8784A;
  color: #fff;
  font-size: 9px;
  font-weight: 600;
  display: flex;
  align-items: center;
  justify-content: center;
  line-height: 1;
}

/* Selected robot indicator */
.selected-indicator {
  position: absolute;
  top: 12px;
  left: 58px;
  z-index: 50;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  background: rgba(28,25,23,0.88);
  border: 0.5px solid rgba(255,255,255,0.08);
  border-radius: 10px;
  font-size: 11px;
  color: #F5F0EB;
  backdrop-filter: blur(8px);
}

.indicator-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #7AB87A;
  box-shadow: 0 0 4px #7AB87A;
  flex-shrink: 0;
}

.dev-toggle {
  position: absolute;
  bottom: 16px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 50;
  display: flex;
  gap: 4px;
  padding: 4px;
  background: rgba(28,25,23,0.9);
  border: 0.5px solid rgba(255,255,255,0.08);
  border-radius: 10px;
  backdrop-filter: blur(8px);
}

.dev-toggle button {
  padding: 5px 14px;
  border-radius: 7px;
  border: none;
  background: transparent;
  color: #9E9A93;
  font-size: 12px;
  font-weight: 500;
  letter-spacing: 0.04em;
  transition: background 120ms, color 120ms;
}
.dev-toggle button.active {
  background: #E8784A;
  color: #fff;
}

.robot-count {
  position: absolute;
  top: 12px;
  right: 12px;
  z-index: 50;
  padding: 3px 10px;
  background: rgba(28,25,23,0.85);
  border: 0.5px solid rgba(91,149,216,0.4);
  border-radius: 8px;
  font-size: 11px;
  color: #B5D4F4;
  letter-spacing: 0.05em;
}
</style>
