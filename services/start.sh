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
"$ZENOHD" --cfg 'listen/endpoints:["tcp/0.0.0.0:7447"]' &
ZENOHD_PID=$!
log "  zenohd pid=$ZENOHD_PID  router=0.0.0.0:7447"
sleep 1   # wait for router to bind :7447

# ── python venv ──────────────────────────────────────────────────────────────
# Find a working Python 3.10+ that can create real venvs.
# Skips Windows Store python/python3 aliases (they break venv in non-ASCII paths),
# but keeps the 'py' launcher which correctly routes to real installs.
find_python() {
    local ver_check='import sys; sys.exit(0 if sys.version_info >= (3,10) else 1)'

    # 1. Windows py launcher (works even from WindowsApps; tries newest first)
    if command -v py &>/dev/null; then
        for ver in -3.14 -3.13 -3.12 -3.11 -3.10; do
            if py "$ver" -c "$ver_check" 2>/dev/null; then
                echo "py $ver"; return 0
            fi
        done
    fi

    # 2. python3 / python on PATH — skip Store/AppData aliases
    for cmd in python3 python; do
        local exe
        exe=$(command -v "$cmd" 2>/dev/null) || continue
        case "$exe" in
            */AppData/Local/Microsoft/WindowsApps/*) continue ;;
            */AppData/Local/Python/*) continue ;;
        esac
        if "$cmd" -c "$ver_check" 2>/dev/null; then
            echo "$cmd"; return 0
        fi
    done

    # 3. Common Windows fixed install paths (not on PATH)
    for dir in /c/Python3{14,13,12,11,10} /c/Python3{14,13,12,11,10}-64; do
        if [[ -x "$dir/python.exe" ]]; then
            if "$dir/python.exe" -c "$ver_check" 2>/dev/null; then
                echo "$dir/python.exe"; return 0
            fi
        fi
    done

    return 1
}

PYTHON3=$(find_python) || die "Python 3.10+ not found. Install from python.org or enable via 'py' launcher."
log "  python: $PYTHON3 ($(${PYTHON3} --version 2>&1))"

# Recreate venv if it doesn't exist or its python is broken
if [[ -d "$VENV" ]] && ! "$VENV_BIN/python" -c "import sys; print(sys.version)" &>/dev/null; then
    log "  existing venv is broken — removing..."
    rm -rf "$VENV"
fi
if [[ ! -d "$VENV" ]]; then
    log "creating venv at $VENV ..."
    ${PYTHON3} -m venv "$VENV"
fi

log "installing requirements..."
"$VENV_BIN/python" -m pip install -q -r "$SCRIPT_DIR/virtual_robot/requirements.txt"
"$VENV_BIN/python" -m pip install -q -r "$SCRIPT_DIR/tools/requirements.txt"

# ── virtual robots ────────────────────────────────────────────────────────────
hr
log "starting $ROBOT_COUNT virtual robot(s)..."
for i in $(seq 1 "$ROBOT_COUNT"); do
    "$VENV_BIN/python" "$SCRIPT_DIR/virtual_robot/virtual_robot.py" "$i" &
    pid=$!
    PIDS+=("$pid")
    log "  robot id=$i  pid=$pid  state=robot/$i/state  cmd=robot/$i/cmd"
done

# ── dev UI server (WebSocket bridge: Zenoh → browser) ────────────────────────
"$VENV_BIN/python" "$SCRIPT_DIR/tools/ui_server.py" &
pid=$!
PIDS+=("$pid")
log "  ui_server pid=$pid  ws://localhost:8766"

# ── coordinator / agents (добавить здесь когда будут готовы) ──────────────────
# log "starting swarm_coordinator..."
# "$VENV/bin/python" "$SCRIPT_DIR/coordinator/swarm_coordinator.py" &
# PIDS+=($!)

hr
log "all Phase 0 services running."
log "  robots : $ROBOT_COUNT"
log "  3D UI  : cd services/tools/ui && npm run dev  →  http://localhost:5173"
log "  monitor: z_sub -k 'robot/**'"
log "  stop   : Ctrl+C"
hr

wait
