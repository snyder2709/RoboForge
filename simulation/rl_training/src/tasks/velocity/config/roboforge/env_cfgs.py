"""
Конфиг среды для обучения ходьбе RoboForge Humanoid v2.
Адаптирован из unitree_rl_mjlab/src/tasks/velocity/config/g1/env_cfgs.py
под параметры нашего робота: 21 DOF, ~28 см, масса ~460г.
"""
from src.assets.robots.roboforge import get_roboforge_robot_cfg, BASE_HEIGHT


def make_roboforge_flat_env_cfg(num_envs: int = 512):
    robot_cfg = get_roboforge_robot_cfg()

    return {
        # ── Робот ────────────────────────────────────────────────
        "robot": robot_cfg,
        "viewer_body": "torso",
        "viewer_z_offset": BASE_HEIGHT + 0.05,  # 0.195м

        # ── Симуляция ────────────────────────────────────────────
        "sim": {
            "dt": 0.005,          # шаг физики 5мс
            "decimation": 4,      # управление каждые 20мс = 50 Гц
            "episode_length_s": 20.0,
        },

        # ── Среда ────────────────────────────────────────────────
        "scene": {
            "num_envs": num_envs,
            "terrain": "flat",    # только плоская поверхность
        },

        # ── Команды скорости ─────────────────────────────────────
        # Маленький робот — скорость меньше чем G1
        "commands": {
            "lin_vel_x":  (-0.3, 0.5),   # м/с   G1: (-1.0, 2.0)
            "lin_vel_y":  (-0.2, 0.2),   # м/с   G1: (-1.0, 1.0)
            "ang_vel_z":  (-0.5, 0.5),   # рад/с G1: (-1.0, 1.0)
            "resample_time_s": (3.0, 8.0),
        },

        # ── Награды ──────────────────────────────────────────────
        "rewards": {
            # Основные
            "track_lin_vel":     {"weight": 1.0},
            "track_ang_vel":     {"weight": 1.0},
            "alive":             {"weight": 0.5},

            # Штрафы за нестабильность
            "body_orientation":  {"weight": -1.0},
            "lin_vel_z":         {"weight": -0.5},
            "ang_vel_xy":        {"weight": -0.05},
            "action_rate":       {"weight": -0.01},
            "joint_torques":     {"weight": -0.0001},

            # Высота торса — держать около BASE_HEIGHT
            "base_height": {
                "weight": -1.0,
                "target": BASE_HEIGHT,
                "std": 0.02,
            },

            # Стандартные отклонения суставов при ходьбе
            "joint_deviation": {
                "weight": -0.1,
                "std": {
                    "left_ankle_roll":  0.15,
                    "right_ankle_roll": 0.15,
                    "left_hip_pitch":   0.40,
                    "right_hip_pitch":  0.40,
                    "left_knee_pitch":  0.40,
                    "right_knee_pitch": 0.40,
                    # Остальные суставы — меньше отклонений
                    "_default": 0.20,
                },
            },
        },

        # ── Терминальные условия ──────────────────────────────────
        "terminations": {
            "base_contact": True,     # упал торсом
            "max_episode_length": True,
        },

        # ── Рандомизация (domain randomization) ───────────────────
        "randomization": {
            "foot_friction":    (0.4, 1.4),   # G1: (0.3, 1.6)
            "com_displacement": 0.03,          # ±3см смещение ЦМ
            "encoder_bias":     0.010,         # ±0.01 рад шум энкодера
        },
    }


class RoboforgeVelocityFlatEnvCfg:
    def __init__(self, num_envs: int = 512):
        self.cfg = make_roboforge_flat_env_cfg(num_envs)
