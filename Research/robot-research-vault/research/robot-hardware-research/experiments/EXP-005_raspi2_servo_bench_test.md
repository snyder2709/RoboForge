# EXP-005 — Стендовый тест сервоприводов на Raspberry Pi 2 Model B+

## Статус
🟡 Запланирован

## Цель

Проверить работу сервоприводов (MG90S и GDW DS041MG) через PCA9685 на Raspberry Pi 2 B+ до закупки полного комплекта. Первый опыт работы с железом. Питание от розетки, без Wi-Fi и Zenoh.

## Имеющееся оборудование

| Компонент | Статус |
|-----------|--------|
| Raspberry Pi 2 Model B+ v1.2 (512MB) | ✅ есть |
| Ethernet кабель | предположительно есть |
| Micro-USB зарядник для Pi (5V 2A+) | предположительно есть |

## Что нужно купить (минимум)

| Компонент | Кол-во | ~USD | Зачем |
|-----------|--------|------|-------|
| PCA9685 16-ch PWM модуль | 1 шт | ~$3 | Управление сервами через I2C |
| MG90S metal gear servo | 3 шт | ~$3 | Тест лёгких серво (руки/голова) |
| GDW DS041MG | 2 шт | ~$8 | Тест несущих серво (ноги) |
| Dupont провода F-F 20 см | 1 набор | ~$2 | Pi → PCA9685 |
| Макетная плата (breadboard) | 1 шт | ~$2 | Удобная разводка |
| USB адаптер 5V 2A для серво | 1 шт | ~$3 | Отдельное питание серво |

**Итого: ~$21**

> Pi питается от своего micro-USB зарядника.
> Сервы питаются от отдельного USB адаптера → PCA9685 V+.
> Никогда не питай сервы от GPIO пинов Pi — сгорит.

## Ссылки AliExpress

| Компонент | Ссылка |
|-----------|--------|
| PCA9685 ~$3.15 | https://www.aliexpress.com/item/32469378576.html |
| MG90S (выбор кол-ва) | https://www.aliexpress.com/item/1005006219266362.html |
| GDW DS041MG | https://www.aliexpress.com/item/1005003740378327.html |
| Dupont провода набор | искать: `dupont wire 40pcs female female 20cm` |
| Breadboard 830 точек | искать: `breadboard 830 point solderless` |

## Схема подключения

```
Raspberry Pi 2 B+          PCA9685
─────────────────          ───────────────────────
Pin 1  (3.3V)    ────────→ VCC  (логика 3.3V)
Pin 6  (GND)     ────────→ GND
Pin 3  (SDA, GPIO2) ─────→ SDA
Pin 5  (SCL, GPIO3) ─────→ SCL

USB 5V адаптер (внешний)
(+) ────────────────────→ V+   (питание серво)
(-) ────────────────────→ GND

PCA9685 выходы            Сервы
──────────────────────────────
CH0 ──────────────────→ Серво 1 (MG90S)
CH1 ──────────────────→ Серво 2 (MG90S)
CH2 ──────────────────→ Серво 3 (GDW DS041MG)
```

## Программная часть

**ОС:** Raspberry Pi OS Lite (64-bit)

**Установка зависимостей:**
```bash
sudo apt update
sudo apt install -y python3-pip i2c-tools
sudo pip3 install adafruit-circuitpython-pca9685 adafruit-circuitpython-motor

# Включить I2C
sudo raspi-config  # → Interface Options → I2C → Enable

# Проверить что PCA9685 найден (должен показать 0x40)
sudo i2cdetect -y 1
```

**Тест скрипт:**
```python
import time
import board
import busio
from adafruit_pca9685 import PCA9685
from adafruit_motor import servo

i2c = busio.I2C(board.SCL, board.SDA)
pca = PCA9685(i2c)
pca.frequency = 50  # 50 Гц для серво

servo0 = servo.Servo(pca.channels[0], min_pulse=500, max_pulse=2500)

# Тест движения
for angle in [0, 45, 90, 135, 180, 90]:
    servo0.angle = angle
    print(f"angle: {angle}")
    time.sleep(1)

pca.deinit()
```

## Проверяемые гипотезы

- [ ] PCA9685 обнаруживается по I2C на адресе 0x40
- [ ] MG90S плавно двигается в диапазоне 0–180°
- [ ] GDW DS041MG держит позицию под нагрузкой
- [ ] Нет помех/перезагрузок Pi при движении серво
- [ ] Pulse width: min=500µs, max=2500µs (подобрать под каждый тип)

## Следующий шаг после успеха

Заказать полный комплект (11× GDW DS041MG + 9× MG90S + Pico 2W + 2× PCA9685) и перейти к сборке каркаса.

## Связанные документы

- [[../index|Исследование: Hardware]]
- [[../../decisions/index|Принятые решения]]
- [[../../../../hld/hardware/hld_hardware_v1_2|HLD Железо v1.3]]
