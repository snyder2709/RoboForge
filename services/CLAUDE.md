# Правила для Claude-агентов — services/

Контекст: Python бэкенд-сервисы RoboForge. Транспорт — **Zenoh везде**.

## Структура

```
services/
├── CLAUDE.md              ← этот файл
├── README.md              ← описание + API (держать актуальным)
├── start.sh               ← запуск всех сервисов Phase 0
├── .venv/                 ← shared Python venv (в .gitignore)
├── virtual_robot/
│   ├── virtual_robot.py
│   └── requirements.txt
├── coordinator/           ← будет: swarm_coordinator.py
└── agents/                ← будет: robot_agent.py
```

## Соглашения Python

- Python **3.11+**
- Type hints обязательны: параметры + return type
- `asyncio` для любой конкурентности
- `logging` вместо `print`: `logging.getLogger(__name__)`
- Формат логов: `%(asctime)s  %(name)s  %(levelname)s  %(message)s`
- Имена файлов: `snake_case.py`
- Классы: `PascalCase`
- Константы: `UPPER_SNAKE_CASE`

## Zenoh

- Единственный транспорт — `eclipse-zenoh` Python
- Нельзя использовать HTTP / MQTT / WebSocket внутри `services/`
- Топики: строго по таблице в `README.md` — авторитетный источник для кода
- Payload: JSON (`json` stdlib)
- QoS согласно HLD:
  - `robot/{id}/cmd` → `Priority.REAL_TIME` + `CongestionControl.DROP`
  - `robot/{id}/state` → BestEffort (по умолчанию)
  - `swarm/*` → `CongestionControl.BLOCK` (Reliable)

## Правило новых сервисов

1. Создать `services/{name}/` с `{name}.py` + `requirements.txt`
2. Добавить запуск в `start.sh`
3. Обновить `services/README.md` — секция API и Quick Start
4. При изменении топиков — обновить HLD:
   `Research/robot-research-vault/hld/software/hld_software_v3_2.md`
5. Добавить диаграмму в vault:
   `Research/robot-research-vault/research/robot-network-architecture/artefacts/diagrams/`

## Правило обновлений

- Изменение API (топики, payload) → обновить `README.md` и HLD **синхронно в одном коммите**
- Изменение порядка запуска → обновить `start.sh` и `README.md`
- История изменений — только в git, не в коде и не в комментариях

## Запрещено

- HTTP серверы внутри сервисов (кроме Ollama-клиента)
- Глобальное состояние между сервисами вне Zenoh
- Прямые вызовы между процессами (только через Zenoh топики)
- Синхронный I/O внутри asyncio loop без `executor`

## Версионирование

- Нет файловых версий (`virtual_robot_v2.py` — запрещено)
- Версия отслеживается git тегами: `services/v{major}.{minor}`
- Breaking change (смена формата payload) → major bump
- Minor: добавление полей (обратносовместимо) → minor bump

## Связанные документы

- [[../Research/robot-research-vault/claude_rules|Правила для Claude-агентов (vault)]]
- [[../Research/robot-research-vault/hld/software/hld_software_v3_2|HLD Программный стек v3.2]]
- [[../Research/robot-research-vault/hld/phases/hld_phase1_v1_1|HLD Фаза 1 v1.3]]
