<script setup lang="ts">
const appVersion = ref('—')

onMounted(async () => {
    appVersion.value = await getVersion().catch(() => '—')
})
</script>

<template>
  <header class="h-[44px] shrink-0 bg-surface-dark border-b border-surface-disabled flex items-center px-4 gap-3">
    <div class="w-6 h-6 rounded-md bg-coral-primary flex items-center justify-center shrink-0">
      <span class="text-[9px] font-medium text-bg-deep">RF</span>
    </div>
    <span class="text-[13px] font-medium text-text-primary">RoboForge</span>
    <span class="text-[10px] text-text-secondary px-1.5 py-0.5 rounded bg-bg-deep border border-surface-disabled">
      v{{ appVersion }}
    </span>
  </header>
</template>
