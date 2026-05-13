#!/usr/bin/env python3
"""Phase 0 Virtual Robot — Zenoh mock одного гуманоидного робота.

Топики:
  Подписка : robot/{id}/cmd   — команды от Agent (CmdPayload)
  Публикация: robot/{id}/state — состояние 2 Гц (StatePayload)

Запуск:
  python virtual_robot.py <robot_id>
"""

import argparse
import asyncio
import json
import logging
import random
import signal
import time
from typing import Any

import zenoh

# ── Константы ─────────────────────────────────────────────────────────────────

SERVO_COUNT: int = 20
STATE_HZ: float = 2.0
STATE_INTERVAL: float = 1.0 / STATE_HZ
ZENOH_ROUTER: str = "tcp/127.0.0.1:7447"

# Маппинг: имя сустава → индекс в servos[] + диапазон (°)
JOINT_MAP: list[dict] = [
    {"index": 0,  "name": "head_pan",            "min_deg": -60,  "max_deg": 60},
    {"index": 1,  "name": "left_shoulder_pitch",  "min_deg": -90,  "max_deg": 90},
    {"index": 2,  "name": "left_elbow_pitch",      "min_deg": -120, "max_deg": 0},
    {"index": 3,  "name": "left_wrist_pitch",      "min_deg": -45,  "max_deg": 45},
    {"index": 4,  "name": "right_shoulder_pitch",  "min_deg": -90,  "max_deg": 90},
    {"index": 5,  "name": "right_elbow_pitch",     "min_deg": -120, "max_deg": 0},
    {"index": 6,  "name": "right_wrist_pitch",     "min_deg": -45,  "max_deg": 45},
    {"index": 7,  "name": "left_hip_yaw",          "min_deg": -30,  "max_deg": 30},
    {"index": 8,  "name": "left_hip_pitch",        "min_deg": -90,  "max_deg": 45},
    {"index": 9,  "name": "left_knee_pitch",       "min_deg": 0,    "max_deg": 120},
    {"index": 10, "name": "left_ankle_pitch",      "min_deg": -45,  "max_deg": 45},
    {"index": 11, "name": "right_hip_yaw",         "min_deg": -30,  "max_deg": 30},
    {"index": 12, "name": "right_hip_pitch",       "min_deg": -90,  "max_deg": 45},
    {"index": 13, "name": "right_knee_pitch",      "min_deg": 0,    "max_deg": 120},
    {"index": 14, "name": "right_ankle_pitch",     "min_deg": -45,  "max_deg": 45},
]

def _servos_to_joints(servos: list[float]) -> dict[str, float]:
    """Конвертирует servos[] (градусы 0-180) в именованные суставы (радианы)."""
    import math
    result = {}
    for jd in JOINT_MAP:
        deg = servos[jd["index"]]
        # 90° = нейтраль → 0 rad, диапазон симметричен
        result[jd["name"]] = round(math.radians(deg - 90.0), 4)
    return result

MODEL_NAME = "humanoid-20dof-v1"
URDF_MODEL = "roboforge_humanoid_v1"

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s  %(name)s  %(levelname)s  %(message)s",
)


# ── Основной класс ────────────────────────────────────────────────────────────

class VirtualRobot:
    """Виртуальный гуманоидный робот (Phase 0 mock).

    Полностью воспроизводит интерфейс Zenoh реального Pico 2W:
    - принимает CmdPayload на robot/{id}/cmd
    - публикует StatePayload на robot/{id}/state @ STATE_HZ
    """

    def __init__(self, robot_id: int) -> None:
        self.robot_id = robot_id
        self.log = logging.getLogger(f"robot.{robot_id}")

        # Состояние
        self.servos: list[float] = [90.0] * SERVO_COUNT
        self.imu: dict[str, float] = {"roll": 0.0, "pitch": 0.0, "yaw": 0.0}

        # Zenoh handles (инициализируются в run())
        self._session: zenoh.Session | None = None
        self._pub: zenoh.Publisher | None = None
        self._sub: zenoh.Subscriber | None = None
        self._running: bool = False

    # ── Zenoh callbacks ───────────────────────────────────────────────────────

    def _on_cmd(self, sample: zenoh.Sample) -> None:
        """Обработчик входящей команды (вызывается из потока zenoh)."""
        try:
            cmd: dict[str, Any] = json.loads(bytes(sample.payload))
            action = cmd.get("action", "unknown")
            self.log.info("cmd  action=%s  ts=%.3f", action, cmd.get("ts", 0))
            self._apply_cmd(cmd)
        except Exception as exc:  # noqa: BLE001
            self.log.warning("bad cmd payload: %s", exc)

    def _apply_cmd(self, cmd: dict[str, Any]) -> None:
        """Применить команду к состоянию (Phase 0 — mock без IK)."""
        action = cmd.get("action", "")

        if action == "stop":
            self.log.info("stop command received")
            return

        if action == "reset":
            self.servos = [90.0] * SERVO_COUNT
            self.imu = {"roll": 0.0, "pitch": 0.0, "yaw": 0.0}
            self.log.info("state reset to defaults")
            return

        if "servos" in cmd:
            for i, angle in enumerate(cmd["servos"]):
                if i < SERVO_COUNT:
                    self.servos[i] = float(max(0.0, min(180.0, angle)))

    # ── Симуляция состояния ───────────────────────────────────────────────────

    def _tick_imu(self) -> None:
        """Имитация шума IMU (мелкий drift)."""
        self.imu["roll"]  += random.gauss(0, 0.002)
        self.imu["pitch"] += random.gauss(0, 0.002)
        self.imu["yaw"]   += random.gauss(0, 0.001)

    def _build_state(self) -> dict[str, Any]:
        return {
            "id":     self.robot_id,
            "ts":     time.time(),
            "servos": [round(a, 2) for a in self.servos],
            "joints": _servos_to_joints(self.servos),
            "imu":    {k: round(v, 4) for k, v in self.imu.items()},
        }

    def _build_descriptor(self) -> dict[str, Any]:
        return {
            "id":         self.robot_id,
            "model":      MODEL_NAME,
            "urdf_model": URDF_MODEL,
            "joints":     JOINT_MAP,
        }

    # ── Жизненный цикл ────────────────────────────────────────────────────────

    async def run(self) -> None:
        """Запустить робота: открыть Zenoh-сессию, войти в publish-loop."""
        self._session = zenoh.open(
            zenoh.Config.from_json5(
                f'{{"connect":{{"endpoints":["{ZENOH_ROUTER}"]}}}}'
            )
        )

        cmd_key        = f"robot/{self.robot_id}/cmd"
        state_key      = f"robot/{self.robot_id}/state"
        descriptor_key = f"robot/{self.robot_id}/descriptor"

        self._sub = self._session.declare_subscriber(cmd_key, self._on_cmd)
        self._pub = self._session.declare_publisher(state_key)
        desc_pub  = self._session.declare_publisher(descriptor_key)

        # Публикуем descriptor один раз при старте
        desc_pub.put(json.dumps(self._build_descriptor()))
        self.log.info("descriptor published  key=%s", descriptor_key)

        self._running = True
        self.log.info(
            "started  id=%d  sub=%s  pub=%s  hz=%.1f",
            self.robot_id, cmd_key, state_key, STATE_HZ,
        )

        while self._running:
            self._tick_imu()
            state = self._build_state()
            self._pub.put(json.dumps(state))
            await asyncio.sleep(STATE_INTERVAL)

        self._session.close()
        self.log.info("stopped  id=%d", self.robot_id)

    def stop(self) -> None:
        """Запросить остановку publish-loop."""
        self._running = False


# ── Entrypoint ────────────────────────────────────────────────────────────────

def main() -> None:
    parser = argparse.ArgumentParser(
        description="Phase 0 Virtual Robot — Zenoh mock",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument("robot_id", type=int, help="Robot ID (целое число, 1-based)")
    args = parser.parse_args()

    robot = VirtualRobot(args.robot_id)

    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)

    def _shutdown(*_: Any) -> None:
        robot.stop()

    signal.signal(signal.SIGINT,  _shutdown)
    signal.signal(signal.SIGTERM, _shutdown)

    loop.run_until_complete(robot.run())


if __name__ == "__main__":
    main()
