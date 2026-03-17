# HLD Фаза 1 — Реализация на виртуальных роботах v1.3

Цель: отладить весь ПО стек без физического железа.
Транспорт: **Zenoh везде** — агенты и виртуальные роботы через единый Zenoh Router.
UI: **Tauri 2.0** — Desktop (Brain PC) + Mobile (iOS/Android), монорепо.

## Версии

| Версия | Дата | Изменения | Статус |
|--------|------|-----------|--------|
| v1.0 | — | MQTT транспорт | архив |
| v1.1 | 2026-03-16 | Zenoh+UDP, Robot Gateway | архив |
| v1.2 | 2026-03-16 | Полный Zenoh, Robot Gateway убран | архив |
| v1.3 | 2026-03-17 | Tauri 2.0 вместо FastAPI Dashboard, монорепо | черновик |

## Зависимости

- [[../software/hld_software_v3_2|HLD Программный стек v3.2]]

## Стек Phase 1

| Слой | Технология |
|------|-----------|
| LLM | Ollama + Phi-3 Mini |
| Агенты + Координатор | Python asyncio + eclipse-zenoh |
| Транспорт | Zenoh Router |
| Виртуальный робот | Python mock + eclipse-zenoh |
| Desktop UI | Tauri 2.0 Desktop (Rust + WebUI) |
| Mobile UI | Tauri 2.0 Mobile (iOS / Android) |
| Монорепо | pnpm workspaces (apps/desktop, apps/mobile, packages/ui) |
| Frontend WebUI | Nuxt 4 (Vue 3 + Vite) — шаблон готов |

## Шаг 1 — Инфраструктура

### 1.1 Zenoh Router

```bash
# Скачать zenohd: https://github.com/eclipse-zenoh/zenoh/releases
./zenohd
# Роутер слушает :7447

# Проверка
pip install eclipse-zenoh
z_put -k test/hello -v "world"
z_sub -k "test/**"
```

### 1.2 Ollama + модель

```bash
ollama pull phi3:mini
ollama serve  # API: http://localhost:11434
```

### 1.3 Python окружение

```bash
python -m venv .venv
.venv\Scripts\activate
pip install eclipse-zenoh openai httpx
```

## Шаг 2 — Виртуальный робот (Python Mock)

Каждый виртуальный робот — отдельный процесс. Подписывается на `robot/{id}/cmd` и публикует `robot/{id}/state` — всё через Zenoh.

```python
# virtual_robot.py
import zenoh, json, time, asyncio

def run(robot_id: int):
    session = zenoh.open(zenoh.Config())

    servos = [90] * 20  # текущие углы

    def on_cmd(sample):
        cmd = json.loads(bytes(sample.payload))
        # Phase 1: заглушка — просто принимаем команду
        print(f"Robot {robot_id} cmd: {cmd}")

    sub = session.declare_subscriber(f"robot/{robot_id}/cmd", on_cmd)
    pub = session.declare_publisher(f"robot/{robot_id}/state")

    while True:
        state = {"id": robot_id, "servos": servos, "ts": time.time()}
        pub.put(json.dumps(state))
        time.sleep(0.5)  # 2 Гц
```

## Шаг 3 — Агенты и Координатор

Публикуют `robot/{id}/cmd` через Zenoh с `Priority::RealTime`:

```python
# agent.py (фрагмент)
import zenoh, json

config = zenoh.Config()
session = zenoh.open(config)

# Объявить publisher с RealTime QoS
pub = session.declare_publisher(
    f"robot/{robot_id}/cmd",
    priority=zenoh.Priority.REAL_TIME(),
    congestion_control=zenoh.CongestionControl.DROP()
)

def send_cmd(action: str):
    pub.put(json.dumps({"action": action, "ts": time.time()}))
```

## Шаг 4 — Dashboard (Tauri 2.0)

**Tauri Desktop** — основной UI на Brain PC. Rust backend подписывается на Zenoh топики и транслирует события в WebView через Tauri events.

```bash
# Структура монорепо
apps/
  desktop/   # Tauri 2.0 Desktop (Brain PC)
  mobile/    # Tauri 2.0 Mobile (iOS / Android)
packages/
  ui/        # Shared Nuxt 4 компоненты (Vue 3)
  zenoh-ipc/ # Tauri plugin: Zenoh ↔ WebView bridge

# Запуск Desktop (dev)
cd apps/desktop && cargo tauri dev
```

**Для Phase 0** (dev-режим без Tauri, для быстрого старта):

```bash
# Опционально: простой HTML/JS клиент для отладки Zenoh топиков
# Подключается к REST API zenohd (:8000 zenoh admin)
z_sub -k "robot/**"   # CLI вместо UI на старте
```

## Шаг 5 — Webots (Phase 1b)

Контроллер Webots подписывается на `robot/{id}/cmd` через eclipse-zenoh — напрямую, без промежуточных компонентов.

```python
# webots_controller.py (фрагмент)
import zenoh
from controller import Robot

robot = Robot()
session = zenoh.open(zenoh.Config())

def on_cmd(sample):
    cmd = json.loads(bytes(sample.payload))
    apply_to_webots_motors(robot, cmd)

session.declare_subscriber("robot/1/cmd", on_cmd)
```

Блокер: **URDF 28-30 см гуманоида** — нет файла.

## Порядок запуска

```bash
# Терминал 1 — Zenoh Router
./zenohd

# Терминал 2 — Виртуальные роботы
python virtual_robot.py 1 &
python virtual_robot.py 2 &
python virtual_robot.py 3 &
python virtual_robot.py 4 &

# Терминал 3 — Агенты + Координатор
python swarm_coordinator.py

# Терминал 4 — Tauri Desktop UI
cd apps/desktop && cargo tauri dev
```

## Полнота разделов

| Раздел | Готовность | Блокер |
|--------|------------|--------|
| Инфраструктура (Zenoh, Ollama) | 90% | — |
| Virtual Robot (Zenoh) | 85% | Написать финальный код (Q-01) |
| Агенты / Координатор | 90% | Заменить paho → zenoh |
| Tauri Desktop UI | 10% | Монорепо не инициализировано, STT не выбран |
| Tauri Mobile UI | 5% | Монорепо не инициализировано |
| Webots | 70% | URDF нет |

**Общая готовность: ~82%**

## Связанные документы

- [[../index|HLD Навигатор]]
- [[../software/hld_software_v3_2|HLD Программный стек v3.2]]
- [[../hardware/hld_hardware_v1_2|HLD Железо v1.2]]
