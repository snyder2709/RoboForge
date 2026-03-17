# RoboForge

Проект по разработке системы управления гуманоидными роботами с распределённой архитектурой.

## Пути

- Vault (знания + HLD): `Research/robot-research-vault/`
- Исследовательские артефакты: `Research/robot-research-vault/research/`
- HLD документы: `Research/robot-research-vault/hld/`

## Скилы

- [[Research/robot-research-vault/skills/research_methodology|research_methodology]] — методика технического исследования архитектуры

## Правила для агентов

- [[Research/robot-research-vault/claude_rules|claude_rules]] — правила генерации артефактов, PlantUML, версионирование

## Статус проекта (2026-03-17)

| Артефакт | Версия | Готовность | Статус |
|----------|--------|------------|--------|
| HLD Железо | v1.2 | ~88% | актуально |
| HLD Программный стек | v3.2 | ~66% | черновик |
| HLD Фаза 1 | v1.3 | ~82% | черновик |
| Исследование сетевой архитектуры | — | ~80% | завершено |
| Исследование hardware решений | — | ~50% | в процессе |

### Brain PC (dev / run)

| Параметр | Значение |
|----------|---------|
| Устройство | Lenovo LOQ 15IRX9 (DESKTOP-2O3KH39) |
| CPU | Intel Core i7-13650HX (14c/20t, 6P+8E) |
| RAM | 16 GB |
| GPU | NVIDIA RTX 3050 6GB Laptop |
| ОС | Windows 11 x64 |

### Готовность к разработке

| Фаза | Статус | Что нужно |
|------|--------|----------|
| Phase 0 — виртуальные роботы | **🔴 Q-01: написать Virtual Robot** | eclipse-zenoh код |
| Phase 1 — Webots | ~2-3 нед | URDF модель |
| Phase 2 — реальный робот | ~4-8 нед после Phase 1 | Закупка ~$134/робот, 3D-печать, IK |

### HLD навигатор

- [[Research/robot-research-vault/hld/index|HLD Навигатор]]

### Исследования

- [[Research/robot-research-vault/research/robot-network-architecture/index|Сетевая архитектура]]
- [[Research/robot-research-vault/research/robot-hardware-research/index|Hardware решения (контроллеры)]]

## Организация vault

```
Research/robot-research-vault/
├── hld/                          ← проектные решения
│   ├── hardware/hld_hardware_v1_2.md   ← актуально
│   ├── software/hld_software_v3_2.md   ← черновик
│   └── phases/hld_phase1_v1_1.md       ← черновик v1.3
└── research/
    ├── robot-network-architecture/
    │   └── artefacts/
    │       ├── architectures/    ← архитектурные варианты
    │       ├── flows/            ← потоки данных
    │       ├── protocols/        ← сравнение протоколов
    │       └── diagrams/         ← системные диаграммы
    └── robot-hardware-research/
        ├── notes/                ← заметки и находки
        ├── artefacts/
        │   └── comparisons/      ← сравнение контроллеров
        ├── experiments/          ← проверка гипотез
        └── decisions/            ← ADR (архитектурные решения)
```
