<script setup lang="ts">
const { isOpen, close } = useDevToolsViewModel()
</script>

<template>
  <UModal v-model:open="isOpen" title="Для разработчиков" :ui="{ content: 'max-w-3xl' }">
    <template #body>
      <ZenohDebugView />
    </template>
    <template #footer>
      <div class="flex justify-end px-4 py-3 border-t border-surface-disabled">
        <button
          class="px-4 py-1.5 rounded-lg bg-surface-dark border border-surface-disabled text-[12px] text-text-secondary hover:bg-surface-hover hover:text-text-primary transition-colors"
          @click="close"
        >
          Закрыть
        </button>
      </div>
    </template>
  </UModal>
</template>
