# RoboForge — HLD Навигатор

## Документы

| Документ | Версия | Готовность | Статус | Обновлён |
|----------|--------|------------|--------|----------|
| [[hardware/hld_hardware_v1_2\|HLD Железо]] | v1.2 | ~88% | актуально | 2026-03-16 |
| [[software/hld_software_v3_2\|HLD Программный стек]] | v3.2 | ~66% | черновик | 2026-03-16 |
| [[phases/hld_phase1_v1_1\|HLD Фаза 1]] | v1.3 | ~82% | черновик | 2026-03-17 |

Статусы: `черновик` | `актуально` | `архив` | `заморожен`

## Зависимости

```
HLD Фаза 1 v1.2  →  HLD Программный стек v3.2
HLD Программный стек v3.2  →  Исследование сетевой архитектуры
HLD Железо v1.2  →  независим
```

## Готовность к разработке

| Фаза | Статус | Блокеры |
|------|--------|---------|
| Phase 0 — виртуальные роботы | **🔴 БЛОКЕР** | Q-01: обновить Virtual Robot под eclipse-zenoh |
| Phase 1 — Webots симулятор | ~2-3 нед | URDF модель |
| Phase 2 — реальное железо | ~4-8 нед после Phase 1 | Закупка (~$134/робот), 3D-печать, IK |

## Открытые вопросы

- [ ] IK алгоритм для 20 DOF (Phase 2 blocker)
- [ ] URDF для Webots (28-30 см гуманоид, адаптация Poppy)
- [ ] zenoh-pico конфигурация Pico 2W / ESP32 (QoS, discovery)
- [ ] Zenoh конфигурация для роя N > 4 роботов

## Связанные исследования

- [[../research/robot-network-architecture/index|Исследование: сетевая архитектура]]
- [[../research/robot-network-architecture/artefacts/architectures/hybrid_zenoh_udp_architecture|Гибридная архитектура Zenoh+UDP]] — источник для SW HLD v3.x (архив)
- [[../research/robot-network-architecture/artefacts/protocols/protocol_comparison|Сравнение протоколов]]
- [[../research/robot-hardware-research/index|Исследование: Hardware решения]] — источник для HLD Железо v1.2
