# Гибридная архитектура (Zenoh + UDP)

Гибридная архитектура: сервисный уровень использует Zenoh, управление роботом передаётся через UDP для минимизации задержки.

```plantuml
@startuml
cloud "Zenoh Router" as ZenohRouter

node "Brain PC" {
 component "Scenario Service" as ScenarioService
 component "Planning Service" as PlanningService
 component "Robot Gateway" as RobotGateway
}

node "Robot" {
 component "UDP Receiver" as UDPReceiver
 component "Motion Buffer" as MotionBuffer
 component "Robot Controller" as RobotController
}

ScenarioService --> ZenohRouter
PlanningService --> ZenohRouter
ZenohRouter --> RobotGateway
RobotGateway --> UDPReceiver
UDPReceiver --> MotionBuffer
MotionBuffer --> RobotController
@enduml
```

## Характеристики

| Параметр | Значение |
|---|---|
| Протокол (сервисы) | Zenoh |
| Протокол (управление) | UDP |
| Масштабируемость | Средняя |
| Latency управления | Минимальная |

## Обоснование выбора

- UDP обеспечивает минимальные задержки для передачи движений
- Zenoh обеспечивает надёжную маршрутизацию между сервисами
- Motion Buffer защищает от потери пакетов UDP

## Связанные документы

- [[zenoh_distributed_architecture|Распределённая архитектура (Zenoh)]]
- [[../protocols/protocol_comparison|Сравнение протоколов]]
- [[../flows/scenario_execution_flow|Поток выполнения сценария]]
