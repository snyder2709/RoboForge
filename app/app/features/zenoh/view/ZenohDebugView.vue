<script setup lang="ts">
import { SCENARIO_OPTIONS } from '~/view-model/zenohDebug'

const vm = useZenohDebugViewModel()

const robotIdOptions = [1, 2, 3, 4] as const
const durationOptions = [
    { label: '5 сек', value: 5 },
    { label: '10 сек', value: 10 },
    { label: '30 сек', value: 30 },
] as const

const isConnected = computed(() =>
    vm.connectionState.value === 'connected' || vm.connectionState.value === 'testing',
)
const canRunTest = computed(() =>
    vm.connectionState.value === 'connected' && !vm.isRunning.value,
)

function runActiveTest() {
    if (vm.testType.value === 'swarm') vm.handleRunSwarmTest()
    else vm.handleRunTest()
}
</script>

<template>
  <div class="flex flex-col gap-5">
    <!-- Connection -->
    <section class="rounded-xl border border-surface-disabled overflow-hidden">
      <div class="px-4 py-2 bg-surface-hover border-b border-surface-disabled">
        <p class="text-[10px] uppercase tracking-[2px] text-text-tertiary">Подключение</p>
      </div>
      <div class="px-4 py-3 flex flex-col gap-3">
        <div class="flex gap-2">
          <input
            v-model="vm.endpoint.value"
            class="flex-1 bg-bg-deep border border-surface-disabled rounded-lg px-3 py-1.5 text-[12px] text-text-primary font-mono focus:outline-none focus:border-coral-primary transition-colors"
            placeholder="tcp/127.0.0.1:7447"
            :disabled="isConnected"
          >
          <button
            class="px-4 py-1.5 rounded-lg text-[12px] font-medium transition-colors shrink-0"
            :class="isConnected
              ? 'bg-surface-dark border border-surface-disabled text-text-secondary hover:bg-surface-hover hover:text-danger'
              : 'bg-coral-primary text-bg-deep hover:opacity-90'"
            @click="vm.handleConnect"
          >
            {{ isConnected ? 'Disconnect' : 'Connect' }}
          </button>
        </div>
        <ZenohStatusBadge :state="vm.connectionState.value" />
      </div>
    </section>

    <!-- Test config -->
    <section class="rounded-xl border border-surface-disabled overflow-hidden">
      <div class="px-4 py-2 bg-surface-hover border-b border-surface-disabled flex items-center justify-between">
        <p class="text-[10px] uppercase tracking-[2px] text-text-tertiary">Конфигурация теста</p>
        <!-- Test type toggle -->
        <div class="flex gap-1 bg-bg-deep rounded-lg p-0.5 border border-surface-disabled">
          <button
            v-for="opt in [{ value: 'single', label: 'Одиночный' }, { value: 'swarm', label: 'Swarm' }]"
            :key="opt.value"
            class="px-3 h-6 rounded-md text-desc transition-colors"
            :class="vm.testType.value === opt.value
              ? 'bg-coral-primary text-bg-deep font-medium'
              : 'text-text-tertiary hover:text-text-secondary'"
            @click="vm.testType.value = opt.value as 'single' | 'swarm'"
          >
            {{ opt.label }}
          </button>
        </div>
      </div>

      <!-- Single test config -->
      <div v-if="vm.testType.value === 'single'" class="px-4 py-3 flex flex-wrap items-end gap-4">
        <div class="flex flex-col gap-1">
          <label class="text-[10px] text-text-tertiary uppercase tracking-[1px]">Робот</label>
          <div class="flex gap-1.5">
            <button
              v-for="id in robotIdOptions"
              :key="id"
              class="w-8 h-8 rounded-lg text-[12px] font-medium border transition-colors"
              :class="vm.robotId.value === id
                ? 'bg-coral-primary border-coral-primary text-bg-deep'
                : 'bg-bg-deep border-surface-disabled text-text-secondary hover:border-coral-primary hover:text-text-primary'"
              @click="vm.robotId.value = id"
            >
              {{ id }}
            </button>
          </div>
        </div>
        <div class="flex flex-col gap-1">
          <label class="text-[10px] text-text-tertiary uppercase tracking-[1px]">Длительность</label>
          <div class="flex gap-1.5">
            <button
              v-for="opt in durationOptions"
              :key="opt.value"
              class="px-3 h-8 rounded-lg text-[12px] border transition-colors"
              :class="vm.durationSecs.value === opt.value
                ? 'bg-coral-primary border-coral-primary text-bg-deep font-medium'
                : 'bg-bg-deep border-surface-disabled text-text-secondary hover:border-coral-primary hover:text-text-primary'"
              @click="vm.durationSecs.value = opt.value"
            >
              {{ opt.label }}
            </button>
          </div>
        </div>
      </div>

      <!-- Swarm test config -->
      <div v-else class="px-4 py-3 flex flex-wrap items-end gap-4">
        <div class="flex flex-col gap-1">
          <label class="text-[10px] text-text-tertiary uppercase tracking-[1px]">Сценарий</label>
          <div class="flex gap-1.5">
            <button
              v-for="opt in SCENARIO_OPTIONS"
              :key="opt.value"
              class="px-3 h-8 rounded-lg text-[12px] border transition-colors"
              :class="vm.scenario.value === opt.value
                ? 'bg-coral-primary border-coral-primary text-bg-deep font-medium'
                : 'bg-bg-deep border-surface-disabled text-text-secondary hover:border-coral-primary hover:text-text-primary'"
              @click="vm.scenario.value = opt.value"
            >
              {{ opt.label }}
            </button>
          </div>
        </div>
        <div class="flex flex-col gap-1">
          <label class="text-[10px] text-text-tertiary uppercase tracking-[1px]">Роботов</label>
          <div class="flex gap-1.5">
            <button
              v-for="id in robotIdOptions"
              :key="id"
              class="w-8 h-8 rounded-lg text-[12px] font-medium border transition-colors"
              :class="vm.robotCount.value === id
                ? 'bg-coral-primary border-coral-primary text-bg-deep'
                : 'bg-bg-deep border-surface-disabled text-text-secondary hover:border-coral-primary hover:text-text-primary'"
              @click="vm.robotCount.value = id"
            >
              {{ id }}
            </button>
          </div>
        </div>
      </div>

      <!-- Run button -->
      <div class="px-4 pb-3 flex justify-end">
        <button
          class="px-5 h-9 rounded-lg text-bar-value font-medium transition-colors flex items-center gap-2"
          :class="canRunTest
            ? 'bg-coral-primary text-bg-deep hover:opacity-90'
            : 'bg-surface-dark border border-surface-disabled text-text-tertiary cursor-not-allowed'"
          :disabled="!canRunTest"
          @click="runActiveTest"
        >
          <UIcon
            :name="vm.isRunning.value ? 'i-heroicons-arrow-path' : 'i-heroicons-play'"
            class="w-4 h-4"
            :class="vm.isRunning.value && 'animate-spin'"
          />
          {{ vm.isRunning.value ? 'Тест идёт…' : (vm.testType.value === 'swarm' ? 'Запустить Swarm' : 'Запустить тест') }}
        </button>
      </div>
    </section>

    <!-- Log -->
    <section class="rounded-xl border border-surface-disabled overflow-hidden">
      <div class="px-4 py-2 bg-surface-hover border-b border-surface-disabled flex items-center justify-between">
        <p class="text-[10px] uppercase tracking-[2px] text-text-tertiary">Лог</p>
        <button
          class="text-[11px] text-text-tertiary hover:text-text-secondary transition-colors"
          @click="vm.clearLogs"
        >
          Очистить
        </button>
      </div>
      <div class="p-3">
        <ZenohLogWindow :entries="vm.logs.value" />
      </div>
    </section>
  </div>
</template>
