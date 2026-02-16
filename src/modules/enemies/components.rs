use bevy::prelude::*;

/// SystemSet для основного цикла AI врагов (chase, death, animation)
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemyCoreSet;

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

/// Кэш Entity AnimationPlayer — избегаем обход иерархии каждый кадр.
/// Заполняется один раз в setup_enemy_animation.
#[derive(Component)]
pub struct CachedAnimPlayer {
    pub entity: Entity,
}

/// LOD уровень врага (обновляется по дистанции до игрока)
#[derive(Component, Reflect, Clone, Copy, PartialEq, Eq, Default)]
#[reflect(Component)]
pub enum EnemyLod {
    #[default]
    Full,      // <15м: полная анимация, ground circle, всё включено
    Reduced,   // 15-25м: ground circle скрыт
    Minimal,   // >25м: анимация заморожена + circle скрыт
}

/// Кэш последнего применённого speed_factor анимации.
/// Обновляем set_speed() только при изменении > 5% (экономим change detection).
#[derive(Component, Default)]
pub struct CachedAnimSpeed {
    pub last_factor: f32,
}

/// Состояние анимации врага.
/// Системы только меняют `current`, центральная система обрабатывает переходы.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EnemyAnimState {
    pub current: EnemyAnim,
    /// Последняя применённая анимация — защита от self-transitions (Bevy #13910).
    /// None = "грязное" состояние, нужен переход (используется для replay атаки).
    #[reflect(ignore)]
    previous: Option<EnemyAnim>,
}

impl EnemyAnimState {
    pub fn new(initial: EnemyAnim) -> Self {
        Self { current: initial, previous: Some(initial) }
    }

    /// Есть ли новый переход для применения
    pub fn needs_transition(&self) -> bool {
        self.previous != Some(self.current)
    }

    /// Отметить текущий переход как применённый
    pub fn mark_applied(&mut self) {
        self.previous = Some(self.current);
    }

    /// Форсировать повторный переход (для replay атаки)
    pub fn request_replay(&mut self) {
        self.previous = None;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Reflect)]
pub enum EnemyAnim {
    #[default]
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
    /// Скорость движения, при которой walk-анимация выглядит натурально при 1.0x
    pub anim_base_speed: f32,
}

impl Default for ChasePlayer {
    fn default() -> Self {
        Self {
            speed: 3.0,
            aggro_range: 30.0,
            attack_range: 2.0,
            anim_base_speed: 3.0,
        }
    }
}

/// Направление орбитирования вокруг цели (CW/CCW)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct OrbitDirection {
    pub clockwise: bool,
    pub change_timer: Timer,
}

/// Маркер: враг получил слот на ближнюю атаку
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HasAttackSlot;

/// Менеджер слотов — лимитирует одновременных атакующих (Diablo-style)
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct AttackSlotManager {
    pub max_slots: u32,
}

impl Default for AttackSlotManager {
    fn default() -> Self {
        Self { max_slots: 4 }
    }
}

/// Маркер портала-разлома ("Разлом Нави")
#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct SpawnPortal {
    pub index: u8, // 0 = "Разлом Огня", 1 = "Разлом Тьмы"
}

/// Маркер визуальной воронки портала (child entity)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PortalVortex;

/// Маркер точечного света портала (child entity)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PortalLight;

/// Анимация появления врага из портала (масштаб 0→1 за 0.5с)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PortalSpawnAnim {
    pub timer: Timer,
}

impl PortalSpawnAnim {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
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
    /// Счётчик врагов, отправленных в портал 0 (для балансировки ~50/50)
    pub portal_0_count: u32,
    /// Счётчик врагов, отправленных в портал 1
    pub portal_1_count: u32,
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
            portal_0_count: 0,
            portal_1_count: 0,
        }
    }
}
