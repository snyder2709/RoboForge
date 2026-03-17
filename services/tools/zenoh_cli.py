#!/usr/bin/env python3
"""Утилиты для ручного тестирования Zenoh топиков.

Использование:
  # Слушать топики
  python tools/zenoh_cli.py sub "robot/**"
  python tools/zenoh_cli.py sub "robot/1/state"

  # Отправить команду
  python tools/zenoh_cli.py put "robot/1/cmd" '{"action":"stop","ts":0}'
  python tools/zenoh_cli.py put "robot/1/cmd" '{"action":"reset","ts":0}'
  python tools/zenoh_cli.py put "robot/1/cmd" '{"action":"move_servos","servos":[45,45,45,45,45,45,45,45,45,45,45,45,45,45,45,45,45,45,45,45],"ts":0}'
"""

import argparse
import json
import sys
import time

import zenoh


def cmd_sub(key: str) -> None:
    session = zenoh.open(zenoh.Config())
    print(f"[sub] listening on: {key}  (Ctrl+C to stop)\n")

    def on_sample(sample: zenoh.Sample) -> None:
        try:
            data = json.loads(bytes(sample.payload))
            print(f"[{sample.key_expr}]  {json.dumps(data, ensure_ascii=False)}")
        except Exception:
            print(f"[{sample.key_expr}]  {bytes(sample.payload)}")

    session.declare_subscriber(key, on_sample)
    try:
        while True:
            time.sleep(0.1)
    except KeyboardInterrupt:
        pass
    finally:
        session.close()


def cmd_put(key: str, value: str) -> None:
    session = zenoh.open(zenoh.Config())
    session.put(key, value)
    print(f"[put] {key}  →  {value}")
    session.close()


def main() -> None:
    parser = argparse.ArgumentParser(description="Zenoh CLI для тестирования")
    sub = parser.add_subparsers(dest="cmd", required=True)

    p_sub = sub.add_parser("sub", help="Подписаться на топик")
    p_sub.add_argument("key", help='Zenoh key expression, напр. "robot/**"')

    p_put = sub.add_parser("put", help="Опубликовать сообщение")
    p_put.add_argument("key", help='Zenoh key, напр. "robot/1/cmd"')
    p_put.add_argument("value", help="JSON строка")

    args = parser.parse_args()

    if args.cmd == "sub":
        cmd_sub(args.key)
    elif args.cmd == "put":
        cmd_put(args.key, args.value)


if __name__ == "__main__":
    main()
