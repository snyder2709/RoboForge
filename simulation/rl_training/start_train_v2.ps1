# RoboForge RL Training v2 — запуск с исправленными конфигами
# Использование: Правой кнопкой на файле → Run with PowerShell

$ErrorActionPreference = "Stop"

$worktreeDir = "$PSScriptRoot\..\..\..\.claude\worktrees\xenodochial-elbakyan\simulation\rl_training"
$worktreeDir = (Resolve-Path $worktreeDir).Path

$venvPython = "$worktreeDir\.venv\Scripts\python.exe"
$trainScript = "$worktreeDir\unitree_rl_mjlab\scripts\train.py"
$workDir = "$worktreeDir\unitree_rl_mjlab"
$logFile = "$worktreeDir\train_v2.log"
$errFile = "$worktreeDir\train_v2_err.log"

Write-Host "=== RoboForge Training v2 ===" -ForegroundColor Green
Write-Host "Worktree: $worktreeDir"
Write-Host "Python:   $venvPython"
Write-Host "Log:      $logFile"
Write-Host ""

if (-not (Test-Path $venvPython)) {
    Write-Error "venv не найден: $venvPython — сначала запусти run_training.sh setup"
}

$env:PYTHONUTF8 = "1"
$env:WANDB_MODE = "disabled"

Write-Host "Запускаю обучение (100 итераций, ~2-3 часа)..." -ForegroundColor Yellow
Write-Host "Лог: $logFile" -ForegroundColor Cyan
Write-Host ""

# Запуск в foreground с выводом на экран И в файл
& $venvPython scripts\train.py Roboforge-Flat `
    --env.scene.num-envs 512 `
    --agent.max-iterations 100 `
    --agent.experiment-name roboforge_flat_v2 `
  2>&1 | Tee-Object -FilePath $logFile

Write-Host ""
Write-Host "Обучение завершено!" -ForegroundColor Green
Write-Host "Модели в: $workDir\logs\rsl_rl\roboforge_flat_v2\"
