# Схема подключения — EXP-006 Hardware-in-the-Loop

Связан с: [[../../experiments/EXP-006_hardware_in_the_loop|EXP-006]]

## Сетевая архитектура

```plantuml
@startuml
skinparam backgroundColor #0a0a12
skinparam defaultFontColor #c8d0e0
skinparam nodeBorderColor #1e1e3a
skinparam componentBorderColor #1e1e3a
skinparam arrowColor #7eb8f7

node "Brain PC\n(DESKTOP-2O3KH39)" as BrainPC {
  component "zenohd\n:7447" as Zenohd
  component "Webots R2025a\nroboforge_controller.py" as Webots
  component "ui_server.py\n:8766" as UIServer
  component "Browser UI\n:5173" as BrowserUI
}

node "Raspberry Pi 2 B+\n(Ethernet)" as RPi {
  component "raspi_controller.py" as RaspiCtrl
}

BrowserUI -down-> UIServer : WebSocket
UIServer -down-> Zenohd : Zenoh pub\nrobot/joints
Zenohd -right-> Webots : sub robot/joints
Zenohd -down-> RaspiCtrl : sub robot/joints
Webots -up-> Zenohd : pub robot/state
RaspiCtrl -up-> Zenohd : pub robot/physical_state

note right of Zenohd : UDP multicast\ndiscovery 7447\nEthernet LAN
@enduml
```

## Схема подключения железа (RPi → PCA9685 → Сервы)

```plantuml
@startuml
skinparam backgroundColor #0a0a12
skinparam defaultFontColor #c8d0e0
skinparam nodeBorderColor #1e1e3a
skinparam componentBorderColor #1e1e3a
skinparam arrowColor #7eb8f7

node "Raspberry Pi 2 B+" as RPi {
  component "Pin 1  — 3.3V" as P33
  component "Pin 6  — GND" as PGND
  component "Pin 3  — SDA" as PSDA
  component "Pin 5  — SCL" as PSCL
}

node "PCA9685 PWM Driver" as PCA {
  component "VCC" as PVCC
  component "GND" as PGND2
  component "SDA" as PSDA2
  component "SCL" as PSCL2
  component "V+" as PVP
  component "CH0..CH5\n(руки)" as CHArms
  component "CH6..CH9\n(ноги)" as CHLegs
}

node "USB 5V 2A\n(внешний)" as USB5V {
  component "+ / -" as USBPins
}

node "MG90S ×6\n(руки)" as Arms
node "GDW DS041MG ×4\n(ноги)" as Legs

P33  --> PVCC  : 3.3V [logic power]
PGND --> PGND2 : GND
PSDA --> PSDA2 : SDA  I2C
PSCL --> PSCL2 : SCL  I2C

USBPins --> PVP   : 5V [servo power]
USBPins --> PGND2 : GND [servo power]

CHArms --> Arms : PWM CH0-5
CHLegs --> Legs : PWM CH6-9

note bottom of PCA
  I2C адрес: 0x40
  Частота PWM: 50 Гц
  Pulse: 500–2500 мкс
end note

note bottom of Arms
  Питание серво — только от V+
  НЕ от GPIO пинов RPi!
end note
@enduml
```

## Маппинг суставов

| Joint | Тип | CH | Диапазон | Центр (0 rad) |
|-------|-----|----|----------|---------------|
| left_shoulder_pitch | MG90S | 0 | 0–180° | 90° |
| left_elbow_pitch | MG90S | 1 | 0–180° | 90° |
| left_wrist_pitch | MG90S | 2 | 0–180° | 90° |
| right_shoulder_pitch | MG90S | 3 | 0–180° | 90° |
| right_elbow_pitch | MG90S | 4 | 0–180° | 90° |
| right_wrist_pitch | MG90S | 5 | 0–180° | 90° |
| left_hip_pitch | GDW DS041MG | 6 | 0–180° | 90° |
| left_knee_pitch | GDW DS041MG | 7 | 90–180° | 90° |
| right_hip_pitch | GDW DS041MG | 8 | 0–180° | 90° |
| right_knee_pitch | GDW DS041MG | 9 | 90–180° | 90° |
