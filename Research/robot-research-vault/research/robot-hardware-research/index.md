# Исследование: Hardware решения для робота

Исследование микроконтроллеров, SoC и модулей управления для гуманоидного робота 28–30 см, 20 DOF.

## Разделы

- [[artefacts/index|Артефакты]] — сравнения контроллеров, диаграммы, спецификации
- [[notes/index|Заметки]] — рабочие записи и находки
- [[experiments/index|Эксперименты]] — проверка гипотез
- [[decisions/index|Решения]] — принятые архитектурные решения (ADR)

## Ключевые результаты

- Сравнение контроллеров: [[artefacts/comparisons/controller_comparison|Таблица сравнения]]
- Текущий выбор: [[decisions/index|ADR-001 — C/ESP-IDF → Pico 2W]]
- На заметку (future): [[notes/index|SG2002 как автономный мозг v2.5]]
- URDF модель: [[artefacts/urdf/humanoid_v1|humanoid_v1.urdf]] — готова
- Webots импорт: [[notes/webots_import_guide|Руководство по импорту URDF в Webots]]

## Артефакты симуляции

| Файл | Описание | Статус |
|------|----------|--------|
| `simulation/worlds/roboforge.wbt` | Базовый мир Webots | ✅ готов |
| `simulation/controllers/roboforge_controller/` | Контроллер + Zenoh | ✅ готов |
| `Research/.../artefacts/urdf/humanoid_v1.urdf` | URDF модель | ✅ готов |

## Статус

| Раздел | Готовность |
|--------|-----------|
| Сравнение контроллеров | ~60% |
| Решение по Phase 1-2 | ~70% |
| Решение по Phase 3 (автономия) | ~20% |
| URDF модель | ~80% |
| Webots симуляция | ~40% |
| Эксперименты | 0% |
