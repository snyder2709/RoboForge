<script setup lang="ts">
const { isSettingsPanelOpen } = useNavigation()
const { open: openDevTools } = useDevToolsViewModel()
</script>

<template>
  <div
    class="h-full bg-surface-dark border-r border-surface-disabled flex flex-col shrink-0 overflow-hidden transition-[width] duration-200"
    :style="{ width: isSettingsPanelOpen ? '340px' : '0px' }"
  >
    <div class="w-[340px] h-full flex flex-col">
      <div class="px-4 py-3 border-b border-surface-disabled shrink-0">
        <p class="text-[10px] uppercase tracking-[2px] text-text-tertiary">Настройки</p>
      </div>

      <button
        class="flex items-center justify-between px-4 py-3 hover:bg-surface-hover transition-colors text-left w-full"
        @click="openDevTools"
      >
        <div>
          <p class="text-[13px] font-medium text-text-primary">Для разработчиков</p>
          <p class="text-[11px] text-text-secondary mt-0.5">Zenoh debug, инструменты отладки</p>
        </div>
        <UIcon name="i-heroicons-chevron-right" class="w-4 h-4 text-text-tertiary shrink-0 ml-2" />
      </button>
    </div>
  </div>
</template>
