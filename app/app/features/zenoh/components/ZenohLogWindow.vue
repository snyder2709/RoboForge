<script setup lang="ts">
import type { ZenohLogEntry } from '../composables/useZenoh'

const props = defineProps<{ entries: readonly ZenohLogEntry[] }>()

const levelColor: Record<string, string> = {
    INFO:  'text-text-secondary',
    SENT:  'text-joint-highlight',
    RECV:  'text-success',
    DONE:  'text-coral-primary',
    ERROR: 'text-danger',
}

const scrollEl = ref<HTMLDivElement>()

function formatTime(ts: number) {
    return new Date(ts * 1000).toLocaleTimeString('ru-RU', { hour12: false })
}

watch(
    () => props.entries.length,
    () => nextTick(() => {
        if (scrollEl.value) scrollEl.value.scrollTop = scrollEl.value.scrollHeight
    }),
)
</script>

<template>
  <div
    ref="scrollEl"
    class="h-64 overflow-y-auto rounded-lg bg-bg-deep border border-surface-disabled p-3 font-mono text-[11px]"
  >
    <div v-if="!props.entries.length" class="text-text-tertiary">
      Лог пуст. Подключитесь и запустите тест.
    </div>
    <div v-for="(entry, i) in props.entries" :key="i" class="flex gap-2 leading-5">
      <span class="text-text-tertiary shrink-0">{{ formatTime(entry.ts) }}</span>
      <span class="shrink-0 w-10" :class="levelColor[entry.level] ?? 'text-text-secondary'">[{{ entry.level }}]</span>
      <span class="text-text-secondary break-all">{{ entry.message }}</span>
    </div>
  </div>
</template>
