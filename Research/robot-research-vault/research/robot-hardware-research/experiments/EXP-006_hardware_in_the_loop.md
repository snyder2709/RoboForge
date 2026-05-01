# EXP-006 — Hardware-in-the-Loop: Zenoh + Физические сервы + Webots

## Статус
🟡 Запланирован (после EXP-005)

## Цель

Проверить что физические сервы и Webots симуляция двигаются синхронно по одной команде через Zenoh. Собрать руки (и/или ноги) на временной платформе — не полный гуманоид, но уже реальные движения. Видеть в браузере что модель соответствует физическому железу.

## Концепция

```
[Browser Debug UI]
       │ slider moved
       ↓
[ui_server.py] → Zenoh pub: robot/joints
       │
       ├──→ [Webots roboforge_controller.py]   → симуляция двигается
       │         Brain PC (localhost)
       │
       └──→ [raspi_controller.py]              → реальные сервы двигаются
                 RPi 2 B+ (Ethernet)
```

Оба подписчика слушают один топик `robot/joints`. Команда уходит один раз — двигаются оба.

## Топики Zenoh

| Топик | Направление | Формат |
|-------|-------------|--------|
| `robot/joints` | Brain PC → все | `{"joint_name": angle_rad, ...}` |
| `robot/state` | Webots → Brain PC | `{"joint_name": angle_rad, ...}` |
| `robot/physical_state` | RPi → Brain PC | `{"joint_name": angle_rad, ...}` |

## Оборудование

| Компонент | Устройство | Статус |
|-----------|-----------|--------|
| Brain PC (DESKTOP-2O3KH39) | Lenovo LOQ | ✅ есть |
| Webots R2025a | Brain PC | ✅ установлен |
| Raspberry Pi 2 B+ | RPi | ✅ есть |
| PCA9685 | RPi | 🛒 купить (EXP-005) |
| SG92R ×3 (руки) | RPi | 🛒 купить (EXP-005) |
| MG996R ×2 (ноги) | RPi | 🛒 купить (EXP-005) |
| Ethernet кабель | RPi ↔ роутер | ✅ |
| Каркас ноги | — | 🛒 купить (см. ниже) |

## Каркас ноги (одна нога для EXP-006)

Алюминиевые кронштейны под MG996R — дешевле и надёжнее оргстекла.

| Тип кронштейна | Кол-во | Где используется | Поиск на Ali |
|----------------|--------|-----------------|--------------|
| U-образный | 3 шт | крепление каждого серво в звено | `servo bracket mg996r U shape aluminum` |
| Г-образный | 2 шт | соединение звеньев между собой | `servo bracket L shape aluminum mg996r` |

**Ступня:** плоский кусок оргстекла 4мм (~5×8 см) или напечатать на Bambu A1 Mini.
**Крепёж:** набор винтов M2/M3 — `m2 m3 screw assortment kit`

## Временная платформа

Для первого теста не нужен каркас гуманоида. Цель — визуально сопоставить движение симуляции Webots и реального железа одновременно. Достаточно одной ноги: бедро → колено.

### Вариант А: Деревянная доска (~$0, 30 мин)

Минимальная сборка для валидации hardware-in-the-loop:

```
[Доска 20×10 см, толщина 15-20 мм]
    │
    └── U-кронштейн (CH6: left_hip_pitch, MG996R)
            │   вращение = сгибание бедра
            └── звено 10-15 см (линейка, реечка, оргстекло)
                    │
                    └── U-кронштейн (CH7: left_knee_pitch, MG996R)
                            │   вращение = сгибание колена
                            └── "ступня" = плоский кусок оргстекла/картона
```

**Что нужно:**
| Материал | Где взять | ~Цена |
|----------|-----------|-------|
| Деревянная доска / фанера | хозмаг / дома | $0 |
| U-кронштейны для MG996R (×2 шт) | AliExpress (уже в плане) | $2 |
| Г-кронштейн (×1 шт) | AliExpress (уже в плане) | $1 |
| Болты M3 + гайки | AliExpress kit (уже в плане) | — |
| Линейка 30 см / рейка | дома | $0 |
| Двусторонний скотч / стяжки | дома | $0 |

**Итого: $0 дополнительно** — всё уже заказано.

### Вариант Б: Напечатать стойку на Bambu A1 Mini

Если есть доступ к принтеру — распечатать простую вертикальную стойку-Т.
Файл: `sim/protos/leg_stand_v1.stl` (TODO после EXP-006).

### Как выглядит тест

```
[Экран: Браузер слева — Webots справа]
         │                    │
   слайдер двигается    нога в симуляции сгибается
         │
   Zenoh pub robot/joints
         │
   RPi → PCA9685 → 2× MG996R
         │
   физическая нога на доске сгибается синхронно
```

Наблюдать: угол бедра в Webots = угол серво на доске ±5°.
Это подтверждает что маппинг `rad → PWM pulse` корректен.

## Маппинг суставов → каналы PCA9685

Начинать с рук (6 серво), потом добавлять ноги:

| Joint (URDF) | Тип серво | PCA9685 CH | Диапазон |
|---|---|---|---|
| left_shoulder_pitch | SG92R | CH0 | -1.57..1.57 rad |
| left_shoulder_roll  | SG92R | CH1 | -1.05..1.05 rad |
| left_elbow_pitch    | SG92R | CH2 | 0..2.09 rad |
| right_shoulder_pitch| SG92R | CH3 | -1.57..1.57 rad |
| right_shoulder_roll | SG92R | CH4 | -1.05..1.05 rad |
| right_elbow_pitch   | SG92R | CH5 | 0..2.09 rad |
| left_hip_pitch      | MG996R| CH6 | -1.04..0.79 rad |
| left_knee_pitch     | MG996R| CH7 | 0..2.09 rad |
| right_hip_pitch     | MG996R| CH8 | -1.04..0.79 rad |
| right_knee_pitch    | MG996R| CH9 | 0..2.09 rad |

> **Кисти (wrist_pitch)** — removed. Физически: фиксированный крюк/заглушка на конце предплечья.
> Канал PCA9685 не нужен. SG92R на кисти не закупать.

## Настройка сети

```
Роутер (192.168.x.x)
  ├── Brain PC  — Ethernet или Wi-Fi  — zenohd запущен
  └── RPi 2 B+  — Ethernet            — raspi_controller.py
```

RPi подключается к тому же роутеру что и Brain PC. Zenoh обнаружит RPi автоматически через multicast discovery (UDP 7447).

## Программная часть на RPi

Файл: `services/raspi_controller/raspi_controller.py`

**Установка на RPi:**
```bash
sudo apt update && sudo apt install -y python3-pip i2c-tools
sudo pip3 install eclipse-zenoh adafruit-circuitpython-pca9685 adafruit-circuitpython-motor
sudo raspi-config  # I2C → Enable
```

**Запуск:**
```bash
python3 raspi_controller.py
```

## Порядок запуска (весь стек)

```bash
# Brain PC — Терминал 1: Zenoh router
./services/.bin/zenohd

# Brain PC — Терминал 2: Webots
# Открыть simulation/worlds/roboforge.wbt → Run

# Brain PC — Терминал 3: UI сервер
cd services && .venv/Scripts/python tools/ui_server.py

# Brain PC — Терминал 4: Фронтенд
cd services/tools/ui && npm run dev

# RPi — SSH терминал
python3 raspi_controller.py
```

Открыть браузер → http://localhost:5173 → вкладка "Phase 1 — Webots" → двигать слайдеры.

## Проверяемые гипотезы

- [ ] RPi обнаруживается Zenoh роутером через Ethernet автоматически
- [ ] Команда из UI → Webots и RPi двигаются синхронно (задержка < 100 мс)
- [ ] Углы в симуляции совпадают с физическими позициями сервов
- [ ] Нет пропуска команд при быстром движении слайдера
- [ ] MG90S и GDW DS041MG корректно отрабатывают граничные углы из URDF

## Следующий шаг после успеха

Доработать URDF (production качество), адаптировать STL от Poppy Humanoid, напечатать каркас и установить все 20 серво.

## TODO: обновить после перехода на URDF v2 (22 DOF)

> Добавлено 2026-04-13 при переходе URDF на полную кинематику (HLD Железо v1.4).

### Таблица маппинга суставов → каналы PCA9685

Текущая таблица (раздел «Маппинг суставов») содержит **только 10 суставов** (6 рук + 4 ноги).
Теперь робот имеет **22 сустава** — нужно добавить 12 новых каналов:

| Joint (новый) | Тип серво | PCA9685 | CH (предлагаемый) |
|---|---|---|---|
| left_shoulder_roll | SG92R | PCA9685 #1 | CH6 |
| right_shoulder_roll | SG92R | PCA9685 #1 | CH7 |
| left_hip_roll | MG996R | PCA9685 #2 | CH0 |
| right_hip_roll | MG996R | PCA9685 #2 | CH1 |
| left_hip_yaw | MG996R | PCA9685 #2 | CH2 |
| right_hip_yaw | MG996R | PCA9685 #2 | CH3 |
| left_ankle_roll | MG996R | PCA9685 #2 | CH4 |
| right_ankle_roll | MG996R | PCA9685 #2 | CH5 |
| left_knee_pitch | MG996R | PCA9685 #2 | CH6 |
| right_knee_pitch | MG996R | PCA9685 #2 | CH7 |
| left_ankle_pitch | MG996R | PCA9685 #2 | CH8 |
| right_ankle_pitch | MG996R | PCA9685 #2 | CH9 |

Итог: потребуется **2× PCA9685** (уже запланировано в HLD электроника).
При финальной распайке пересмотреть нумерацию — разбить по физическому расположению
(PCA9685 #1 = голова+руки, PCA9685 #2 = ноги).

### Другие документы для обновления

- [ ] **HLD Железо v1.4** (`hld/hardware/hld_hardware_v1_2.md`):
  строка «DOF: 21» vs сумма таблицы (22) — уточнить (вероятно, одна пара ankle_roll считалась как 1).
  Строка «Вес 350–450 г» занижена: 12×MG996R(55г)+10×SG92R(9г)+структура ≈ 1 кг.
- [ ] **raspi_controller.py** (`services/raspi_controller/`):
  добавить 12 новых суставов в словарь channel_map.
- [ ] **Webots PROTO** (`simulation/`):
  при создании production URDF для Webots взять humanoid_v2.urdf как основу.

## Связанные документы

- [[EXP-005_raspi2_servo_bench_test|EXP-005]] — предварительный тест сервов
- [[../artefacts/diagrams/exp006_hardware_in_the_loop_wiring|Схема подключения (PlantUML)]]
- [[../../../../hld/hardware/hld_hardware_v1_2|HLD Железо v1.3]]
- [[../../../../hld/phases/hld_phase1_v1_1|HLD Фаза 1 v1.4]]
