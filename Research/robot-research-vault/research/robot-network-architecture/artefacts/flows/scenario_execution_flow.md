# Поток выполнения сценария

Последовательность взаимодействия компонентов при запуске и выполнении двигательного сценария на роботе.

```plantuml
@startuml
actor Operator
participant "Brain Client"
participant "Scenario Service"
participant "Robot Gateway"
participant "Robot Controller"

Operator -> "Brain Client": запуск сценария
"Brain Client" -> "Scenario Service": scenario_start
"Scenario Service" -> "Robot Gateway": start command
"Robot Gateway" -> "Robot Controller": execute scenario

loop сегменты движения
"Scenario Service" -> "Robot Gateway": motion segment
"Robot Gateway" -> "Robot Controller": segment
end

"Robot Controller" -> "Robot Gateway": state update
"Robot Gateway" -> "Scenario Service": telemetry
@enduml
```

## Связанные документы

- [[../architectures/hybrid_zenoh_udp_architecture|Гибридная архитектура (Zenoh + UDP)]]
- [[../index|Индекс артефактов]]
