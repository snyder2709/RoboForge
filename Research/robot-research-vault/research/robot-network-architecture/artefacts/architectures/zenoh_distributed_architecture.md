# Распределённая архитектура робота (Zenoh)

Полностью распределённая архитектура на основе протокола Zenoh. Все компоненты взаимодействуют через центральный Zenoh Router.

```plantuml
@startuml
actor Operator
cloud "Zenoh Router" as Router

node "Client PC" {
 component "Brain Client" as BrainClient
 component "UI" as UI
}

node "Services" {
 component "Vision Service" as VisionService
 component "Planning Service" as PlanningService
 component "Scenario Service" as ScenarioService
}

node "Robot 1" {
 component "Zenoh Gateway" as ZenohGateway
 component "Robot Controller" as RobotController
}

Operator --> UI
UI --> Router
BrainClient --> Router

Router --> VisionService
Router --> PlanningService
Router --> ScenarioService
Router --> ZenohGateway
ZenohGateway --> RobotController
@enduml
```

## Характеристики

| Параметр | Значение |
|---|---|
| Протокол | Zenoh |
| Топология | Централизованный роутер |
| Масштабируемость | Высокая |
| Latency | Низкая |

## Связанные документы

- [[hybrid_zenoh_udp_architecture|Гибридная архитектура (Zenoh + UDP)]]
- [[../protocols/protocol_comparison|Сравнение протоколов]]
