<template>
  <div class="servo-grid">
    <div v-for="(angle, i) in servos" :key="i" class="servo-item">
      <span class="servo-idx">{{ i }}</span>
      <n-slider
        :value="angle"
        :min="0"
        :max="180"
        :step="1"
        :tooltip="false"
        size="small"
        @update:value="(v) => $emit('servo-change', i, v)"
      />
      <span class="servo-val">{{ Math.round(angle) }}°</span>
    </div>
  </div>
</template>

<script setup>
import { NSlider } from 'naive-ui'

defineProps({
  servos: { type: Array, required: true },
})

defineEmits(['servo-change'])
</script>

<style scoped>
.servo-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px 12px;
  padding: 8px 0;
}
.servo-item {
  display: flex;
  align-items: center;
  gap: 6px;
}
.servo-idx {
  color: #556;
  font-size: 10px;
  min-width: 16px;
  font-family: 'Courier New', monospace;
}
.servo-val {
  color: #7eb8f7;
  font-size: 10px;
  min-width: 30px;
  text-align: right;
  font-family: 'Courier New', monospace;
}
</style>
