#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════
#  RoboForge RL Training — от URDF до ходьбы
#  Использование:
#    ./run_training.sh setup     — установить зависимости
#    ./run_training.sh validate  — проверить URDF в MuJoCo viewer
#    ./run_training.sh train     — запустить обучение
#    ./run_training.sh play      — воспроизвести результат
#    ./run_training.sh all       — setup → validate → train → play
# ═══════════════════════════════════════════════════════════════

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
URDF_V2="${REPO_ROOT}/Research/robot-research-vault/research/robot-hardware-research/artefacts/urdf/humanoid_v2.urdf"
MJCF_OUT="${SCRIPT_DIR}/src/assets/robots/roboforge/xmls/roboforge.xml"
LOGS_DIR="${SCRIPT_DIR}/logs"
VENV_DIR="${SCRIPT_DIR}/.venv"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
info()    { echo -e "${GREEN}[INFO]${NC} $1"; }
warn()    { echo -e "${YELLOW}[WARN]${NC} $1"; }
error()   { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# ───────────────────────────────────────────────────────────────
cmd_setup() {
    info "Шаг 1/4 — Установка зависимостей"

    # ── Venv на Python 3.10 ─────────────────────────────────────
    if [ ! -f "${VENV_DIR}/Scripts/activate" ] && [ ! -f "${VENV_DIR}/bin/activate" ]; then
        info "Создаю venv на Python 3.10 в ${VENV_DIR}..."
        PY310=$(command -v py 2>/dev/null)
        if [ -n "${PY310}" ]; then
            py -3.10 -m venv "${VENV_DIR}" || error "Не удалось создать venv. Убедись что Python 3.10 установлен: py install 3.10"
        else
            python3.10 -m venv "${VENV_DIR}" || error "Не удалось создать venv. Убедись что python3.10 доступен в PATH."
        fi
        info "✓ venv создан"
    else
        info "venv уже существует, пропускаю создание"
    fi

    # ── Активация venv ──────────────────────────────────────────
    if [ -f "${VENV_DIR}/Scripts/activate" ]; then
        # Windows (Git Bash / MSYS2)
        # shellcheck disable=SC1091
        source "${VENV_DIR}/Scripts/activate"
    else
        # Linux / macOS
        # shellcheck disable=SC1091
        source "${VENV_DIR}/bin/activate"
    fi
    info "✓ venv активирован: $(python --version)"

    # ── Зависимости ─────────────────────────────────────────────
    # MuJoCo + RSL-RL (до torch, чтобы не перезаписать CUDA-сборку)
    info "Устанавливаю зависимости (requirements.txt)..."
    python -m pip install -r "${SCRIPT_DIR}/requirements.txt"

    # unitree_rl_mjlab — до torch по той же причине
    if [ ! -d "${SCRIPT_DIR}/unitree_rl_mjlab" ]; then
        info "Клонирую unitree_rl_mjlab..."
        git clone https://github.com/unitreerobotics/unitree_rl_mjlab \
            "${SCRIPT_DIR}/unitree_rl_mjlab"
        cd "${SCRIPT_DIR}/unitree_rl_mjlab" && python -m pip install -e . && cd "${SCRIPT_DIR}"
    else
        warn "unitree_rl_mjlab уже клонирован, пропускаю"
    fi

    # PyTorch (CUDA 12 для RTX 3050) — последним, чтобы rsl-rl / unitree
    # не перезаписали CPU-версией
    info "Устанавливаю PyTorch + CUDA 12 (requirements-torch.txt)..."
    python -m pip install -r "${SCRIPT_DIR}/requirements-torch.txt" --force-reinstall --no-deps

    # Проверка
    info "Проверяю установку..."
    python -c "import mujoco; print('MuJoCo', mujoco.__version__, '— OK')"
    python -c "import torch; print('PyTorch', torch.__version__, '— OK')"
    python -c "import torch; print('CUDA available:', torch.cuda.is_available())"

    info "✓ Setup завершён"
    info "  Для ручной активации venv: source ${VENV_DIR}/Scripts/activate"
}

# ───────────────────────────────────────────────────────────────
cmd_validate() {
    info "Шаг 2/4 — Валидация URDF в MuJoCo viewer"
    cmd_activate_venv

    [ -f "${URDF_V2}" ] || error "URDF не найден: ${URDF_V2}"

    info "Конвертирую URDF → MJCF..."
    mkdir -p "$(dirname "${MJCF_OUT}")"

    # Готовим пути Windows
    WIN_URDF=$(cygpath -w "${URDF_V2}")
    WIN_MJCF=$(cygpath -w "${MJCF_OUT}")

    # Конвертация
    python3 -c "import mujoco; model = mujoco.MjModel.from_xml_path(r'${WIN_URDF}'); mujoco.mj_saveLastXML(r'${WIN_MJCF}', model)" \
        && info "MJCF сохранён: ${MJCF_OUT}" \
        || error "Ошибка конвертации. Проверь URDF."

    info "Проверяю физику (без viewer)..."
    # Передаем Windows-путь напрямую в Python через аргумент
    python3 - "${WIN_MJCF}" <<'PYEOF'
import mujoco, sys, os

# Получаем путь из аргумента командной строки
mjcf_path = sys.argv[1]
model = mujoco.MjModel.from_xml_path(mjcf_path)
data  = mujoco.MjData(model)

print(f"  Звеньев (links):   {model.nbody}")
print(f"  Суставов (joints): {model.njnt}")
print(f"  Актуаторов:        {model.nu}")
print(f"  Масса робота:      {sum(model.body_mass):.3f} кг")

# Прогнать 100 шагов
for _ in range(100):
    mujoco.mj_step(model, data)

import math
if any(math.isnan(x) for x in data.qpos):
    print("FAIL: NaN в qpos — проверь inertia в URDF")
    sys.exit(1)
else:
    print("OK: 100 шагов без NaN")
PYEOF

    # info "Открываю MuJoCo viewer..."
    # python3 -m mujoco.viewer --mjcf="${WIN_MJCF}"
}

# ───────────────────────────────────────────────────────────────
cmd_train() {
    info "Шаг 3/4 — Запуск обучения ходьбе"

    [ -f "${MJCF_OUT}" ] || error "MJCF не найден. Сначала запусти: $0 validate"

    mkdir -p "${LOGS_DIR}"

    NUM_ENVS=${NUM_ENVS:-512}
    MAX_ITER=${MAX_ITER:-1000}

    cmd_activate_venv
    info "Параметры: num_envs=${NUM_ENVS}, max_iterations=${MAX_ITER}"
    info "TensorBoard: tensorboard --logdir ${LOGS_DIR}"

    # RESUME=1 — дообучить с последнего чекпойнта (берётся автоматически)
    if [ -n "${RESUME}" ]; then
        warn "Дообучение с последнего чекпойнта..."
        RESUME_ARG="--agent.resume True"
    else
        RESUME_ARG=""
    fi

    cd "${SCRIPT_DIR}/unitree_rl_mjlab"
    WANDB_MODE=disabled python scripts/train.py Roboforge-Flat \
        --env.scene.num-envs=${NUM_ENVS} \
        --agent.max-iterations=${MAX_ITER} \
        --agent.experiment-name="roboforge_flat" \
        ${RESUME_ARG} \
        ${VIDEO:+--video --video-interval=500 --video-length=200}

    info "✓ Обучение завершено. Модели в: ${SCRIPT_DIR}/unitree_rl_mjlab/logs/rsl_rl/roboforge_flat/"
}

# ───────────────────────────────────────────────────────────────
cmd_activate_venv() {
    if [ -f "${VENV_DIR}/Scripts/activate" ]; then
        # shellcheck disable=SC1091
        source "${VENV_DIR}/Scripts/activate"
    else
        # shellcheck disable=SC1091
        source "${VENV_DIR}/bin/activate"
    fi
}

cmd_play() {
    info "Шаг 4/4 — Воспроизведение результата"
    cmd_activate_venv

    TRAIN_LOGS="${SCRIPT_DIR}/unitree_rl_mjlab/logs/rsl_rl/roboforge_flat"

    # Найти последний чекпойнт
    LATEST=$(ls -td "${TRAIN_LOGS}/"*/ 2>/dev/null | head -1)
    [ -n "${LATEST}" ] || error "Чекпойнт не найден в ${TRAIN_LOGS}. Сначала запусти train."

    LATEST_PT=$(ls -t "${TRAIN_LOGS}/"*/model_*.pt 2>/dev/null | head -1)
    [ -n "${LATEST_PT}" ] || error "Чекпойнт .pt не найден в ${TRAIN_LOGS}."

    info "Загружаю: ${LATEST_PT}"
    cd "${SCRIPT_DIR}/unitree_rl_mjlab"
    python scripts/play.py Roboforge-Flat \
        --checkpoint-file "${LATEST_PT}"
}

# ───────────────────────────────────────────────────────────────
case "${1:-help}" in
    setup)    cmd_setup ;;
    validate) cmd_validate ;;
    train)    cmd_train ;;
    play)     cmd_play ;;
    all)
        cmd_setup
        cmd_validate
        cmd_train
        cmd_play
        ;;
    *)
        echo "Использование: $0 {setup|validate|train|play|all}"
        echo ""
        echo "  setup    — установить зависимости (однократно)"
        echo "  validate — конвертировать URDF→MJCF и открыть viewer"
        echo "  train    — обучение (NUM_ENVS=512 MAX_ITER=3000 по умолчанию)"
        echo "  play     — воспроизвести последний чекпойнт"
        echo "  all      — полный цикл setup→validate→train→play"
        echo ""
        echo "  Переменные среды:"
        echo "    NUM_ENVS=256 MAX_ITER=1000 ./run_training.sh train"
        ;;
esac
