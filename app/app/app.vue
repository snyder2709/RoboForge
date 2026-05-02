<template>
  <UApp>
    <div class="flex flex-col h-screen bg-bg-deep text-text-primary overflow-hidden">
      <AppTopBar />
      <div class="flex flex-1 min-h-0">
        <AppSideRail />
        <SettingsPanel />
        <main class="flex-1 overflow-auto min-w-0">
          <NuxtPage />
        </main>
      </div>
      <AppBottomBar />
    </div>
    <DevToolsModal />
  </UApp>
</template>
