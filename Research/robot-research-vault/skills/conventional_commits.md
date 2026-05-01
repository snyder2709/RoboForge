# Conventional Commits — конвенция коммитов RoboForge

Спецификация: https://www.conventionalcommits.org/ru/v1.0.0/

## Формат

```
<тип>(<scope>): <описание>

[тело — опционально]

[сноски — опционально]
```

## Типы

| Тип | Когда использовать | SemVer |
|-----|--------------------|--------|
| `feat` | новая функциональность | MINOR |
| `fix` | исправление бага | PATCH |
| `docs` | только документация (HLD, README, vault) | — |
| `refactor` | рефакторинг без изменения поведения | — |
| `perf` | улучшение производительности | — |
| `test` | добавление/правка тестов | — |
| `build` | система сборки, зависимости | — |
| `ci` | CI/CD конфигурация | — |
| `chore` | прочее (обновление .gitignore, инструменты) | — |
| `style` | форматирование, пробелы (не CSS) | — |
| `sim` | изменения симуляции / RL-обучения | — |
| `urdf` | изменения URDF/MJCF модели робота | — |
| `hw` | схемы, BOM, аппаратные решения | — |

> `sim`, `urdf`, `hw` — расширения специфичные для RoboForge.

## Scope (опциональный)

Указывает затронутую часть проекта:

| Scope | Область |
|-------|---------|
| `rl` | RL-обучение (`simulation/rl_training/`) |
| `urdf` | URDF/MJCF модели |
| `services` | Python-сервисы (`services/`) |
| `ui` | Tauri/Nuxt приложение (`apps/`) |
| `zenoh` | Zenoh топики, конфигурация |
| `pico` | Pico 2W / FreeRTOS прошивка |
| `hld` | HLD документы |
| `exp` | Эксперименты (EXP-xxx) |
| `webots` | Webots симуляция |

## Breaking Changes

Двумя способами (равнозначны):

```
feat(zenoh)!: change robot/{id}/cmd payload format

BREAKING CHANGE: cmd payload is now msgpack instead of JSON
```

## Примеры для RoboForge

```
feat(rl): add ankle_roll to leg actuators

fix(urdf): flip left_knee_pitch axis to anatomically correct direction

docs(hld): update HLD Hardware v1.4 → v1.5, remove wrist DOF

sim(rl): increase network hidden_dims to 512/256/128 for 20-DOF policy

urdf: add full 22-DOF arm chain to humanoid_v2

refactor(services): migrate virtual_robot to eclipse-zenoh 1.x API

chore: add requirements-torch.txt for CUDA 12.6 PyTorch

exp(exp-006): add PCA9685 channel mapping for new arm joints

feat(ui)!: replace FastAPI dashboard with Tauri 2.0 desktop app
BREAKING CHANGE: WebSocket API removed, all comms via Zenoh
```

## Правила

1. Описание — **повелительное наклонение**, строчными: `add`, `fix`, `update`, не `added`, `fixes`
2. Описание не заканчивается точкой
3. Тело отделяется пустой строкой от заголовка
4. Тело объясняет **почему**, а не что (что видно из diff)
5. Коммит должен касаться одной логической единицы изменений
6. Breaking change — всегда явно, либо `!` либо сноской `BREAKING CHANGE:`

## Применение при работе с Claude

При запросе сделать коммит Claude должен:
1. Определить тип по характеру изменений
2. Выбрать scope из таблицы выше (или не использовать если изменения глобальные)
3. Написать описание на **английском** (git-история интернациональная)
4. Добавить тело на русском если изменение нетривиальное и требует объяснения
