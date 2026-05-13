---

kanban-plugin: board

---

## Запланировано

- [ ] **[Protocol]** Coordinator-сервис: распределение команд по роботам
- [ ] **[Protocol]** Agent-сервис: исполнение команд на роботе
- [ ] **[Backend]** Команда `send_cmd` — публикация `robot/{id}/cmd` из UI
- [ ] **[Viewer]** Отображение IMU данных (pitch/roll) на карточке / в 3D
- [ ] **[Viewer]** Анимация суставов по реальным joint-значениям из `robot/{id}/state`
- [ ] **[Viewer]** Swarm: клик по роботу → select (ray casting или 2D bbox)
- [ ] **[UI]** Панель управления роботом: джойстик / кнопки команд
- [ ] **[UI]** Timeline / лог команд и состояния
- [ ] **[UI]** Настройки подключения Zenoh (router address) из UI
- [ ] **[Simulation]** Webots: обновить PROTO под финальный URDF
- [ ] **[Simulation]** RL-тренировка: интеграция unitree_rl_mjlab
- [ ] **[Phase 1]** Webots контроллер: полный Zenoh-цикл state → cmd → action
- [ ] **[Phase 2]** Реальный робот: закупка hardware ~$134/робот


## Backlog

- [ ] **[Viewer]** Тепловая карта нагрузки по суставам
- [ ] **[Viewer]** Ghost-режим: показывать целевую позу vs текущую
- [ ] **[UI]** Мобильное приложение (Tauri mobile)
- [ ] **[Protocol]** Запись и воспроизведение сессии (record/replay)
- [ ] **[Docs]** HLD Software обновить до v3.4+ под текущую app/ структуру


## В работе

- [ ] **[Viewer]** Prompt загрузки URDF при подключении к роботу с неизвестной моделью
- [ ] **[URDF]** Качественный URDF с реальными mesh'ами (сейчас placeholder-геометрия)
- [ ] **[URDF]** Масштаб и origin суставов под реальную механику робота


## Готово

- [x] **[Backend]** Декомпозиция Rust-бэкенда на модули (events, robot, zenoh, swarm, commands)
- [x] **[Backend]** Постоянные подписчики `robot/*/state` и `robot/*/descriptor` при connect
- [x] **[Backend]** Команда `read_urdf_file` через Tauri FS + plugin-dialog
- [x] **[Protocol]** RobotDescriptor: поле `urdf_model` для связки робота с URDF-файлом
- [x] **[Protocol]** virtual_robot.py публикует `urdf_model = "roboforge_humanoid_v1"`
- [x] **[URDF]** Создан `roboforge_humanoid_v1.urdf` — 15-DOF гуманоид из Webots proto
- [x] **[URDF]** Загрузка URDF через file picker (useUrdfStore, importUrdf)
- [x] **[URDF]** Кэш URDF по `urdf_model`, клонирование шаблона для каждого робота
- [x] **[Viewer]** URDFLoader вместо procedural buildRobot
- [x] **[Viewer]** Правильный floor offset (0.185 * URDF_SCALE) из цепочки суставов
- [x] **[Viewer]** OrbitControls: zoom, rotate, ограничение полярного угла (5°–85°)
- [x] **[Viewer]** Сохранение позиции камеры в localStorage (rf_cam_solo / rf_cam_swarm)
- [x] **[Viewer]** 3-точечное освещение (key + fill + rim + hemisphere)
- [x] **[Viewer]** Статичный halo (без мигания в animation loop)
- [x] **[Viewer]** CSS overlay-лейблы над роботами (project3D → screen coords)
- [x] **[Viewer]** Solo / Swarm режим, корректный масштаб всех роботов (scale=1.0)
- [x] **[UI]** RobotSelectOverlay — панель выбора робота с карточками
- [x] **[UI]** Выбранный робот: auto-select первого, selectedRobot, selectedId
- [x] **[UI]** Кнопка выбора робота с badge-счётчиком, dev-toggle Solo/Auto/Swarm




%% kanban:settings
```
{"kanban-plugin":"board","list-collapse":[null,null,null]}
```
%%