# RoboForge — HLD Навигатор

## Документы

| Документ | Версия | Готовность | Статус | Обновлён |
|----------|--------|------------|--------|----------|
| [[hardware/hld_hardware_v1_1\|HLD Железо]] | v1.1 | ~85% | актуально | — |
| [[software/hld_software_v3_0\|HLD Программный стек]] | v3.0 | ~73% | черновик | 2026-03-16 |
| [[phases/hld_phase1_v1_1\|HLD Фаза 1]] | v1.1 | ~76% | черновик | 2026-03-16 |

Статусы: `черновик` | `актуально` | `архив` | `заморожен`

## Зависимости

```
HLD Фаза 1 v1.1  →  HLD Программный стек v3.0
HLD Программный стек v3.0  →  Исследование сетевой архитектуры
HLD Железо v1.1  →  независим
```

## Готовность к разработке

| Фаза | Статус | Блокеры |
|------|--------|---------|
| Phase 0 — виртуальные роботы | **ГОТОВ К СТАРТУ** | — |
| Phase 1 — Webots симулятор | ~2-3 нед | Robot Gateway, URDF модель |
| Phase 2 — реальное железо | ~4-8 нед после Phase 1 | Закупка (~$134/робот), 3D-печать, IK |

## Открытые вопросы

- [ ] UDP формат пакета: бинарный или JSON? Финальная схема 20 DOF
- [ ] Robot Gateway API-контракт (Zenoh → UDP)
- [ ] IK алгоритм для 20 DOF (Phase 2 blocker)
- [ ] URDF для Webots (28-30см гуманоид, Poppy-адаптация)
- [ ] Zenoh конфигурация для роя N > 4 роботов

## Связанные исследования

- [[../research/robot-network-architecture/index|Исследование: сетевая архитектура]]
- [[../research/robot-network-architecture/artefacts/architectures/hybrid_zenoh_udp_architecture|Гибридная архитектура Zenoh+UDP]] — источник для SW HLD v3.0
- [[../research/robot-network-architecture/artefacts/protocols/protocol_comparison|Сравнение протоколов]]
