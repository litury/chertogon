use bevy::prelude::*;

/// Оружие игрока (автоатака à la Vampire Survivors)
#[derive(Component)]
pub struct Weapon {
    pub damage: f32,
    pub range: f32,
    pub cooldown: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            damage: 5.0,
            range: 2.5,
            cooldown: 1.0,
        }
    }
}

/// Таймер перезарядки автоатаки
#[derive(Component)]
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
#[derive(Component)]
pub struct AttackAnimTimer {
    pub timer: Timer,
}

/// HP игрока
#[derive(Component)]
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

/// Таймер контактного урона врага (чтобы не бил каждый кадр)
#[derive(Component)]
pub struct EnemyAttackCooldown {
    pub timer: Timer,
    pub damage: f32,
}

impl EnemyAttackCooldown {
    pub fn new(damage: f32, cooldown: f32) -> Self {
        let mut timer = Timer::from_seconds(cooldown, TimerMode::Once);
        timer.finish(); // Первый удар сразу
        Self { timer, damage }
    }
}
