#!/usr/bin/env python3
"""RoboForge dev UI — WebSocket bridge to Zenoh.

Слушает Zenoh robot/** и пробрасывает состояния в браузер через WebSocket.
Принимает команды от браузера и публикует в Zenoh robot/{id}/cmd.

Usage:
  python services/tools/ui_server.py [--port 8766]

Frontend: cd services/tools/ui && npm run dev  → http://localhost:5173
"""

import argparse
import asyncio
import json
import logging
import signal
import time
from typing import Any

import websockets
import websockets.asyncio.server
import zenoh

WS_PORT: int = 8766
ZENOH_SUB_KEY: str = "robot/**"

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s  %(name)s  %(levelname)s  %(message)s",
)


class UIServer:
    def __init__(self, port: int) -> None:
        self._port = port
        self._clients: set[websockets.asyncio.server.ServerConnection] = set()
        self._clients_lock = asyncio.Lock()
        self._zenoh_session: zenoh.Session | None = None
        self._loop: asyncio.AbstractEventLoop | None = None
        self.log = logging.getLogger("ui_server")

    async def _ws_handler(
        self, ws: websockets.asyncio.server.ServerConnection
    ) -> None:
        async with self._clients_lock:
            self._clients.add(ws)
        self.log.info("client connected  total=%d", len(self._clients))
        await ws.send(json.dumps({"type": "connected", "ts": time.time()}))
        try:
            async for raw in ws:
                try:
                    data: dict[str, Any] = json.loads(raw)
                    if data.get("type") == "cmd":
                        await self._handle_cmd(data)
                    elif data.get("type") == "scenario":
                        await self._handle_scenario(data)
                except json.JSONDecodeError:
                    self.log.warning("invalid json from client")
        except websockets.ConnectionClosed:
            pass
        finally:
            async with self._clients_lock:
                self._clients.discard(ws)
            self.log.info("client disconnected  total=%d", len(self._clients))

    async def _handle_cmd(self, data: dict[str, Any]) -> None:
        robot_id = data.get("robot_id")
        action = data.get("action", "")
        cmd: dict[str, Any] = {"action": action, "ts": time.time()}
        if action == "move_servos" and "servos" in data:
            cmd["servos"] = data["servos"]
        key = f"robot/{robot_id}/cmd"
        loop = asyncio.get_event_loop()
        await loop.run_in_executor(
            None, self._zenoh_session.put, key, json.dumps(cmd)  # type: ignore[union-attr]
        )
        self.log.info("cmd  robot=%s  action=%s", robot_id, action)

    async def _handle_scenario(self, data: dict[str, Any]) -> None:
        payload = {"robot_id": data["robot_id"], "scenario": data["scenario"], "ts": data.get("ts", time.time())}
        loop = asyncio.get_event_loop()
        await loop.run_in_executor(
            None, self._zenoh_session.put, "swarm/scenario", json.dumps(payload)  # type: ignore[union-attr]
        )
        self.log.info("scenario  robot=%s  key=%s", data["robot_id"], data["scenario"])


    async def _broadcast(self, msg: str) -> None:
        async with self._clients_lock:
            dead: set = set()
            for ws in self._clients:
                try:
                    await ws.send(msg)
                except websockets.ConnectionClosed:
                    dead.add(ws)
            self._clients -= dead

    def _on_state(self, sample: zenoh.Sample) -> None:
        try:
            payload: dict[str, Any] = json.loads(bytes(sample.payload))
            msg = json.dumps({"type": "state", "robot_id": payload["id"], **payload})
            asyncio.run_coroutine_threadsafe(self._broadcast(msg), self._loop)  # type: ignore[arg-type]
        except Exception as exc:
            self.log.warning("bad state payload: %s", exc)

    async def start(self) -> None:
        self._loop = asyncio.get_event_loop()
        self._zenoh_session = zenoh.open(zenoh.Config())
        self._zenoh_session.declare_subscriber(ZENOH_SUB_KEY, self._on_state)
        self.log.info("zenoh subscribed to %s", ZENOH_SUB_KEY)

        async with websockets.asyncio.server.serve(
            self._ws_handler, "localhost", self._port
        ):
            self.log.info("ws listening on ws://localhost:%d", self._port)
            self.log.info("open frontend: cd services/tools/ui && npm run dev")
            await asyncio.Future()


def main() -> None:
    parser = argparse.ArgumentParser(description="RoboForge Dev UI WS bridge")
    parser.add_argument("--port", type=int, default=WS_PORT)
    args = parser.parse_args()

    server = UIServer(port=args.port)
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)

    def _shutdown(*_: Any) -> None:
        loop.stop()

    signal.signal(signal.SIGINT, _shutdown)
    signal.signal(signal.SIGTERM, _shutdown)

    loop.run_until_complete(server.start())


if __name__ == "__main__":
    main()
