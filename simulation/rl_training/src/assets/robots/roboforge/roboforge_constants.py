"""
RoboForge Humanoid v2 — константы для RL тренировки
21 DOF: 12x MG996R (ноги) + 10x SG92R (руки, голова)

MG996R: stall torque ~0.98 Nm @ 6V
SG92R:  stall torque ~0.245 Nm @ 5V
"""
import os

URDF_PATH = os.path.join(
    os.path.dirname(__file__),
    "../../../../..",
    "Research/robot-research-vault/research/robot-hardware-research/artefacts/urdf/humanoid_v2.urdf"
)

# Порядок суставов — совпадает с каналами PCA9685
JOINT_NAMES = [
    # Ноги (MG996R)
    "left_hip_yaw",
    "left_hip_roll",
    "left_hip_pitch",
    "left_knee_pitch",
    "left_ankle_pitch",
    "left_ankle_roll",
    "right_hip_yaw",
    "right_hip_roll",
    "right_hip_pitch",
    "right_knee_pitch",
    "right_ankle_pitch",
    "right_ankle_roll",
    # Руки (SG92R)
    "left_shoulder_pitch",
    "left_shoulder_roll",
    "left_elbow_pitch",
    "left_wrist_pitch",
    "right_shoulder_pitch",
    "right_shoulder_roll",
    "right_elbow_pitch",
    "right_wrist_pitch",
    # Голова (SG92R)
    "head_pan",
]

NUM_ACTIONS = len(JOINT_NAMES)  # 21

# PD gains — намного ниже чем у G1 (0.98 Nm vs 25-139 Nm у G1)
# Ноги (MG996R)
KP_LEGS = 8.0
KD_LEGS = 0.3
# Руки и голова (SG92R)
KP_ARMS = 3.0
KD_ARMS = 0.1

KP = {j: KP_LEGS for j in JOINT_NAMES[:12]}
KP.update({j: KP_ARMS for j in JOINT_NAMES[12:]})

KD = {j: KD_LEGS for j in JOINT_NAMES[:12]}
KD.update({j: KD_ARMS for j in JOINT_NAMES[12:]})

# Начальная поза — стоя, колени слегка согнуты
DEFAULT_JOINT_POS = {j: 0.0 for j in JOINT_NAMES}
DEFAULT_JOINT_POS.update({
    "left_hip_pitch":  -0.15,
    "right_hip_pitch": -0.15,
    "left_knee_pitch":  0.30,
    "right_knee_pitch": 0.30,
    "left_ankle_pitch": -0.15,
    "right_ankle_pitch": -0.15,
})

# Высота торса над землёй в позе стоя (м)
BASE_HEIGHT = 0.145

# Масса робота (кг) — обновлена для MG996R (55г × 12 + SG92R 9г × 10)
TOTAL_MASS_KG = 0.46


def get_roboforge_robot_cfg():
    """Возвращает конфиг робота для mjlab."""
    return {
        "urdf_path": URDF_PATH,
        "joint_names": JOINT_NAMES,
        "num_actions": NUM_ACTIONS,
        "default_joint_pos": DEFAULT_JOINT_POS,
        "kp": KP,
        "kd": KD,
        "base_height": BASE_HEIGHT,
        "body_name": "torso",
    }
