<template>
  <div class="webots-panel">
    <div class="webots-header">
      <span class="robot-label">Webots Robot</span>
      <n-tag :type="isLive ? 'success' : 'default'" size="small" round>
        {{ isLive ? 'live' : 'idle' }}
      </n-tag>
      <n-button size="tiny" @click="resetAll" style="margin-left: auto">Reset T-pose</n-button>
    </div>

    <div v-if="joints.length === 0" class="no-joints">
      Joints not loaded — restart ui_server.py
    </div>

    <div v-else class="joint-grid">
      <div v-for="joint in joints" :key="joint.name" class="joint-row">
        <span class="joint-name">{{ joint.name }}</span>
        <n-slider
          v-model:value="positions[joint.name]"
          :min="joint.min"
          :max="joint.max"
          :step="0.01"
          @update:value="sendCmd"
        />
        <span class="joint-value">{{ formatAngle(positions[joint.name]) }}</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, onUnmounted } from 'vue'
import { NSlider, NTag, NButton } from 'naive-ui'
import { useWsService } from '../services/wsService'

const { onMessage, send } = useWsService()

const joints = ref([])
const positions = reactive({})
const isLive = ref(false)

let unsub = null
let debounceTimer = null

onMounted(() => {
  unsub = onMessage((msg) => {
    if (msg.type === 'joints_config') {
      joints.value = msg.joints
      msg.joints.forEach((j) => {
        if (!(j.name in positions)) positions[j.name] = 0.0
      })
    } else if (msg.type === 'webots_state') {
      Object.assign(positions, msg.joints)
      isLive.value = true
    }
  })
})

onUnmounted(() => {
  unsub?.()
  clearTimeout(debounceTimer)
})

function sendCmd() {
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    send({ type: 'phase1_cmd', joints: { ...positions } })
  }, 50)
}

function resetAll() {
  joints.value.forEach((j) => { positions[j.name] = 0.0 })
  send({ type: 'phase1_cmd', joints: { ...positions } })
}

function formatAngle(v) {
  return (v ?? 0).toFixed(2) + ' rad'
}
</script>

<style scoped>
.webots-panel {
  background: #0f0f1e;
  border: 1px solid #1e1e3a;
  border-radius: 6px;
  padding: 12px;
  max-width: 720px;
  margin: 10px auto;
}

.webots-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 14px;
}

.robot-label {
  font-size: 13px;
  letter-spacing: 1px;
  color: #7eb8f7;
  text-transform: uppercase;
}

.no-joints {
  color: #556;
  font-size: 12px;
  padding: 20px 0;
  text-align: center;
}

.joint-grid {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.joint-row {
  display: grid;
  grid-template-columns: 180px 1fr 72px;
  align-items: center;
  gap: 10px;
}

.joint-name {
  font-size: 11px;
  color: #8899bb;
  font-family: 'Courier New', monospace;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.joint-value {
  font-size: 11px;
  color: #c8d0e0;
  text-align: right;
  font-family: 'Courier New', monospace;
}
</style>
