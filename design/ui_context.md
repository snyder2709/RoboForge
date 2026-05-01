# RoboForge — UI Design Context

Контекст для проектирования главного экрана desktop-приложения. Документ самодостаточный — можно скармливать любому LLM с нуля.

---

## 1. Проект

**RoboForge** — система управления гуманоидными роботами 28см. Монорепо включает Python services на шине Zenoh, симуляцию в Webots, и Tauri desktop+mobile приложение. Сейчас Phase 1 (Webots-контроллер работает, debug UI есть). Phase 2 — реальные роботы (~$134/штука, 3D-печать корпуса).

**Brain PC dev-машина:** Lenovo LOQ 15IRX9, i7-13650HX, RTX 3050 6GB, Win11. То есть приложение должно работать на mid-range железе — без тяжёлых WebGL эффектов.

**Язык интерфейса:** русский (основной) + английский (вторичный). Все технические термины и токены остаются на английском.

---

## 2. Задача

Спроектировать главный экран desktop-приложения, размер `1440×900`, Tauri-окно. Экран — командный центр: пользователь видит активного робота или рой в 3D-вьювере и запускает сценарии движения.

**Композиционный референс:** PUBG main menu — кинематографичный hero shot в центре, UI-панели скрыты по умолчанию, выезжают по hover/click. Сильная глубина, атмосфера, тёмный фон.

**Тональный референс:** продуктовый hero shot Apple встречает game launcher. Не геймерская агрессия, а спокойная сосредоточенность с акцентами.

---

## 3. Палитра (Claude warm dark)

### Surfaces
| Token | Hex | Use |
|---|---|---|
| `bg-deep` | `#0F0E0D` | основной фон viewer'а |
| `surface-dark` | `#1C1917` | панели, бары, карточки |
| `surface-hover` | `#2A2724` | hover-состояния |
| `surface-disabled` | `#3A3733` | disabled, дивайдеры |

### Brand
| Token | Hex | Use |
|---|---|---|
| `coral-primary` | `#E8784A` | CTA, active, halo, solo-тэг |
| `coral-light` | `#F5C4B3` | текст на coral fills |
| `coral-dim` | `#FBE2D2` | highlight внутри coral элементов |

### Cool counterpoint (suspenders для роботов и swarm)
| Token | Hex | Use |
|---|---|---|
| `joint-blue` | `#3A6FAF` | суставы роботов, swarm icon |
| `joint-highlight` | `#5B95D8` | блик на суставе, swarm-mode dot |
| `swarm-light` | `#B5D4F4` | текст swarm-тэгов |

### Text
| Token | Hex | Use |
|---|---|---|
| `text-primary` | `#F5F0EB` | основной (warm cream) |
| `text-secondary` | `#9E9A93` | вторичный |
| `text-tertiary` | `#5F5E5A` | хинты |
| `text-disabled` | `#3A3733` | disabled |

### Robot body (для viewer)
| Token | Hex | Use |
|---|---|---|
| `robot-shell` | `#EAE7E1` | основной корпус |
| `robot-shell-stroke` | `#2A2724` | контур деталей |
| `robot-accent-gray` | `#9E9A93` | стыки, шея, бёдра |
| `robot-belt` | `#D6D2CB` | плечевой ярем, ремень |

### Status
| Token | Hex | Use |
|---|---|---|
| `success` | `#7AB87A` | online, signal good |
| `danger` | `#E63E3E` | E-STOP outline |
| `danger-fill` | `#9E2828` | E-STOP background |
| `warning` | `#E8B89E` | зарядка, attention |

---

## 4. Типографика

- **Семейство:** Anthropic Sans / system-ui sans-serif
- **Веса:** только 400 regular и 500 medium. **Никаких 600/700** — выглядят тяжело на тёмном.
- **Кейс:** sentence case везде. Никакого Title Case и ALL CAPS, кроме маленьких uppercase лейблов с `letter-spacing: 1.6-2.0` (типа `ACTIVE ROBOT`).
- **Размеры:**
  - 18px / 500 — section headers
  - 14px / 500 — card titles, primary CTA labels
  - 13px / 500 — bottom bar values
  - 11px / 400 — descriptions, secondary metadata
  - 10px / 400 — uppercase micro-labels
  - 9px / 500 — emergency micro-label

---

## 5. Information Architecture (КРИТИЧНОЕ РЕШЕНИЕ)

**Одна навигация, не две.** В прошлой итерации были дублирующие уровни (rail с пунктами Scenarios+Presets+Manual + панель с теми же табами). Это баг.

### Финальная IA:

**Левый thin rail (56px) = top-level разделы приложения:**

1. **Library** — сценарии, пресеты, manual control (внутри табы)
2. **Robots** — список физических роботов, состояние, calibration
3. **Mission control** — карта waypoints, патрули, многоэтапные миссии (out of scope сейчас, но место зарезервировано)
4. **Simulation** — Webots, viewer settings
5. **Logs** — телеметрия и журналы
6. **Settings** (внизу рейла, отдельный визуальный блок)

3px coral-индикатор слева от иконки активной секции.

**Правый thin rail (56px) = инспекторы активного робота / роя:**
- Telemetry (графики)
- Joints (углы суставов)
- IMU
- Vision (камера)

**Левая панель (340px) выезжает по hover/click на rail-иконку.**

**Правая панель** — то же самое, выезжает с правой стороны.

---

## 6. 3D Viewer behavior

Центральная зона — Three.js viewer с реальной 3D-моделью робота, синхронизация позы с физическим роботом по Zenoh.

### Solo mode
- Камера: фронтальная, eye-level робота, FOV ~35°
- Робот занимает **60% высоты экрана**
- Coral halo плотный за силуэтом, диаметр ≈ ширина робота × 4
- Под роботом одна тень + лёгкая coral-лужа на полу
- Edge vignette затемняет углы

### Swarm mode
- Камера приподнимается: pitch ~30°, отъезд назад
- FOV ~50°
- **Дальние роботы ВЫШЕ на экране** (меньше y координата) — критично для перспективы. С приподнятой камерой далёкие объекты выше в кадре.
- Halo расширяется и охватывает всю формацию
- Появляется отражающий пол с уходящей вдаль перспективной сеткой — линии **должны сходиться к vanishing point**, не быть параллельными горизонталями
- Анимация перехода solo↔swarm: 800ms ease-out

### Перепроверить при рендере
- Ноги робота приземляются ровно на координату тени (не парят над полом — это была ошибка)
- Каждая тень совпадает с центром стопы по x

---

## 7. Tag system

Только два тэга, цветовая кодировка:

| Tag | Color | Pill bg | Pill text | Meaning |
|---|---|---|---|---|
| `solo` | coral | `#E8784A` 0.18 + 0.5 stroke 0.5 | `#F5C4B3` | для одного робота |
| `swarm` | blue | `#5B95D8` 0.16 + stroke 0.45 | `#B5D4F4` | для 2+ роботов |

Сценарий **может нести оба тэга** одновременно → работает в обоих режимах. Тогда показываем оба pill-а подряд.

В русской локализации — без перевода, кириллицей по транслитерации: «соло» / «рой».

Цвета тэгов = цвета режимов в viewer'е (coral solo, blue swarm). Единый визуальный язык.

---

## 8. Иерархия Scenarios → Presets → Manual

Три уровня абстракции — важно не путать пользователя.

### Preset
Атомарное движение: помахать, поклониться, шаг вальса. Длительность 2-10s. Без логики, без триггеров. Можно записать через manual mode и сохранить.

### Scenario
Композиция пресетов + timeline + опционально triggers/waypoints. От 30s до бесконечности (патрули, follow-the-leader).

### Manual control
Прямое управление одним роботом (WASD/joystick/gamepad). Hold-to-record для записи в новый пресет. **ТОЛЬКО SOLO**.

Принципиально: одновременное ручное управление несколькими роботами небезопасно (рассинхронизация, коллизии). Manual = solo захват.

**Поведение Manual в табах Library:**
- Доступен когда swarm-сценарий не активен
- **Disabled** с tooltip «Available in solo mode» когда активен swarm
- Вход в manual через подтверждение останавливает любой активный сценарий

---

## 9. Card states

Карточка сценария всегда в одном из четырёх состояний — визуально различимых:

1. **Available** — обычный вид, hover активен (scale 1.02 + усиление top edge highlight)
2. **Locked** — недостаточно роботов онлайн. Текст «Needs 1 more robot online» вместо CTA. Карточка с opacity 0.55. Бэдж `LOCKED` справа сверху. Тэг своего режима блёкнет.
3. **Running** — coral border 1.2px, лёгкая пульсация, кнопка превращается в Stop, `RUNNING` бэдж пульсирует
4. **Incompatible** — выбран solo, а сценарий только swarm — затемнено + блокер-тэг

---

## 10. Liquid glass card spec

Полупрозрачный стеклянный материал в духе Apple Liquid Glass.

```
background: linear-gradient(180deg,
  rgba(255,255,255,0.06) 0%,
  rgba(255,255,255,0.02) 50%,
  rgba(0,0,0,0.18) 100%
);
border: 0.5px solid rgba(255,255,255,0.08);
border-radius: 14px;
```

**Top edge highlight** (имитация преломлённого света):
```
0.6px line, x: 14% to 94% width, top edge,
stroke: rgba(255,255,255,0.14-0.18)
```

**Running variant:**
```
background: linear-gradient(180deg, 
  rgba(232,120,74,0.14) 0%, 
  rgba(232,120,74,0.04) 100%);
border: 1.2px solid #E8784A;
```

### Анатомия карточки (фиксированная высота 92px)

```
┌────────────────────────────────────────┐
│ ┌──────┐  Title (14/500)         [✦]  │
│ │ thumb│  Description (11)             │
│ │ 68×68│  [solo][swarm] meta...        │
│ └──────┘                                │
└────────────────────────────────────────┘
```

- Thumbnail 68×68: dark fill `#0E0D0C`, stroke 0.5px `#3A3733`, силуэт ключевого кадра действия. **Тщательно выравнивать содержимое — в прошлой итерации боты вылазили за пределы.**
- Title: 14px / 500 / `text-primary`
- Description: 11px / 400 / `text-secondary`, одна строка
- Tags inline → метаданные одной строкой
- Status badge (NEW / FAVORITE / RUNNING / LOCKED) — top-right corner, 24×12 pill

### Метаданные — ФИКСИРОВАННЫЙ шаблон

```
[robot count] · [duration] · [status or last run]
```

Примеры:
- `3 robots · 30s · running`
- `2 robots · 45s · last run yesterday`
- `1 robot · 6s · 3× this week`
- `4 robots · 2 min · locked`

В прошлой итерации шаблоны плавали — фиксируем.

---

## 11. Components

### Top bar — 44px tall
- Logo (20×20 coral square с robot-icon внутри) + «RoboForge» 14/500 + version chip 11/400 secondary
- Native Tauri window controls справа: settings gear, minimize, maximize, close (close — coral X)

### Left rail — 56px wide
- 5-6 иконок top-level разделов
- 3px coral индикатор слева от активной
- Settings внизу отдельной группой
- Опциональный hover-tooltip справа от иконки (76×24 pill)

### Left panel — 340px wide, frosted dark
```
background: linear-gradient(90deg,
  rgba(28,25,23,0.94) 0%,
  rgba(28,25,23,0.78) 100%);
border-right: 0.5px coral 0.25 opacity
```

Содержимое:
- Section caps label (10/400 letter-spacing 2)
- Section title (18/500)
- Tabs (если применимо)
- Search bar (32px tall, dark fill, search icon left)
- Filter chips
- Cards list (vertical, gap ~12px)

### Right rail / panel — зеркально левому
Содержит инспекторы вместо контента.

### Bottom bar — 76px tall, всегда виден
Coral hairline сверху (0.6px, 0.4 opacity).

**Solo mode layout:**
```
[avatar+name]  [BATTERY]  [LINK]  [MODE]  ........  [E-STOP]
```

**Swarm mode layout:**
```
[3 dots+swarm name]  [LOWEST BATTERY]  [SYNC DRIFT]  [MODE]  ...  [STOP ALL]
```

Каждый блок: маленький uppercase label сверху + значение снизу.

### E-STOP button
- 124×48 (для swarm-варианта «STOP ALL» проверить ширину — letter-spacing 3 + font-size 15 на 8 символах может переполниться, рассмотреть letter-spacing 2 или font-size 14)
- Background `#9E2828`, border 1.5px `#E63E3E`
- 3px lighter red strip на левом edge
- Power icon: dark circle 9px с red dot 5px внутри
- Two-line: 9px label «EMERGENCY» / 15px «STOP» или «STOP ALL»

### Active scenario banner — top center, только когда running
- Pill 280×38, position fixed-style сверху по центру
- Status dot (coral pulse) + scenario name + progress timer
- 220×2 progress bar внизу pill-а
- **НЕ дублировать с running-карточкой в панели** — показывать в одном месте. Если панель открыта и видна running-карточка → банер не нужен. Если панель закрыта → банер виден.

---

## 12. Robot 3D model — анатомия для viewer

Гуманоид 28см real-life. В hero view рендер ~540px tall:

| Часть | Размер | Цвет |
|---|---|---|
| Head | 100×80 rounded rect | `#EAE7E1` shell |
| Visor strip | 76×22 | `#0E0D0C` с 2 голубыми сенсорами 3px |
| Antenna LED | 3px circle сверху | `#E8784A` |
| Neck | 24×20 | `#9E9A93` |
| Shoulder yoke | 220×26 | `#D6D2CB` |
| Joints (плечи, локти, бёдра, колени) | balls 14-20px | `#3A6FAF` core, `#5B95D8` highlight, `#0E0D0C` socket |
| Torso | trapezoidal hexagon | `#EAE7E1` |
| Chest core | circle 18px outer / 8px coral inner | `#E8784A` glow |
| Arms | upper + elbow + forearm + hand 26×32 | shell |
| Hand | 3-fingered gripper indication (3 vertical lines) | `#9E9A93` |
| Hips/waist | gray segment + light belt | `#9E9A93` + `#D6D2CB` |
| Legs | thigh + knee + shin + foot | shell |
| Foot | converging trapezoid 60×26 | `#2A2724` |

В swarm mode роботы scale 0.65 (задние) и 0.95 (передний).

---

## 13. Критичные баги прошлых итераций — НЕ ПОВТОРЯТЬ

### Геометрия
1. **Роботы парили в воздухе** — feet at y≈676 при floor shadows at y≈780. Зазор 80-100px. Проверять, что y(feet) == y(shadow center).
2. **Перспектива перевёрнута** — задние роботы должны быть ВЫШЕ на экране, не ниже. Меньше y = дальше при tilt-down camera.
3. **Превью в карточках вылазили** — `<symbol viewBox="120 280">` при scale 0.18 рендерил бота от y=442 до 492, а thumbnail container y=378-446. Всегда сначала вычислять финальный bounding box после `translate × scale`, потом сверять с container'ом.
4. **Floor grid не сходился к vanishing point** — параллельные горизонтали + почти-вертикали не дают перспективу. Линии должны сходиться: горизонтали — к одной точке схода, продольные — к другой.

### Цвет
5. **Halo с coral→blue давал мутный коричневый в середине** — gradient stops `coral 0.22` → `blue 0.08` смешиваются в грязный. Решение: радиальный coral до края + отдельным слоем soft cool rim-light синим, не в одном градиенте.

### Структура
6. **Двойная навигация** — rail с теми же названиями что и табы внутри панели. Решено выше — rail = top-level разделы, табы = подразделы внутри секции.
7. **Двойная индикация disabled** — иконка в рейле faded + таб faded. Достаточно одной.
8. **Дубликат running-info** — баннер сверху + running-карточка в панели. Одна точка правды.

### Тексты
9. **Терминология плавает** — "Active swarm" / "Scenario running · swarm" / "Swarm mode" — выбрать одну. Рекомендую: «Swarm» как mode label, «Synchronized march» как scenario name, «3 of 4 robots active» как swarm composition. Не смешивать.
10. **Метаданные карточек разноформатные** — фиксирован шаблон `[count] · [duration] · [status]`.
11. **«In tolerance» избыточно** — если индикатор зелёный, текст не нужен. Цвет уже это сообщает.

### Иконография
12. **Scenarios иконка была play (▶)** — путаница с «запустить». Использовать filmstrip или layered cards.
13. **Manual иконка нечитаемая** — нужна явная (gamepad, joystick, hand).

---

## 14. Out of scope сейчас

- Timeline editor сценариев (drag-drop пресетов на тайм-линию) — отдельный full-screen вид
- Map waypoints для patrol — отдельный screen с топ-даун видом
- Manual mode UI (joystick, hold-to-record экран)
- Settings, Logs детальные экраны (рейл-секции зарезервированы, но дизайн позже)
- Mobile/tablet версии (Tauri mobile)
- Onboarding / pairing нового робота
- Battery charging flow

---

## 15. Что нужно от следующей итерации

Перепроектировать главный экран swarm-mode с фиксами всех багов выше. Минимальный output:

- Mockup 1440×900 с:
  - Корректной перспективой (3 робота, дальние выше на экране)
  - Роботы стоят на полу (не парят)
  - Floor grid с vanishing point
  - Открытая левая панель Library/Scenarios с 5 карточками во всех 4 состояниях (Available, Running, Locked, NEW)
  - Корректно выровненные превью на карточках
  - Чистый coral halo (без мутной середины)
  - Рейл-навигация без дубликатов с табами
  - Single-source-of-truth для running-сценария
- Список design tokens готовых к импорту в код (CSS custom properties или JSON)
- Спецификация анимаций solo↔swarm transition (timeline 800ms)

Опционально:
- Интерактивный HTML prototype с реальными hover-states
- Solo-mode вариант того же экрана
- Состояние «выбрана карточка, готовится к запуску» (preview before play)

---

## 16. Технические констрейнты

- Tauri webview (Chromium-based, актуальная версия)
- Three.js для 3D viewer — модель должна быть лёгкой (target 60fps на RTX 3050 Laptop)
- Zenoh-ipc мост между frontend и Python-сервисами
- Поддержка Windows 11 native (тёмные оконные контролы)
- Ресайз окна — но 1440×900 — основной target

---
