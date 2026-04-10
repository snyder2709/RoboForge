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
| left_elbow_pitch | SG92R | CH1 | 0..2.09 rad |
| left_wrist_pitch | SG92R | CH2 | -1.57..1.57 rad |
| right_shoulder_pitch | SG92R | CH3 | -1.57..1.57 rad |
| right_elbow_pitch | SG92R | CH4 | 0..2.09 rad |
| right_wrist_pitch | SG92R | CH5 | -1.57..1.57 rad |
| left_hip_pitch | MG996R | CH6 | -1.04..1.04 rad |
| left_knee_pitch | MG996R | CH7 | 0..2.09 rad |
| right_hip_pitch | MG996R | CH8 | -1.04..1.04 rad |
| right_knee_pitch | MG996R | CH9 | 0..2.09 rad |

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

## Связанные документы

- [[EXP-005_raspi2_servo_bench_test|EXP-005]] — предварительный тест сервов
- [[../artefacts/diagrams/exp006_hardware_in_the_loop_wiring|Схема подключения (PlantUML)]]
- [[../../../../hld/hardware/hld_hardware_v1_2|HLD Железо v1.3]]
- [[../../../../hld/phases/hld_phase1_v1_1|HLD Фаза 1 v1.4]]
