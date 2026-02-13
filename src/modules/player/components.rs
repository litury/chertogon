use bevy::prelude::*;

/// Маркер компонент игрока
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

/// Компонент анимированного персонажа
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AnimatedCharacter {
    pub current_animation: AnimationState,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Reflect)]
pub enum AnimationState {
    Idle,
    Walking,
    Running,
    Attacking,
    HitReaction,
}

/// Компонент для хранения индексов нод анимаций в графе
/// (подобно ID элементов в DOM дереве)
#[derive(Component, Clone, Copy)]
pub struct PlayerAnimations {
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub run: AnimationNodeIndex,
    pub attack: AnimationNodeIndex,
    pub hit: AnimationNodeIndex,
}

/// Маркер для визуальной модели игрока (child entity)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerModel;

/// Маркер завершения настройки анимаций
/// Добавляется к AnimationPlayer entity после успешной инициализации
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AnimationSetupComplete;

/// Таймер стаггера при получении урона (блокирует ввод, играет hit анимацию)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerHitStagger {
    pub timer: Timer,
    /// Emissive уже выставлен (set-once оптимизация)
    #[reflect(ignore)]
    pub emissive_applied: bool,
}

/// Окно иммунитета к повторному стаггеру (0.5с после выхода из HitReaction)
/// Урон проходит, но новый стаггер не срабатывает — даёт окно для атаки
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct StaggerCooldown {
    pub timer: Timer,
}

/// Модифицируемые характеристики игрока (базовые + бонусы от апгрейдов)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerStats {
    pub move_speed_multiplier: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            move_speed_multiplier: 1.0,
        }
    }
}

/// Маркер для entity оружия (child кости RightHand)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WeaponModel;

/// Маркер завершения крепления оружия к кости
/// Добавляется к PlayerModel после успешного attachment
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WeaponAttachmentComplete;
