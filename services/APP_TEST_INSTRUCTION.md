# Инструкция: тест Zenoh взаимодействия (App ↔ Virtual Robot)

Тест проверяет двусторонний pub/sub обмен между Tauri-приложением и стеком виртуальных роботов.
Виртуальный робот эмулирует Pico 2W / Raspberry Pi с установленным zenoh-pico:
он подписывается на `robot/{id}/cmd` и публикует `robot/{id}/state` с частотой 2 Гц.

---

## 1. Запустить стек виртуальных роботов

```bash
cd services
bash start.sh
```

Скрипт запускает:
- `zenohd` — Zenoh router на порту `:7447`
- 4 × `virtual_robot.py` (id 1–4) — симуляция робота с Zenoh pub/sub

Ожидаемый вывод:
```
[start.sh] zenohd pid=...  router=:7447
[start.sh] robot id=1  pid=...  sub=robot/1/cmd  pub=robot/1/state
...
[start.sh] all Phase 0 services running.
```

---

## 2. Запустить Tauri-приложение

В отдельном терминале:
```bash
cd app
pnpm tauri:dev
```

Приложение откроется с chrome-интерфейсом (top bar, left rail, bottom bar).

---

## 3. Открыть Developer Tools → Zenoh Debug

1. Кликнуть иконку ⚙ (Settings) в левом rail-меню снизу
2. В панели «Настройки» кликнуть **«Для разработчиков»**
3. Откроется модальное окно с секцией **Zenoh Debug**

---

## 4. Подключиться к Zenoh

1. Endpoint: `tcp/127.0.0.1:7447` (значение по умолчанию)
2. Нажать **Connect**
3. Статус изменится: `● Connected`

---

## 5. Запустить тест

1. Выбрать **Robot: 1** (или любой запущенный)
2. Выбрать **Длительность: 5 сек**
3. Нажать **▶ Запустить тест**
4. Статус изменится на `Testing…`

---

## 6. Ожидаемый результат

### Лог в приложении:

```
14:23:45 [INFO]  Test start: robot 1, 5s
14:23:45 [SENT]  robot/1/cmd — reset
14:23:46 [SENT]  robot/1/cmd — move_servos 45°×20
14:23:46 [RECV]  robot/1/state #1: {"id":1,"ts":...,"servos":[45,45,...],"imu":{...}}
14:23:46 [RECV]  robot/1/state #2: {"id":1,"ts":...,"servos":[45,45,...],"imu":{...}}
...
14:23:51 [SENT]  robot/1/cmd — reset (cleanup)
14:23:51 [DONE]  Test complete — 10 state messages received
```

Минимальный успех: ≥ 2 строки `[RECV]` за время теста.

### Консоль virtual_robot.py (терминал со start.sh):

```
14:23:45  robot.1  INFO  cmd  action=reset  ts=...
14:23:46  robot.1  INFO  cmd  action=move_servos  ts=...
14:23:51  robot.1  INFO  cmd  action=reset  ts=...
```

Наличие строки `cmd  action=move_servos` подтверждает:
**приложение успешно опубликовало команду → виртуальный робот получил её → ответил состоянием**.

---

## Что проверяется

| Направление | Топик | Проверка |
|-------------|-------|----------|
| App → Robot | `robot/1/cmd` | Строки `[SENT]` в логе + строки `cmd` в консоли робота |
| Robot → App | `robot/1/state` | Строки `[RECV]` в логе |

Это полная симуляция обмена данными как на реальном железе: приложение выступает в роли Software Agent, виртуальный робот — в роли Pico 2W / RPi с zenoh-pico.

---

## Устранение неполадок

| Симптом | Причина | Решение |
|---------|---------|---------|
| Connect → Error | zenohd не запущен | Запустить `bash start.sh` |
| 0 RECV сообщений | Нет робота с таким ID | Проверить, какие роботы запущены в start.sh |
| `pnpm tauri:dev` зависает | Порт 3001 занят | `lsof -i :3001` → завершить процесс |
| Долгий Connect | Первый запуск zenoh crate | Подождать (zenoh инициализируется) |
