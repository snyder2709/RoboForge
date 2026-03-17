"""
RoboForge Webots Controller — Phase 1
Управляет роботом в симуляторе Webots, получает команды через Zenoh.

Топики Zenoh:
  robot/joints   — входящие команды (JSON: {"joint_name": angle_rad, ...})
  robot/state    — исходящее состояние (JSON: {"joint_name": angle_rad, ...})
"""

import json
from controller import Robot

# Имена суставов должны совпадать с URDF humanoid_v1.urdf
JOINT_NAMES = [
    "head_pan",
    "left_shoulder_pitch", "left_elbow_pitch", "left_wrist_pitch",
    "right_shoulder_pitch", "right_elbow_pitch", "right_wrist_pitch",
    "left_hip_yaw", "left_hip_pitch", "left_knee_pitch", "left_ankle_pitch",
    "right_hip_yaw", "right_hip_pitch", "right_knee_pitch", "right_ankle_pitch",
]

robot = Robot()
timestep = int(robot.getBasicTimeStep())

# Инициализация моторов — T-поза (все углы 0)
motors = {}
sensors = {}
for name in JOINT_NAMES:
    motor = robot.getDevice(name)
    if motor:
        motor.setPosition(0.0)
        motors[name] = motor

        sensor = robot.getDevice(name + "_sensor")
        if sensor:
            sensor.enable(timestep)
            sensors[name] = sensor

# Zenoh подключение (опционально — Phase 1 интеграция)
_zenoh_enabled = False
try:
    import zenoh

    _session = zenoh.open(zenoh.Config())
    _target_positions = {}

    def _on_joint_cmd(sample):
        try:
            cmd = json.loads(bytes(sample.payload))
            _target_positions.update(cmd)
        except Exception as e:
            print(f"[controller] bad cmd: {e}")

    _sub = _session.declare_subscriber("robot/joints", _on_joint_cmd)
    _pub = _session.declare_publisher("robot/state")
    _zenoh_enabled = True
    print("[controller] Zenoh connected")
except ImportError:
    print("[controller] zenoh not installed — running standalone")
except Exception as e:
    print(f"[controller] Zenoh error: {e} — running standalone")


def apply_positions(target: dict):
    for name, angle in target.items():
        if name in motors:
            motors[name].setPosition(float(angle))


def read_state() -> dict:
    state = {}
    for name, sensor in sensors.items():
        state[name] = round(sensor.getValue(), 4)
    return state


# Главный цикл симуляции
while robot.step(timestep) != -1:
    if _zenoh_enabled and _target_positions:
        apply_positions(_target_positions)
        _target_positions.clear()

        state = read_state()
        _pub.put(json.dumps(state).encode())
