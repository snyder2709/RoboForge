<script setup lang="ts">
const { activeSection, activate } = useNavigation()

const topItems = [
    { icon: 'i-heroicons-squares-2x2', section: 'library' as const, label: 'Library', disabled: true },
    { icon: 'i-heroicons-cpu-chip', section: 'robots' as const, label: 'Robots', disabled: true },
    { icon: 'i-heroicons-map', section: 'mission' as const, label: 'Mission', disabled: true },
    { icon: 'i-heroicons-beaker', section: 'simulation' as const, label: 'Simulation', disabled: true },
    { icon: 'i-heroicons-document-text', section: 'logs' as const, label: 'Logs', disabled: true },
]
</script>

<template>
  <nav class="w-14 shrink-0 flex flex-col h-full bg-bg-deep border-r border-surface-disabled">
    <div class="pt-2 flex flex-col">
      <SideRailItem
        v-for="item in topItems"
        :key="item.section"
        :icon="item.icon"
        :label="item.label"
        :disabled="item.disabled"
        :active="activeSection === item.section"
        @click="activate(item.section)"
      />
    </div>
    <div class="flex-1" />
    <div class="pb-2">
      <SideRailItem
        icon="i-heroicons-cog-6-tooth"
        label="Settings"
        :active="activeSection === 'settings'"
        @click="activate('settings')"
      />
    </div>
  </nav>
</template>
