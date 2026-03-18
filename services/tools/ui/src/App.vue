<template>
  <n-config-provider :theme="darkTheme" :theme-overrides="themeOverrides">
    <n-global-style />
    <div class="app">
      <header class="app-header">
        <span class="logo">RoboForge</span>
        <n-tag :type="statusType" size="small" round>{{ wsStatus }}</n-tag>
        <span class="version">v{{ version }}</span>
      </header>

      <n-tabs v-model:value="activeTab" type="line" class="tabs">
        <n-tab-pane name="phase0" tab="Phase 0 — Virtual">
          <div class="grid">
            <RobotPanel v-for="id in robotIds" :key="id" :robot-id="id" />
          </div>
        </n-tab-pane>
        <n-tab-pane name="phase1" tab="Phase 1 — Webots">
          <WebotsPanel />
        </n-tab-pane>
      </n-tabs>
    </div>
  </n-config-provider>
</template>

<script setup>
import { ref, computed } from 'vue'
import { NConfigProvider, NGlobalStyle, NTag, NTabs, NTabPane, darkTheme } from 'naive-ui'
import RobotPanel from './components/RobotPanel.vue'
import WebotsPanel from './components/WebotsPanel.vue'
import { wsStatus } from './services/wsService'
import { version } from '../package.json'

const robotIds = [1, 2, 3, 4]
const activeTab = ref('phase0')

const themeOverrides = {
  common: {
    primaryColor:       '#7eb8f7',
    primaryColorHover:  '#a8d4ff',
    bodyColor:          '#0a0a12',
    cardColor:          '#0f0f1e',
    borderColor:        '#1e1e3a',
    textColorBase:      '#c8d0e0',
  },
}

const statusType = computed(() => ({
  connected:    'success',
  disconnected: 'error',
  connecting:   'warning',
}[wsStatus.value] ?? 'default'))
</script>

<style>
* { box-sizing: border-box; margin: 0; padding: 0; }
body { background: #0a0a12; font-family: 'Courier New', monospace; }

.app-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 16px;
  background: #0f0f1e;
  border-bottom: 1px solid #1e1e3a;
}

.logo {
  font-size: 14px;
  letter-spacing: 2px;
  color: #7eb8f7;
  text-transform: uppercase;
}

.version { color: #334; font-size: 11px; margin-left: auto; }

.tabs {
  padding: 0 10px;
}

.grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 10px;
  padding: 10px;
}
</style>
