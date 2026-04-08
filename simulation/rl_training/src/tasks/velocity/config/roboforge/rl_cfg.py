"""
PPO конфиг для RoboForge.
Архитектура сети и гиперпараметры — без изменений относительно G1.
PPO универсален: меняем только размер входа/выхода через env_cfg.
"""


class RoboforgeVelocityFlatPPORunnerCfg:
    # ── Сеть ────────────────────────────────────────────────────
    actor_hidden_dims  = (512, 256, 128)
    critic_hidden_dims = (512, 256, 128)
    activation         = "elu"
    init_noise_std     = 1.0
    normalize_obs      = True

    # ── PPO ─────────────────────────────────────────────────────
    clip_param         = 0.2
    entropy_coef       = 0.01
    learning_rate      = 1.0e-3
    lr_schedule        = "adaptive"
    value_loss_coef    = 1.0
    use_clipped_value_loss = True
    gamma              = 0.99
    lam                = 0.95

    # ── Runner ──────────────────────────────────────────────────
    num_steps_per_env  = 24        # шагов на среду за итерацию
    num_learning_epochs = 5
    num_mini_batches   = 4
    max_iterations     = 3000      # ~30-90 мин на RTX 3050

    # ── Логи и чекпойнты ────────────────────────────────────────
    save_interval      = 100       # сохранять модель каждые 100 итераций
    experiment_name    = "roboforge_flat"
