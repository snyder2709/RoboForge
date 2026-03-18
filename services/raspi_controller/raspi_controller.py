"""
RoboForge — Raspberry Pi Controller (EXP-006)
Подписывается на robot/joints через Zenoh и двигает физические сервы через PCA9685.

Топики Zenoh:
  robot/joints        ← команды {joint_name: angle_rad}
  robot/physical_state → состояние {joint_name: angle_rad}

Запуск:
  python3 raspi_controller.py

Требования:
  pip3 install eclipse-zenoh adafruit-circuitpython-pca9685 adafruit-circuitpython-motor
  raspi-config → Interface Options → I2C → Enable
"""

import json
import math
import time
import logging

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s  %(levelname)s  %(message)s",
)
log = logging.getLogger("raspi_ctrl")

# ──────────────────────────────────────────────
# Маппинг: joint_name → (pca_channel, min_deg, max_deg, center_offset)
# center_offset: смещение чтобы 0 rad = нейтральное положение серво
# Редактируй под реально подключённые суставы
# ──────────────────────────────────────────────
JOINT_MAP = {
    # Руки (MG90S) — PCA9685 #1, каналы 0-5
    "left_shoulder_pitch":  (0,   0, 180, 90),
    "left_elbow_pitch":     (1,   0, 180, 90),
    "left_wrist_pitch":     (2,   0, 180, 90),
    "right_shoulder_pitch": (3,   0, 180, 90),
    "right_elbow_pitch":    (4,   0, 180, 90),
    "right_wrist_pitch":    (5,   0, 180, 90),
    # Ноги (GDW DS041MG) — PCA9685 #1, каналы 6-9
    "left_hip_pitch":       (6,   0, 180, 90),
    "left_knee_pitch":      (7,  90, 180, 90),   # колено только сгибается
    "right_hip_pitch":      (8,   0, 180, 90),
    "right_knee_pitch":     (9,  90, 180, 90),
}

# ──────────────────────────────────────────────
# Параметры импульсов (мкс) — подобрать под конкретные сервы
# ──────────────────────────────────────────────
SERVO_MIN_PULSE = 500    # мкс
SERVO_MAX_PULSE = 2500   # мкс


def rad_to_deg(rad: float) -> float:
    return math.degrees(rad)


def init_hardware():
    import board
    import busio
    from adafruit_pca9685 import PCA9685
    from adafruit_motor import servo as adafruit_servo

    i2c = busio.I2C(board.SCL, board.SDA)
    pca = PCA9685(i2c)
    pca.frequency = 50

    servos = {}
    for name, (ch, _, _, _) in JOINT_MAP.items():
        servos[name] = adafruit_servo.Servo(
            pca.channels[ch],
            min_pulse=SERVO_MIN_PULSE,
            max_pulse=SERVO_MAX_PULSE,
        )

    log.info("PCA9685 init OK — %d joints mapped", len(servos))
    return pca, servos


def apply_joints(servos: dict, joints: dict) -> dict:
    """Применить команды и вернуть фактические углы."""
    applied = {}
    for name, angle_rad in joints.items():
        if name not in JOINT_MAP:
            continue
        _, min_deg, max_deg, offset_deg = JOINT_MAP[name]
        target_deg = rad_to_deg(angle_rad) + offset_deg
        target_deg = max(min_deg, min(max_deg, target_deg))
        servos[name].angle = target_deg
        applied[name] = round(math.radians(target_deg - offset_deg), 4)
    return applied


def main():
    pca, servos = init_hardware()

    import zenoh

    session = zenoh.open(zenoh.Config())
    pub = session.declare_publisher("robot/physical_state")
    applied_positions: dict = {}

    def on_joints(sample):
        try:
            joints = json.loads(bytes(sample.payload))
            state = apply_joints(servos, joints)
            applied_positions.update(state)
            log.info("joints applied: %s", {k: f"{v:.2f}" for k, v in state.items()})
        except Exception as exc:
            log.warning("bad payload: %s", exc)

    sub = session.declare_subscriber("robot/joints", on_joints)
    log.info("subscribed to robot/joints — ready")
    log.info("joints mapped: %s", list(JOINT_MAP.keys()))

    try:
        while True:
            if applied_positions:
                pub.put(json.dumps(applied_positions))
            time.sleep(0.1)
    except KeyboardInterrupt:
        log.info("stopping...")
    finally:
        sub.undeclare()
        session.close()
        pca.deinit()
        log.info("done")


if __name__ == "__main__":
    main()
