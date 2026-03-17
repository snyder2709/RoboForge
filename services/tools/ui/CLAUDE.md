# Правила для Claude-агентов — services/tools/ui/

Vue 3 + Vite + Naive UI dev-инструмент для мониторинга и управления роботами.

## Архитектура: MVVM

```
View        src/components/     ← Vue SFC (.vue) — только отображение и пользовательский ввод
ViewModel   src/viewmodels/     ← Composables (use*) — реактивное состояние + бизнес-логика
Model       src/models/         ← Plain JS объекты/фабрики — форма данных без логики
Service     src/services/       ← Внешние источники данных (WebSocket, future: HTTP)
```

### Правила слоёв

- **View** не содержит бизнес-логику. Только `defineProps`, `defineEmits`, вызовы composables.
- **ViewModel** вызывает Services и возвращает реактивные данные. Не импортирует Vue компоненты.
- **Model** — чистые функции/константы, нет реактивности, нет импортов Vue.
- **Service** — синглтоны, изолированы от Vue реактивности (используют `ref` минимально).

## Стек

| Инструмент | Версия | Назначение |
|-----------|--------|------------|
| Vue 3     | ^3.x   | UI framework (Composition API) |
| Vite      | ^8.x   | Dev сервер + бандлер |
| Naive UI  | latest | Компонентная библиотека |
| Three.js  | ^0.183 | 3D визуализация |

## Соглашения

- Только Composition API (`<script setup>`), никакого Options API
- ViewModel composables: `use` префикс, файл = `use{Name}.js`
- Компоненты: `PascalCase.vue`
- Props: camelCase, с типами и required
- Emit: kebab-case событий
- Никакого прямого доступа к WebSocket из компонентов — только через ViewModel
- Стили: `<style scoped>` в компонентах, глобальные только в `App.vue`

## Тёмная тема

Naive UI `darkTheme` + themeOverrides в App.vue. Не переопределять цвета в компонентах.

## WebSocket

WS соединение — синглтон в `src/services/wsService.js`.
Порт: `ws://localhost:8766` (ui_server.py).
Никогда не создавать `new WebSocket()` вне wsService.

## Versioning

Версия в `package.json`. Bump по semver:
- `patch` — bug fix, UI правки
- `minor` — новый компонент, новый сценарий
- `major` — смена протокола WS, breaking change API

## Правило новых сценариев

1. Добавить в `src/models/Scenario.js` → `SCENARIOS` и `SCENARIO_OPTIONS`
2. Реализовать кадры в `services/scenarios/scenarios.py` → `SCENARIOS`
3. Обновить `services/README.md` — секция Scenarios

## Запрещено

- `<script>` без `setup` в SFC
- Мутация props
- Прямой вызов `fetch`/`WebSocket` в компонентах
- `any` типы без комментария `// TODO`
- Файлы с версиями в имени (`component_v2.vue`)
