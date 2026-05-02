<template>
  <footer class="h-[76px] shrink-0 bg-surface-dark border-t border-surface-disabled flex items-center px-4 justify-between">
    <div class="flex items-center gap-4">
      <span class="text-[11px] text-text-tertiary">Нет роботов</span>
    </div>

    <button
      class="w-[124px] h-[48px] rounded-xl flex flex-col items-center justify-center gap-0.5 border-[1.5px] border-danger relative overflow-hidden"
      style="background-color: #9E2828"
    >
      <div class="absolute left-0 top-0 bottom-0 w-[3px] bg-danger opacity-70" />
      <span class="text-[9px] font-medium tracking-[2px] uppercase text-danger">EMERGENCY</span>
      <span class="text-[15px] font-medium text-text-primary">STOP</span>
    </button>
  </footer>
</template>
