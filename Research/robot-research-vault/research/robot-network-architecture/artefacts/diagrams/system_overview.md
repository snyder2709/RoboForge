# Обзор системы

Высокоуровневая диаграмма системы управления роботом.

```plantuml
@startuml
actor Operator

node "Brain PC" as BrainPC {
 component Vision
 component Planning
 component Scenario
}

node Robot {
 component Controller
 component Sensors
 component Motors
}

Operator --> BrainPC
BrainPC --> Robot
@enduml
```

## Связанные документы

- [[../index|Индекс артефактов]]
- [[../architectures/zenoh_distributed_architecture|Распределённая архитектура]]
