# RoboForge — Services

Python бэкенд-сервисы системы управления роем гуманоидных роботов.
Транспорт: **Zenoh везде** (`eclipse-zenoh` Python).

## Обзор

```
Brain PC
├── virtual_robot ×N  — фазозависимый нижний слой (Phase 0: mock)
├── coordinator       — Swarm Coordinator, цикл 5 сек
└── agents ×N         — Robot Agent, цикл 1 сек (один процесс = один робот)

Zenoh Router (:7447)  — единственная шина данных
```

## Структура

```
services/
├── CLAUDE.md              ← правила для агентов
├── README.md              ← этот файл
├── start.sh               ← запуск всех Phase 0 сервисов
├── .venv/                 ← shared Python venv (git-ignored)
├── virtual_robot/
│   ├── virtual_robot.py
│   └── requirements.txt
├── coordinator/           ← TODO Phase 0
└── agents/                ← TODO Phase 0
```

## Требования

| Компонент | Версия |
|-----------|--------|
| Python | 3.11+ |
| zenohd | 1.x (binary) |
| eclipse-zenoh | 1.x |

```bash
# Скачать zenohd
# https://github.com/eclipse-zenoh/zenoh/releases
# Положить в ./bin/zenohd или добавить в PATH
```

## Быстрый старт

```bash
# Запуск всех Phase 0 сервисов (N=4 роботов по умолчанию)
./services/start.sh

# Переопределить количество роботов
ROBOT_COUNT=2 ./services/start.sh

# Ручной запуск одного робота
source services/.venv/bin/activate
python services/virtual_robot/virtual_robot.py 1

# Мониторинг топиков (отдельный терминал)
z_sub -k "robot/**"
z_sub -k "swarm/**"
```

## API — Zenoh топики

Авторитетный список топиков. При изменении — обновить синхронно с HLD.

| Топик | Направление | QoS | Частота | Payload |
|-------|-------------|-----|---------|---------|
| `robot/{id}/cmd` | Agent → VirtualRobot | RealTime + Drop | по событию | [CmdPayload](#cmdpayload) |
| `robot/{id}/state` | VirtualRobot → Agent | BestEffort | 2 Гц | [StatePayload](#statepayload) |
| `swarm/world_state` | Coordinator → все | Reliable | 0.2 Гц | [WorldStatePayload](#worldstatepayload) |
| `swarm/events` | любой → Coordinator | Reliable | по событию | [EventPayload](#eventpayload) |
| `ui/voice_cmd` | Tauri → Coordinator | Reliable | по событию | [VoiceCmdPayload](#voicecmdpayload) |
| `ui/status` | Coordinator → Tauri | BestEffort | 1 Гц | [UiStatusPayload](#uistatuspayload) |

### CmdPayload

```json
{
  "action": "move_servos",
  "servos": [90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0,
             90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0],
  "ts": 1710000000.123
}
```

| Поле | Тип | Описание |
|------|-----|----------|
| `action` | string | Тип команды: `move_servos`, `stop`, `reset` |
| `servos` | float[20] | Углы сервоприводов в градусах (0–180). Опционально. |
| `ts` | float | Unix timestamp отправителя |

### StatePayload

```json
{
  "id": 1,
  "ts": 1710000000.456,
  "servos": [90.0, 91.2, 89.5, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0,
             90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0, 90.0],
  "imu": {
    "roll": 0.0012,
    "pitch": -0.0034,
    "yaw": 0.0001
  }
}
```

| Поле | Тип | Описание |
|------|-----|----------|
| `id` | int | ID робота |
| `ts` | float | Unix timestamp публикации |
| `servos` | float[20] | Текущие углы сервоприводов |
| `imu` | object | Ориентация IMU в радианах |

### WorldStatePayload

```json
{
  "ts": 1710000000.0,
  "robots": {
    "1": {"state": "idle", "last_cmd_ts": 1710000000.0},
    "2": {"state": "moving", "last_cmd_ts": 1710000000.1}
  }
}
```

### EventPayload

```json
{
  "type": "robot_ready",
  "robot_id": 1,
  "ts": 1710000000.0
}
```

### VoiceCmdPayload

```json
{
  "text": "Robot 1, go to table",
  "lang": "ru",
  "ts": 1710000000.0
}
```

### UiStatusPayload

```json
{
  "ts": 1710000000.0,
  "active_robots": [1, 2, 3, 4],
  "message": "Робот 1: движение к точке A"
}
```

## Phase 0 — Порядок запуска

```
1. zenohd          — Zenoh Router (:7447)
2. virtual_robot   — N процессов, каждый с robot_id
3. coordinator     — Swarm Coordinator (TODO)
4. agents          — N Robot Agents (TODO)
```

## Связанные документы

- [HLD Программный стек v3.2](../Research/robot-research-vault/hld/software/hld_software_v3_2.md)
- [HLD Фаза 1 v1.3](../Research/robot-research-vault/hld/phases/hld_phase1_v1_1.md)
- [Правила для агентов](CLAUDE.md)
