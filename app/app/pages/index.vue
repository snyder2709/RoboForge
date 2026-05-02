<script setup lang="ts">
const appVersion = ref<string>("")
const osInfo = ref<string>("")
const shellOutput = ref<string>("")
const isLoading = reactive({
  version: false,
  notification: false,
  os: false,
  shell: false,
})

async function testVersion() {
  isLoading.version = true
  appVersion.value = await getVersion()
  isLoading.version = false
}

async function testNotification() {
  isLoading.notification = true
  await sendNotification({ title: "RoboForge", body: "Native notification works ✓" })
  isLoading.notification = false
}

async function testOS() {
  isLoading.os = true
  const p = await platform()
  const a = await arch()
  osInfo.value = `${p} / ${a}`
  isLoading.os = false
}

async function testShell() {
  isLoading.shell = true
  const cmd = await Command.create("echo", ["RoboForge shell ✓"]).execute()
  shellOutput.value = cmd.stdout.trim()
  isLoading.shell = false
}
</script>

<template>
  <div class="h-full bg-bg-deep text-text-primary flex flex-col items-center justify-center gap-8 p-8">
    <!-- Logo -->
    <div class="flex items-center gap-3">
      <div class="w-8 h-8 rounded-md bg-coral-primary flex items-center justify-center">
        <span class="text-xs font-medium text-bg-deep">RF</span>
      </div>
      <span class="text-[14px] font-medium tracking-wide">RoboForge</span>
      <span class="text-[11px] text-text-secondary px-2 py-0.5 rounded bg-surface-dark border border-surface-disabled">
        v{{ appVersion || "—" }}
      </span>
    </div>

    <!-- Title -->
    <div class="text-center">
      <h1 class="text-[18px] font-medium text-text-primary mb-1">App template ready</h1>
      <p class="text-[11px] text-text-secondary">Nuxt 4 + Tauri 2 · pnpm · Desktop / iOS / Android</p>
    </div>

    <!-- Native API test panel -->
    <div class="w-full max-w-md rounded-[14px] border border-white/8 overflow-hidden"
      style="background: linear-gradient(180deg, rgba(255,255,255,0.06) 0%, rgba(255,255,255,0.02) 50%, rgba(0,0,0,0.18) 100%)">

      <div class="px-4 py-3 border-b border-surface-disabled">
        <p class="text-[10px] uppercase tracking-[2px] text-text-tertiary">Native API tests</p>
      </div>

      <div class="divide-y divide-surface-disabled">
        <!-- App version -->
        <div class="flex items-center justify-between px-4 py-3">
          <div>
            <p class="text-[13px] font-medium">App version</p>
            <p class="text-[11px] text-text-secondary">{{ appVersion || "not fetched" }}</p>
          </div>
          <button
            class="px-3 py-1.5 rounded-lg bg-surface-dark border border-surface-disabled text-[11px] text-text-secondary hover:bg-surface-hover hover:text-text-primary transition-colors"
            :disabled="isLoading.version"
            @click="testVersion"
          >
            {{ isLoading.version ? "…" : "Run" }}
          </button>
        </div>

        <!-- Notification -->
        <div class="flex items-center justify-between px-4 py-3">
          <div>
            <p class="text-[13px] font-medium">Notification</p>
            <p class="text-[11px] text-text-secondary">tauri-plugin-notification</p>
          </div>
          <button
            class="px-3 py-1.5 rounded-lg bg-surface-dark border border-surface-disabled text-[11px] text-text-secondary hover:bg-surface-hover hover:text-text-primary transition-colors"
            :disabled="isLoading.notification"
            @click="testNotification"
          >
            {{ isLoading.notification ? "…" : "Run" }}
          </button>
        </div>

        <!-- OS info -->
        <div class="flex items-center justify-between px-4 py-3">
          <div>
            <p class="text-[13px] font-medium">OS info</p>
            <p class="text-[11px] text-text-secondary">{{ osInfo || "not fetched" }}</p>
          </div>
          <button
            class="px-3 py-1.5 rounded-lg bg-surface-dark border border-surface-disabled text-[11px] text-text-secondary hover:bg-surface-hover hover:text-text-primary transition-colors"
            :disabled="isLoading.os"
            @click="testOS"
          >
            {{ isLoading.os ? "…" : "Run" }}
          </button>
        </div>

        <!-- Shell -->
        <div class="flex items-center justify-between px-4 py-3">
          <div>
            <p class="text-[13px] font-medium">Shell command</p>
            <p class="text-[11px] text-text-secondary">{{ shellOutput || "not run" }}</p>
          </div>
          <button
            class="px-3 py-1.5 rounded-lg bg-surface-dark border border-surface-disabled text-[11px] text-text-secondary hover:bg-surface-hover hover:text-text-primary transition-colors"
            :disabled="isLoading.shell"
            @click="testShell"
          >
            {{ isLoading.shell ? "…" : "Run" }}
          </button>
        </div>
      </div>
    </div>

    <!-- E-STOP preview -->
    <button
      class="w-[124px] h-[48px] rounded-xl flex flex-col items-center justify-center gap-0.5 border-[1.5px] border-danger relative overflow-hidden"
      style="background-color: #9E2828"
    >
      <div class="absolute left-0 top-0 bottom-0 w-[3px] bg-danger opacity-70" />
      <span class="text-[9px] font-medium tracking-[2px] uppercase text-danger">EMERGENCY</span>
      <span class="text-[15px] font-medium text-text-primary">STOP</span>
    </button>
  </div>
</template>
