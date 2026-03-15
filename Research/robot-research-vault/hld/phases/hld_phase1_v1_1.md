# HLD Фаза 1 — Реализация на виртуальных роботах v1.1

Цель: отладить весь ПО стек без физического железа.
Транспорт: Zenoh (сервисы) + UDP (движение).

## Версии

| Версия | Дата | Изменения | Статус |
|--------|------|-----------|--------|
| v1.0 | — | MQTT транспорт | архив |
| v1.1 | 2026-03-16 | Zenoh+UDP согласно [[../software/hld_software_v3_0\|HLD SW v3.0]] | черновик |

## Зависимости

- [[../software/hld_software_v3_0|HLD Программный стек v3.0]] — должен быть финализирован

## Стек Phase 1

| Слой | Технология |
|------|-----------|
| LLM | Ollama + Phi-3 Mini |
| Агенты + Координатор | Python asyncio |
| Транспорт (сервисы) | Zenoh Router + eclipse-zenoh |
| Транспорт (движение) | UDP socket |
| Robot Gateway | Python процесс (новый) |
| Виртуальный робот | Python mock |
| Dashboard | FastAPI + WebSocket, :8000 |

## Шаг 1 — Инфраструктура

### 1.1 Zenoh Router

```bash
# Скачать zenohd: https://github.com/eclipse-zenoh/zenoh/releases
./zenohd
# Роутер слушает :7447 (по умолчанию)

# Проверка
pip install eclipse-zenoh
z_put -k test/hello -v "world"
z_sub -k "test/**"
```

### 1.2 Ollama + модель

```bash
# Установить Ollama: https://ollama.com
ollama pull phi3:mini
ollama serve  # API на http://localhost:11434
```

### 1.3 Python окружение

```bash
python -m venv .venv
source .venv/bin/activate  # или .venv\Scripts\activate на Windows
pip install eclipse-zenoh openai fastapi uvicorn websockets httpx
# paho-mqtt — НЕ НУЖЕН в v1.1
```

## Шаг 2 — Виртуальный робот (Python Mock)

Каждый виртуальный робот — отдельный процесс.

**Поведение:**
- Публикует `robot/{id}/state` через Zenoh (2 Гц)
- Подписывается на `robot/{id}/cmd` через Zenoh
- Принимает UDP пакеты от Robot Gateway (порт 9000+id)
- Симулирует изменение позиции серво

```python
# virtual_robot.py (skeleton)
import zenoh, asyncio, socket, json, time

async def run(robot_id: int):
    session = zenoh.open()
    udp_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    udp_sock.bind(("0.0.0.0", 9000 + robot_id))
    udp_sock.setblocking(False)

    pub = session.declare_publisher(f"robot/{robot_id}/state")
    sub = session.declare_subscriber(f"robot/{robot_id}/cmd", lambda s: handle_cmd(s))

    while True:
        state = {"id": robot_id, "pos": [0]*20, "ts": time.time()}
        pub.put(json.dumps(state))
        await asyncio.sleep(0.5)
```

## Шаг 3 — Robot Gateway

Новый компонент (отсутствовал в v1.0). Один процесс на робота.

**Поведение:**
- Подписывается на `robot/{id}/cmd` через Zenoh
- Переводит команду в углы серво (IK — упрощённая заглушка для Phase 1)
- Отправляет UDP пакет на Virtual Robot / ESP32

```python
# robot_gateway.py (skeleton)
import zenoh, socket, json

def on_cmd(sample):
    cmd = json.loads(sample.payload)
    servos = inverse_kinematics(cmd)  # заглушка: [90]*20
    pkt = json.dumps({"servos": servos, "timestamp": cmd["ts"]}).encode()
    udp_sock.sendto(pkt, ("127.0.0.1", 9000 + cmd["robot_id"]))

session = zenoh.open()
udp_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sub = session.declare_subscriber("robot/+/cmd", on_cmd)
```

> ⚠️ IK — заглушка `[90]*20` для Phase 1. Реальная IK — Phase 2 blocker.

## Шаг 4 — Агенты и Координатор

Минимальные изменения от v1.0 (замена paho → eclipse-zenoh):

```python
# Было (v1.0 paho):
# client.publish("robot/1/cmd", payload)

# Стало (v1.1 zenoh):
session = zenoh.open()
pub = session.declare_publisher("robot/1/cmd")
pub.put(json.dumps(cmd))
```

Цикл координатора: 5 сек. Цикл агента: 1 сек. Логика без изменений.

## Шаг 5 — Dashboard

Без изменений от v1.0. FastAPI + WebSocket + Canvas 2D.

```bash
uvicorn dashboard:app --port 8000
# Открыть http://localhost:8000
```

## Шаг 6 — Webots (Phase 1b)

Контроллер Webots использует eclipse-zenoh вместо paho-mqtt.
URDF модели нет — **блокер для Phase 1b**.

## Шаг 7 — ESP32 (Phase 2)

ESP32 получает только UDP. Zenoh на ESP32 в Phase 2 не нужен.

```python
# MicroPython UDP receiver (skeleton)
import usocket, ujson, machine

sock = usocket.socket(usocket.AF_INET, usocket.SOCK_DGRAM)
sock.bind(("0.0.0.0", 9001))  # robot_id = 1

while True:
    data, _ = sock.recvfrom(256)
    cmd = ujson.loads(data)
    set_servos(cmd["servos"])  # → PCA9685
```

## Порядок запуска

```bash
# Терминал 1 — Zenoh Router
./zenohd

# Терминал 2 — Robot Gateway (один на все роботы)
python robot_gateway.py

# Терминал 3 — Виртуальные роботы
python virtual_robot.py 1 &
python virtual_robot.py 2 &
python virtual_robot.py 3 &
python virtual_robot.py 4 &

# Терминал 4 — Агенты + Координатор
python swarm_coordinator.py

# Терминал 5 — Dashboard
uvicorn dashboard:app --port 8000
```

## Полнота разделов

| Раздел | Готовность | Блокер |
|--------|------------|--------|
| Инфраструктура (Zenoh, Ollama) | 85% | Конфиг zenohd не задокументирован |
| Virtual Robot | 80% | Код переписать на eclipse-zenoh |
| Robot Gateway | 40% | Нет реализации, только скелет |
| Агенты / Координатор | 90% | Заменить paho → zenoh |
| Dashboard | 100% | — |
| Webots | 70% | URDF нет |
| ESP32 | 50% | UDP firmware не реализован |

**Общая готовность: ~76%**

## Связанные документы

- [[../index|HLD Навигатор]]
- [[../software/hld_software_v3_0|HLD Программный стек v3.0]]
- [[../hardware/hld_hardware_v1_1|HLD Железо v1.1]]
