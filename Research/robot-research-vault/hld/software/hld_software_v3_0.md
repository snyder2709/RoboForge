# HLD Программный стек v3.1

Архитектура ПО системы управления роем гуманоидных роботов.
Транспорт: **Zenoh везде** — PC-сторона и ESP32 через zenoh-pico.
ESP32 прошивка: **C/ESP-IDF** (не MicroPython).

> Принцип: весь ПО отлаживается на виртуальных роботах (Phase 0, $0). Переход на железо — замена нижнего слоя.

## Версии

| Версия | Дата | Изменения | Статус |
|--------|------|-----------|--------|
| v1.0 | — | Первый вариант, MQTT | архив |
| v2.1 | — | 5-слойная арх., MQTT | архив |
| v3.0 | 2026-03-16 | Zenoh+UDP вместо MQTT | архив |
| v3.1 | 2026-03-16 | Полный Zenoh (zenoh-pico на ESP32), убран UDP и Robot Gateway | черновик |

Обоснование перехода на C/ESP-IDF: GC-паузы MicroPython 10–50 мс создают jitter серво; zenoh-pico не имеет MicroPython-биндинга.

---

## Диаграмма 1 — Общая архитектура системы

```plantuml
@startuml
node "Brain PC" as BrainPC {
  component "Ollama LLM" as LLM
  component "Swarm Coordinator" as Coordinator
  component "Robot Agent ×N" as Agents
}

cloud "Zenoh Router" as ZenohRouter

node "ESP32 ×N (C/ESP-IDF)" as ESP32Node {
  component "zenoh-pico" as ZenohPico
  component "Servo Task\n(FreeRTOS, RT)" as ServoTask
  component "State Task\n(FreeRTOS)" as StateTask
  component "PCA9685 Driver\n(I2C 400 kHz)" as PCA
  component "MPU-6050 Driver" as IMU
}

LLM --> Coordinator
Coordinator --> Agents
Agents --> ZenohRouter : robot/{id}/cmd
ZenohRouter --> ZenohPico : robot/{id}/cmd
ZenohPico --> ServoTask : очередь команд
ServoTask --> PCA : 20 углов
StateTask --> IMU : чтение IMU
StateTask --> ZenohRouter : robot/{id}/state (2 Гц)
@enduml
```

---

## Диаграмма 2 — Внутреннее устройство ESP32 (FreeRTOS)

```plantuml
@startuml
node "ESP32 (FreeRTOS)" as ESP32 {
  component "Zenoh Task\nPriority: HIGH\n(zenoh-pico)" as ZTask
  component "Servo Control Task\nPriority: REALTIME\n50 Гц loop" as ServoTask
  component "State Publisher Task\nPriority: NORMAL\n2 Гц loop" as StateTask
  component "PCA9685\n(I2C)" as PCA
  component "MPU-6050\n(I2C)" as IMU
}

ZTask --> ServoTask : cmd queue\n(FreeRTOS Queue)
ServoTask --> PCA : set_angles(20)\nI2C batch write
StateTask --> IMU : read_imu()
StateTask --> ZTask : state payload
@enduml
```

---

## Диаграмма 3 — Поток команды движения (sequence)

```plantuml
@startuml
participant "Swarm\nCoordinator" as Coord
participant "Robot\nAgent" as Agent
participant "Zenoh\nRouter" as Router
participant "zenoh-pico\n(ESP32)" as ZPico
participant "Servo Task\n(FreeRTOS)" as Servo
participant "PCA9685" as PCA

Coord -> Agent : swarm/world_state
Agent -> Agent : LLM решение (~1 сек)
Agent -> Router : pub robot/{id}/cmd\n{action, params}
Router -> ZPico : sub robot/{id}/cmd
ZPico -> Servo : xQueueSend(cmd)
note over Servo : ~50 мс цикл\n(hardware timer)
Servo -> PCA : I2C write\n20 углов серво
PCA -> PCA : PWM → движение

loop 2 Гц
  ZPico -> Router : pub robot/{id}/state\n{pos, imu, ts}
end
@enduml
```

---

## Слои системы

### Слой 1 — LLM (локальный)

| Параметр | Значение |
|----------|---------|
| Runtime | Ollama |
| Модель | Phi-3 Mini (2.2 ГБ) / Llama 3.2 3B |
| API | http://localhost:11434/v1 |
| Temperature агент | 0.3 |
| Temperature координатор | 0.2 |

### Слой 2 — Swarm Coordinator

- Python asyncio, цикл: **5 сек**
- Публикует: `swarm/world_state`
- Получает: `swarm/events`

### Слой 3 — Robot Agents ×N

- Один процесс = один агент = один робот
- Цикл решений: **1 сек**
- Подписка: `robot/{id}/state`, `swarm/world_state`
- Публикация: `robot/{id}/cmd`

### Слой 4 — Transport Layer

**Единый протокол — Zenoh везде.**

**Zenoh QoS для motion команд:**
```
Priority::RealTime + CongestionControl::Drop
→ near-UDP latency (~1–5 мс) без отдельного UDP стека
```

**Топики:**

| Топик | Направление | QoS | Частота |
|-------|-------------|-----|---------|
| `robot/{id}/cmd` | Агент → ESP32 | RealTime + Drop | по решению |
| `robot/{id}/state` | ESP32 → Агент | BestEffort | 2 Гц |
| `robot/{id}/sensor` | ESP32 → Агент | BestEffort | по событию |
| `swarm/world_state` | Координатор → все | Reliable | 0.2 Гц |
| `swarm/events` | любой → Координатор | Reliable | по событию |

### Слой 5 — Robot Body (фазозависимый)

| Фаза | Реализация | Транспорт |
|------|-----------|-----------|
| Phase 0 | Python Virtual Robot | Zenoh (eclipse-zenoh) |
| Phase 1 | Webots контроллер | Zenoh (eclipse-zenoh) |
| Phase 2 | ESP32 C/ESP-IDF | Zenoh (zenoh-pico) |

---

## Стек и зависимости

| Компонент | Технология |
|-----------|-----------|
| Python агенты | eclipse-zenoh |
| Zenoh Router | zenohd binary |
| LLM runtime | Ollama + Phi-3 Mini |
| Dashboard | FastAPI + WebSocket |
| Симулятор | Webots R2023b+ |
| ESP32 runtime | C/ESP-IDF |
| ESP32 транспорт | zenoh-pico (idf component) |

---

## Иерархия управления

```
LLM (1–5 сек)              → стратегия через Zenoh
Агент (1 сек)              → robot/{id}/cmd через Zenoh (RealTime QoS)
ESP32 Zenoh Task           → FreeRTOS Queue → Servo Task
Servo Task (50 Гц, RT)    → IK → PCA9685 I2C → 20 серво
```

---

## ESP32 C/ESP-IDF — ключевые решения

| Решение | Почему |
|---------|--------|
| FreeRTOS Servo Task с `REALTIME` priority | Точный 50 Гц без jitter, вытесняет всё кроме прерываний |
| zenoh-pico как IDF component | `idf_component_manager` → `idf add zenohpico` |
| I2C batch write PCA9685 | Обновление всех 20 серво за одну транзакцию (<1 мс) |
| Hardware timer для servo loop | Без GC пауз, детерминированный тайминг |
| Motion command queue | FreeRTOS Queue 10 элементов, Zenoh task → Servo task |

---

## Debug инфраструктура

| Инструмент | Применение |
|-----------|-----------|
| `z_sub -k "robot/**"` | Live просмотр всех Zenoh топиков |
| `z_scout` | Обнаружение участников сети |
| Dashboard FastAPI :8000 | Визуализация позиций |
| JTAG + OpenOCD | Дебаг ESP32 с брейкпоинтами |
| ESP-IDF Monitor | Логи по UART, panic decoder |
| Ollama logs | Решения LLM |

---

## Полнота разделов

| Раздел | Готовность | Блокер |
|--------|------------|--------|
| Слои 1–3 (PC сторона) | 100% | — |
| Zenoh топики + QoS | 90% | Конфиг роя N > 4 |
| Phase 0 Virtual Robot | 85% | Обновить под eclipse-zenoh |
| Phase 1 Webots | 75% | URDF нет |
| ESP32 C/ESP-IDF скелет | 50% | Код не написан |
| IK (обратная кинематика) | 30% | Алгоритм не выбран |

**Общая готовность: ~72%**

---

## Связанные документы

- [[../index|HLD Навигатор]]
- [[../hardware/hld_hardware_v1_1|HLD Железо v1.1]]
- [[../phases/hld_phase1_v1_1|HLD Фаза 1 v1.1]]
- [[../../research/robot-network-architecture/artefacts/architectures/zenoh_distributed_architecture|Распределённая архитектура Zenoh]]
- [[../../research/robot-network-architecture/artefacts/protocols/protocol_comparison|Сравнение протоколов]]
