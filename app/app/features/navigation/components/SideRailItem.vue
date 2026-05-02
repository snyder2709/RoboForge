<script setup lang="ts">
defineProps<{
    icon: string
    label: string
    active: boolean
    disabled?: boolean
}>()

defineEmits<{ click: [] }>()
</script>

<template>
  <button
    class="relative w-[56px] h-[44px] flex items-center justify-center transition-colors"
    :class="[
      active ? 'text-text-primary' : 'text-text-tertiary hover:text-text-secondary',
      disabled && 'cursor-not-allowed opacity-40',
    ]"
    :disabled="disabled"
    :title="label"
    @click="!disabled && $emit('click')"
  >
    <div
      v-if="active"
      class="absolute left-0 top-[10px] bottom-[10px] w-[3px] bg-coral-primary rounded-r"
    />
    <UIcon :name="icon" class="w-5 h-5" />
  </button>
</template>
