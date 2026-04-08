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

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
info()    { echo -e "${GREEN}[INFO]${NC} $1"; }
warn()    { echo -e "${YELLOW}[WARN]${NC} $1"; }
error()   { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# ───────────────────────────────────────────────────────────────
cmd_setup() {
    info "Шаг 1/4 — Установка зависимостей"

    # Проверить Python
    python3 --version || error "Python3 не найден"

    # MuJoCo
    info "Устанавливаю MuJoCo..."
    pip install mujoco

    # PyTorch (CUDA 12 для RTX 3050)
    info "Устанавливаю PyTorch + CUDA 12..."
    pip install torch torchvision --index-url https://download.pytorch.org/whl/cu121

    # RSL-RL (PPO)
    info "Устанавливаю rsl-rl..."
    pip install rsl-rl

    # unitree_rl_mjlab
    if [ ! -d "${SCRIPT_DIR}/unitree_rl_mjlab" ]; then
        info "Клонирую unitree_rl_mjlab..."
        git clone https://github.com/unitreerobotics/unitree_rl_mjlab \
            "${SCRIPT_DIR}/unitree_rl_mjlab"
        cd "${SCRIPT_DIR}/unitree_rl_mjlab" && pip install -e . && cd "${SCRIPT_DIR}"
    else
        warn "unitree_rl_mjlab уже клонирован, пропускаю"
    fi

    # Проверка
    info "Проверяю установку..."
    python3 -c "import mujoco; print('MuJoCo', mujoco.__version__, '— OK')"
    python3 -c "import torch; print('PyTorch', torch.__version__, '— OK')"
    python3 -c "import torch; print('CUDA available:', torch.cuda.is_available())"

    info "✓ Setup завершён"
}

# ───────────────────────────────────────────────────────────────
cmd_validate() {
    info "Шаг 2/4 — Валидация URDF в MuJoCo viewer"

    [ -f "${URDF_V2}" ] || error "URDF не найден: ${URDF_V2}"

    # Конвертировать URDF → MJCF
    info "Конвертирую URDF → MJCF..."
    mkdir -p "$(dirname "${MJCF_OUT}")"
    python3 -m mujoco.compile "${URDF_V2}" "${MJCF_OUT}" \
        && info "MJCF сохранён: ${MJCF_OUT}" \
        || error "Ошибка конвертации. Проверь URDF."

    # Статическая проверка физики
    info "Проверяю физику (без viewer)..."
    python3 - <<'PYEOF'
import mujoco, sys

model = mujoco.MjModel.from_xml_path("${MJCF_OUT}")
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

    info "Открываю MuJoCo viewer (закрой окно чтобы продолжить)..."
    python3 -m mujoco.viewer --mjcf="${MJCF_OUT}"

    info "✓ Валидация завершена"
}

# ───────────────────────────────────────────────────────────────
cmd_train() {
    info "Шаг 3/4 — Запуск обучения ходьбе"

    [ -f "${MJCF_OUT}" ] || error "MJCF не найден. Сначала запусти: $0 validate"

    mkdir -p "${LOGS_DIR}"

    NUM_ENVS=${NUM_ENVS:-512}
    MAX_ITER=${MAX_ITER:-3000}

    info "Параметры: num_envs=${NUM_ENVS}, max_iterations=${MAX_ITER}"
    info "TensorBoard: tensorboard --logdir ${LOGS_DIR}"

    cd "${SCRIPT_DIR}"
    python3 scripts/train.py Roboforge-Flat \
        --env.scene.num-envs=${NUM_ENVS} \
        --train.runner.max_iterations=${MAX_ITER} \
        --train.runner.experiment_name="roboforge_flat" \
        --log_dir="${LOGS_DIR}"

    info "✓ Обучение завершено. Модели в: ${LOGS_DIR}/rsl_rl/roboforge_flat/"
}

# ───────────────────────────────────────────────────────────────
cmd_play() {
    info "Шаг 4/4 — Воспроизведение результата"

    # Найти последний чекпойнт
    LATEST=$(ls -td "${LOGS_DIR}/rsl_rl/roboforge_flat/"*/ 2>/dev/null | head -1)
    [ -n "${LATEST}" ] || error "Чекпойнт не найден в ${LOGS_DIR}. Сначала запусти train."

    info "Загружаю: ${LATEST}"
    cd "${SCRIPT_DIR}"
    python3 scripts/play.py Roboforge-Flat \
        --load_run "${LATEST}"
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
