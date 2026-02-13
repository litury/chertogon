use bevy::prelude::*;

/// Маркер компонент для врага
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Enemy;

/// Здоровье врага
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
}

/// Тип врага
#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub enum EnemyType {
    Upyr,      // Упырь — славянский зомби (медленный, HP 20)
    Leshiy,    // Леший — лесной дух (быстрый фланкер, HP 15)
    Volkolak,  // Волколак — четвероногий хищник (быстрый, HP 12)
}

/// Маркер для визуальной модели врага (child entity)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EnemyModel;

/// Индексы анимаций врага в AnimationGraph
#[derive(Component, Clone, Copy)]
pub struct EnemyAnimations {
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub run: AnimationNodeIndex,
    pub attack: AnimationNodeIndex,
    pub death: AnimationNodeIndex,
    pub hit: AnimationNodeIndex,
    pub scream: AnimationNodeIndex,
}

/// Маркер завершения настройки анимаций врага
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EnemyAnimationSetupComplete;

/// Состояние анимации врага
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EnemyAnimState {
    pub current: EnemyAnim,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Reflect)]
pub enum EnemyAnim {
    Idle,
    Walking,
    Running,
    Attacking,
    HitReaction,
    Screaming,
    Dying,
}

/// Маркер умирающего врага (проигрывает death анимацию перед despawn)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EnemyDying {
    pub timer: Timer,
}

/// Маркер трупа врага (статичный визуальный объект после death-анимации)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EnemyCorpse;

/// Таймер повтора анимации атаки (пока враг в Attacking состоянии)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EnemyAttackAnimTimer {
    pub timer: Timer,
}

/// Таймер крика при спавне — враг кричит и не двигается
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpawnScream {
    pub timer: Timer,
}

/// AI врага: преследование + агро-зона + атака
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChasePlayer {
    pub speed: f32,
    pub aggro_range: f32,
    pub attack_range: f32,
}

impl Default for ChasePlayer {
    fn default() -> Self {
        Self {
            speed: 3.0,
            aggro_range: 30.0,
            attack_range: 2.0,
        }
    }
}

/// Фаза волны
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WavePhase {
    Spawning,   // спавним врагов по одному
    Fighting,   // все заспавнены, ждём пока убьют
    Cooldown,   // пауза перед следующей волной
}

/// Состояние волновой системы спавна
#[derive(Resource)]
pub struct WaveState {
    pub current_wave: u32,
    pub enemies_to_spawn: u32,
    pub spawn_timer: Timer,
    pub wave_cooldown: Timer,
    pub phase: WavePhase,
}

impl Default for WaveState {
    fn default() -> Self {
        let mut cooldown = Timer::from_seconds(3.0, TimerMode::Once);
        cooldown.finish(); // Первая волна стартует сразу
        Self {
            current_wave: 0,
            enemies_to_spawn: 0,
            spawn_timer: Timer::from_seconds(0.8, TimerMode::Repeating),
            wave_cooldown: cooldown,
            phase: WavePhase::Cooldown,
        }
    }
}
