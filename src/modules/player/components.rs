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
}

/// Компонент для хранения индексов нод анимаций в графе
/// (подобно ID элементов в DOM дереве)
#[derive(Component, Clone, Copy)]
pub struct PlayerAnimations {
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub run: AnimationNodeIndex,
    pub attack: AnimationNodeIndex,
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

/// Маркер для entity оружия (child кости RightHand)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WeaponModel;

/// Маркер завершения крепления оружия к кости
/// Добавляется к PlayerModel после успешного attachment
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WeaponAttachmentComplete;
