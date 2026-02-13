use bevy::prelude::*;

/// Множитель дистанции промаха (1.1× от attack_range — минимальный запас на jitter физики)
pub const MISS_RANGE_MULTIPLIER: f32 = 1.1;

/// Оружие игрока (автоатака à la Vampire Survivors)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Weapon {
    pub damage: f32,
    pub range: f32,
    pub cooldown: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            damage: 10.0,
            range: 3.0,
            cooldown: 1.0,
        }
    }
}

/// Таймер перезарядки автоатаки
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AttackCooldown {
    pub timer: Timer,
}

impl AttackCooldown {
    pub fn new(cooldown: f32) -> Self {
        let mut timer = Timer::from_seconds(cooldown, TimerMode::Once);
        timer.finish(); // Готов к первой атаке сразу
        Self { timer }
    }
}

/// Таймер длительности анимации атаки (для сброса обратно в idle/walk)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AttackAnimTimer {
    pub timer: Timer,
}

/// HP игрока
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerHealth {
    pub current: f32,
    pub max: f32,
}

impl PlayerHealth {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}

/// Отложенный удар — урон наносится при ударе анимации, а не в начале замаха
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PendingAttack {
    pub target: Entity,
    pub damage: f32,
    pub direction: Vec3,
    pub timer: Timer,
    /// Максимальная дистанция для попадания (weapon.range * MISS_RANGE_MULTIPLIER)
    pub max_range: f32,
}

/// Таймер контактного урона врага (чтобы не бил каждый кадр)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EnemyAttackCooldown {
    pub timer: Timer,
    pub damage: f32,
    /// Максимальная дистанция для попадания (attack_range * MISS_RANGE_MULTIPLIER)
    pub max_range: f32,
}

impl EnemyAttackCooldown {
    /// Создаёт с явным указанием attack_range (miss tolerance = 1.3×)
    pub fn new(damage: f32, cooldown: f32, attack_range: f32) -> Self {
        Self {
            timer: Timer::from_seconds(cooldown, TimerMode::Once),
            damage,
            max_range: attack_range * MISS_RANGE_MULTIPLIER,
        }
    }
}
