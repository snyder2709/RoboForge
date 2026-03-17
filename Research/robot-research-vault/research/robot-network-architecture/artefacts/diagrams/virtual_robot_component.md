# Virtual Robot — Компонентная диаграмма (Phase 0)

Визуальное представление `services/virtual_robot/virtual_robot.py`.

## Компонентная диаграмма

```plantuml
@startuml
skinparam componentStyle rectangle

node "Brain PC" as BrainPC {

  component "virtual_robot.py" as VR {
    component "VirtualRobot" as VRClass {
      component "_on_cmd()\nZenoh callback" as OnCmd
      component "_apply_cmd()\nmock servo update" as ApplyCmd
      component "_tick_imu()\nGaussian noise drift" as TickImu
      component "_build_state()\nJSON serialization" as BuildState
      component "asyncio loop\nSTATE_HZ = 2 Hz" as Loop
    }
  }

  component "Robot Agent" as Agent
  component "Swarm Coordinator" as Coord

}

cloud "Zenoh Router\n:7447" as ZR

Agent --> ZR : pub robot/{id}/cmd\nPriority.REAL_TIME + DROP
ZR --> OnCmd : sub robot/{id}/cmd
OnCmd --> ApplyCmd
Loop --> TickImu
Loop --> BuildState
BuildState --> ZR : pub robot/{id}/state\nBestEffort @ 2 Hz
ZR --> Agent : sub robot/{id}/state
Coord --> Agent : swarm/world_state
@enduml
```

## Диаграмма последовательности — один цикл

```plantuml
@startuml
participant "Robot Agent" as A
participant "Zenoh Router" as Z
participant "VirtualRobot" as R

== Команда (по событию) ==
A -> Z : pub robot/{id}/cmd\n{"action":"move_servos","servos":[...],"ts":T}
Z -> R : _on_cmd(sample)
R -> R : _apply_cmd(cmd)\nself.servos[i] = angle

== Состояние (каждые 0.5 с) ==
loop asyncio @ 2 Hz
  R -> R : _tick_imu()\ngauss noise
  R -> R : _build_state()
  R -> Z : pub robot/{id}/state\n{"id":1,"ts":T,"servos":[...],"imu":{...}}
  Z -> A : on_state(sample)
end
@enduml
```

## Топики

| Топик | Тип | Направление |
|-------|-----|-------------|
| `robot/{id}/cmd` | sub | Agent → VirtualRobot |
| `robot/{id}/state` | pub | VirtualRobot → Agent |

## Связанные файлы

- [[../../../../../services/virtual_robot/virtual_robot.py|virtual_robot.py]] (код)
- [[../../../hld/software/hld_software_v3_2|HLD Программный стек v3.2]]
- [[../../../hld/phases/hld_phase1_v1_1|HLD Фаза 1 v1.3]]
