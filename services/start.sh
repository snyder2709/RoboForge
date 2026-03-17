#!/usr/bin/env bash
# start.sh — Phase 0 launcher: zenohd + virtual robots
# Usage: ./services/start.sh
#        ROBOT_COUNT=2 ./services/start.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROBOT_COUNT="${ROBOT_COUNT:-4}"
VENV="$SCRIPT_DIR/.venv"
# Windows venv uses Scripts/, Unix uses bin/
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
    VENV_BIN="$VENV/Scripts"
else
    VENV_BIN="$VENV/bin"
fi
PIDS=()
ZENOHD_PID=""

# ── helpers ──────────────────────────────────────────────────────────────────
log()  { echo "[start.sh] $*"; }
die()  { echo "[start.sh] ERROR: $*" >&2; exit 1; }
hr()   { echo "[start.sh] ─────────────────────────────────────"; }

cleanup() {
    hr
    log "stopping all services..."
    for pid in "${PIDS[@]:-}"; do
        kill "$pid" 2>/dev/null && log "  killed pid=$pid" || true
    done
    if [[ -n "$ZENOHD_PID" ]]; then
        kill "$ZENOHD_PID" 2>/dev/null && log "  killed zenohd pid=$ZENOHD_PID" || true
    fi
    log "done."
}
trap cleanup EXIT INT TERM

# ── zenohd ───────────────────────────────────────────────────────────────────
ZENOHD=""
if command -v zenohd &>/dev/null; then
    ZENOHD="zenohd"
elif [[ -x "$SCRIPT_DIR/.bin/zenohd" ]]; then
    ZENOHD="$SCRIPT_DIR/.bin/zenohd"
elif [[ -x "$SCRIPT_DIR/.bin/zenohd.exe" ]]; then
    ZENOHD="$SCRIPT_DIR/.bin/zenohd.exe"
else
    die "zenohd not found. Add to PATH or place at services/.bin/zenohd.exe.
Download: https://github.com/eclipse-zenoh/zenoh/releases"
fi

hr
log "starting zenohd..."
"$ZENOHD" &
ZENOHD_PID=$!
log "  zenohd pid=$ZENOHD_PID  router=:7447"
sleep 1   # wait for router to bind :7447

# ── python venv ──────────────────────────────────────────────────────────────
if [[ ! -d "$VENV" ]]; then
    log "creating venv at $VENV ..."
    python3 -m venv "$VENV"
fi

log "installing requirements..."
"$VENV_BIN/pip" install -q -r "$SCRIPT_DIR/virtual_robot/requirements.txt"
"$VENV_BIN/pip" install -q -r "$SCRIPT_DIR/tools/requirements.txt"

# ── virtual robots ────────────────────────────────────────────────────────────
hr
log "starting $ROBOT_COUNT virtual robot(s)..."
for i in $(seq 1 "$ROBOT_COUNT"); do
    "$VENV_BIN/python" "$SCRIPT_DIR/virtual_robot/virtual_robot.py" "$i" &
    pid=$!
    PIDS+=("$pid")
    log "  robot id=$i  pid=$pid  state=robot/$i/state  cmd=robot/$i/cmd"
done

# ── dev UI ────────────────────────────────────────────────────────────────────
"$VENV_BIN/python" "$SCRIPT_DIR/tools/ui_server.py" &
pid=$!
PIDS+=("$pid")
log "  ui ws  pid=$pid  ws://localhost:8766"
log "  ui app : cd services/tools/ui && npm run dev"

# ── coordinator / agents (добавить здесь когда будут готовы) ──────────────────
# log "starting swarm_coordinator..."
# "$VENV/bin/python" "$SCRIPT_DIR/coordinator/swarm_coordinator.py" &
# PIDS+=($!)

hr
log "all Phase 0 services running."
log "  robots : $ROBOT_COUNT"
log "  ui     : http://localhost:8765"
log "  monitor: z_sub -k 'robot/**'"
log "  stop   : Ctrl+C"
hr

wait
