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
| MG90S ×3 (руки) | RPi | 🛒 купить (EXP-005) |
| GDW DS041MG ×2 (ноги) | RPi | 🛒 купить (EXP-005) |
| Ethernet кабель | RPi ↔ роутер | ✅ |
| Временная платформа | — | 🔨 сделать |

## Временная платформа

Для первого теста не нужен каркас гуманоида. Варианты:

- **Деревянная доска** — прикрутить серво-кронштейны, достаточно для теста диапазона движения
- **Картонная коробка** — самый быстрый вариант
- **3D-печать кронштейнов** — только стойки под MG90S/GDW, без всего каркаса

Минимально: зафиксировать 2-3 сервы так чтобы видеть движение плеча → локтя → запястья.

## Маппинг суставов → каналы PCA9685

Начинать с рук (6 серво), потом добавлять ноги:

| Joint (URDF) | Тип серво | PCA9685 CH | Диапазон |
|---|---|---|---|
| left_shoulder_pitch | MG90S | CH0 | -1.57..1.57 rad |
| left_elbow_pitch | MG90S | CH1 | 0..2.09 rad |
| left_wrist_pitch | MG90S | CH2 | -1.57..1.57 rad |
| right_shoulder_pitch | MG90S | CH3 | -1.57..1.57 rad |
| right_elbow_pitch | MG90S | CH4 | 0..2.09 rad |
| right_wrist_pitch | MG90S | CH5 | -1.57..1.57 rad |
| left_hip_pitch | GDW DS041MG | CH6 | -1.04..1.04 rad |
| left_knee_pitch | GDW DS041MG | CH7 | 0..2.09 rad |
| right_hip_pitch | GDW DS041MG | CH8 | -1.04..1.04 rad |
| right_knee_pitch | GDW DS041MG | CH9 | 0..2.09 rad |

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
- [[../../../../hld/hardware/hld_hardware_v1_2|HLD Железо v1.3]]
- [[../../../../hld/phases/hld_phase1_v1_1|HLD Фаза 1 v1.4]]
