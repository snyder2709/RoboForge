<script setup lang="ts">
import type { ZenohFsmState } from '../composables/useZenoh'

const props = defineProps<{ state: ZenohFsmState }>()

const config = computed(() => ({
    disconnected: { dot: 'bg-text-tertiary', label: 'Disconnected', text: 'text-text-tertiary' },
    connecting:   { dot: 'bg-warning animate-pulse', label: 'Connecting…', text: 'text-warning' },
    connected:    { dot: 'bg-success', label: 'Connected', text: 'text-success' },
    testing:      { dot: 'bg-joint-highlight animate-pulse', label: 'Testing…', text: 'text-joint-highlight' },
    error:        { dot: 'bg-danger', label: 'Error', text: 'text-danger' },
}[props.state] ?? { dot: 'bg-text-tertiary', label: props.state, text: 'text-text-tertiary' }))
</script>

<template>
  <div class="flex items-center gap-1.5">
    <div class="w-2 h-2 rounded-full shrink-0" :class="config.dot" />
    <span class="text-[11px]" :class="config.text">{{ config.label }}</span>
  </div>
</template>
