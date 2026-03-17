# Импорт URDF в Webots

## Файлы симуляции

| Файл | Путь |
|------|------|
| Мир Webots | `simulation/worlds/roboforge.wbt` |
| Контроллер | `simulation/controllers/roboforge_controller/roboforge_controller.py` |
| URDF модель | `Research/robot-research-vault/research/robot-hardware-research/artefacts/urdf/humanoid_v1.urdf` |

## Что нужно

- Webots R2023b или новее (Windows x64)
- URDF файл: [[../artefacts/urdf/humanoid_v1|humanoid_v1.urdf]]

## Шаги импорта

### 1. Открыть Webots

Запустить Webots → создать новый мир (`File → New World`) или открыть существующий.

### 2. Запустить импорт URDF

```
Wizards → Import a URDF robot
```

В диалоге выбрать файл `humanoid_v1.urdf`.

### 3. Настройки импорта

| Параметр | Значение |
|----------|---------|
| Normal map | отключить |
| Self-collision | включить для ног |
| Add bounding objects | включить |
| Use inertia from URDF | включить |

Нажать **Import**.

### 4. Разместить робота

После импорта робот появляется в сцене. В `Scene Tree`:
- Найти `Robot` → `translation` → установить `0 0 0.14` (поднять над полом ~14 см)
- `rotation` → `0 1 0 0` (стоячая поза)

### 5. Добавить пол

`Add → PROTO nodes → Floors → RectangleArena` — стандартный пол для тестов.

### 6. Запустить симуляцию

Нажать ▶ (Run). Робот упадёт — это ожидаемо до настройки контроллера.

## Дебаг суставов вручную

В `Scene Tree` → раскрыть `Robot` → найти нужный `HingeJoint` → `position` — менять значение в радианах и смотреть в 3D-вьюере.

Или через `Robot Window` (двойной клик на роботе) — там слайдеры по всем суставам.

## Подключение контроллера Python

Создать файл `roboforge_controller.py`:

```python
from controller import Robot

robot = Robot()
timestep = int(robot.getBasicTimeStep())

motors = {}
joint_names = [
    "head_pan",
    "left_shoulder_pitch", "left_elbow_pitch", "left_wrist_pitch",
    "right_shoulder_pitch", "right_elbow_pitch", "right_wrist_pitch",
    "left_hip_yaw", "left_hip_pitch", "left_knee_pitch", "left_ankle_pitch",
    "right_hip_yaw", "right_hip_pitch", "right_knee_pitch", "right_ankle_pitch",
]

for name in joint_names:
    m = robot.getDevice(name)
    m.setPosition(0.0)  # T-поза
    motors[name] = m

while robot.step(timestep) != -1:
    pass  # сюда логику управления
```

В `Scene Tree` → `Robot` → `controller` → вписать путь к файлу.

## Интеграция с Zenoh (Phase 1)

Контроллер читает команды через zenoh-python и транслирует в `setPosition()`:

```python
import zenoh

session = zenoh.open(zenoh.Config())
sub = session.declare_subscriber(
    "robot/joints",
    lambda sample: apply_joints(sample.payload)
)
```

Zenoh-сессия на стороне Webots работает как обычный zenoh-peer — не отличается от реального робота.

## Частые проблемы

| Проблема | Причина | Решение |
|----------|---------|---------|
| Робот падает сразу | Нет контроллера / нет gravity compensation | Добавить контроллер с `setPosition` |
| Суставы проваливаются друг в друга | Неправильные collision shapes | Уменьшить радиус цилиндров в URDF на 10% |
| "Unknown joint" в контроллере | Опечатка в имени | Проверить имена в URDF |
| Импорт не находит файл | Пробелы в пути | Путь без пробелов |

## Ссылки

- [[../artefacts/urdf/humanoid_v1|URDF модель]]
- [[../decisions/index|ADR: выбор контроллера]]
- [[../../robot-network-architecture/index|Сетевая архитектура (Zenoh)]]
