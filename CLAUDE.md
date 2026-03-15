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

## Статус проекта (2026-03-16)

| Артефакт | Версия | Готовность | Статус |
|----------|--------|------------|--------|
| HLD Железо | v1.1 | ~85% | актуально |
| HLD Программный стек | v3.0 | ~73% | черновик |
| HLD Фаза 1 | v1.1 | ~76% | черновик |
| Исследование сетевой архитектуры | — | ~80% | завершено |

### Готовность к разработке

| Фаза | Статус | Что нужно |
|------|--------|----------|
| Phase 0 — виртуальные роботы | **ГОТОВ К СТАРТУ** | — |
| Phase 1 — Webots | ~2-3 нед | Robot Gateway + URDF |
| Phase 2 — реальный робот | ~4-8 нед после Phase 1 | Закупка ~$134/робот, 3D-печать, IK |

### HLD навигатор

- [[Research/robot-research-vault/hld/index|HLD Навигатор]]

## Организация vault

```
Research/robot-research-vault/
├── hld/                          ← проектные решения
│   ├── hardware/hld_hardware_v1_1.md
│   ├── software/hld_software_v3_0.md
│   └── phases/hld_phase1_v1_1.md
└── research/
    └── robot-network-architecture/
        └── artefacts/
            ├── architectures/    ← архитектурные варианты
            ├── flows/            ← потоки данных
            ├── protocols/        ← сравнение протоколов
            └── diagrams/         ← системные диаграммы
```
