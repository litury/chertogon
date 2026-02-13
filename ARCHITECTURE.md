# Архитектура Chertogon

## Обзор

Bevy ECS игра с модульной архитектурой. Каждый модуль — изолированный Plugin. Состояние игры управляется через `GameState` как центральный роутер.

## Граф зависимостей модулей

```
              shared (GameState, GameLayer, constants)
              toolkit (asset_paths)
                  │
    ┌─────────────┼─────────────────────┐
    │             │                     │
  world        input                 camera
  (арена)      (WASD/touch)         (follow, zoom)
    │             │                     │
    │         ┌───┘                     │
    │         ▼                         │
    │       player ◄────────────────────┘
    │       (движение, анимации)
    │         │
    │         │    enemies
    │         │    (спавн, AI)
    │         │       │
    │         └───┬───┘
    │             ▼
    │          combat
    │          (автоатака, урон, VFX)
    │             │
    │             ▼
    │           menu
    │           (UI: title, HUD, game over)
    └─────────────┘
```

## Поток данных

```
Input (WASD/Touch)
  → InputState (Resource)
    → Player Movement (LinearVelocity)
      → Camera Follow (экспоненциальное сглаживание)
      → Enemy AI (дистанция до Player → Idle/Chase/Attack)
        → Combat (автоатака ближайшего, PendingAttack → урон)
          → VFX (slash, частицы, hitstop, camera shake, knockback)
          → KillCount, GameTimer
            → Menu HUD (обновление UI)
            → Game Over (PlayerHealth ≤ 0 → fade → GameOver state)
```

## Состояния игры (GameState)

```
TitleScreen ──[any key]──► Playing ──[HP ≤ 0]──► GameOver
     ▲                        ▲                      │
     │                        └──────[Заново]────────┤
     └──────────────────────[В Меню]─────────────────┘
```

Переходы через `FadeState` — плавное затемнение между экранами. Fade использует `Real` time (работает при паузе `Virtual` time).

## Физика и коллизии

Avian3D, нулевая гравитация (top-down). Коллизии через слои `GameLayer`:

| Слой | Коллайдирует с |
|------|---------------|
| `Static` (стены, пол) | Player, Enemy, Projectile |
| `Player` | Static, Enemy |
| `Enemy` | Static, Player, Enemy |
| `Projectile` | Static (зарезервирован) |

Вспомогательные методы: `GameLayer::player_layers()`, `::enemy_layers()`, `::static_layers()`.

## Иерархия сущностей

```
Player (RigidBody::Dynamic, Collider, InputState)
  └── PlayerModel (SceneRoot — bogatyr_merged.glb)
        └── "RightHand" bone
              └── WeaponModel (SceneRoot — runic_sword.glb)

Enemy (RigidBody::Dynamic, Collider, Health, ChasePlayer)
  └── EnemyModel (SceneRoot — upyr_merged.glb)
  └── GroundCircle (HP ring mesh)
```

Анимации привязаны к SceneRoot через `AnimationGraph`. Поиск `AnimationPlayer` в дочерних сущностях — паттерн poll-until-ready (каждый кадр проверяет, загрузился ли GLB).

## Боевой конвейер

```
AttackCooldown.finished()
  → player_auto_attack_system: находит ближайшего врага, поворачивает модель, играет анимацию
  → spawn PendingAttack (target, damage, direction, timer=0.42s)
    → apply_pending_attack_system: при срабатывании таймера:
        ├── Health -= damage
        ├── spawn_slash() — огненная дуга
        ├── spawn_hit_particles() — искры
        ├── spawn_damage_number() — всплывающий урон
        ├── spawn_blood_decal() — пятно крови на полу
        ├── CameraShake.trigger()
        ├── Hitstop.trigger() — замедление на 50мс
        ├── Staggered — отбрасывание
        └── HitFlash — масштабный импульс модели
```

## Волновой спавнер

```
WaveState::Cooldown (5с)
  → WaveState::Spawning (2 + wave_number врагов)
    → WaveState::Fighting (ждём пока все убиты)
      → wave_number += 1, обратно к Cooldown
```

## Ключевые ресурсы (Resources)

| Ресурс | Модуль | Описание |
|--------|--------|----------|
| `InputState` | input | Направление, бег, кнопка атаки |
| `CameraShake` | combat | Интенсивность тряски камеры |
| `Hitstop` | combat | Замедление виртуального времени |
| `KillCount` | combat | Счётчик убийств |
| `GameTimer` | combat | Время раунда (MM:SS) |
| `FadeState` | menu | Управление fade-переходами |
| `WaveState` | enemies | Текущая волна и её фаза |
