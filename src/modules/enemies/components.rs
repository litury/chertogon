use bevy::prelude::*;

/// Маркер компонент для врага
#[derive(Component)]
pub struct Enemy;

/// Здоровье врага
#[derive(Component)]
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
#[derive(Component, Clone, Copy)]
pub enum EnemyType {
    Upyr,   // Упырь — славянский зомби (MVP)
}

/// Маркер для визуальной модели врага (child entity)
#[derive(Component)]
pub struct EnemyModel;

/// Индексы анимаций врага в AnimationGraph
#[derive(Component, Clone, Copy)]
pub struct EnemyAnimations {
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub attack: AnimationNodeIndex,
    pub death: AnimationNodeIndex,
}

/// Маркер завершения настройки анимаций врага
#[derive(Component)]
pub struct EnemyAnimationSetupComplete;

/// Состояние анимации врага
#[derive(Component)]
pub struct EnemyAnimState {
    pub current: EnemyAnim,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnemyAnim {
    Idle,
    Walking,
    Attacking,
    Dying,
}

/// Маркер умирающего врага (проигрывает death анимацию перед despawn)
#[derive(Component)]
pub struct EnemyDying {
    pub timer: Timer,
}

/// AI врага: преследование + агро-зона + атака
#[derive(Component)]
pub struct ChasePlayer {
    pub speed: f32,
    pub aggro_range: f32,
    pub attack_range: f32,
}

impl Default for ChasePlayer {
    fn default() -> Self {
        Self {
            speed: 3.0,
            aggro_range: 12.0,
            attack_range: 2.0,
        }
    }
}
