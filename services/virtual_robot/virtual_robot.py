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
            "imu":    {k: round(v, 4) for k, v in self.imu.items()},
        }

    # ── Жизненный цикл ────────────────────────────────────────────────────────

    async def run(self) -> None:
        """Запустить робота: открыть Zenoh-сессию, войти в publish-loop."""
        self._session = zenoh.open(
            zenoh.Config.from_json5(
                f'{{"connect":{{"endpoints":["{ZENOH_ROUTER}"]}}}}'
            )
        )

        cmd_key   = f"robot/{self.robot_id}/cmd"
        state_key = f"robot/{self.robot_id}/state"

        self._sub = self._session.declare_subscriber(cmd_key, self._on_cmd)
        self._pub = self._session.declare_publisher(state_key)

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
