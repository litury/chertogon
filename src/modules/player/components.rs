use bevy::prelude::*;

/// Маркер компонент игрока
#[derive(Component)]
pub struct Player;

/// Компонент анимированного персонажа
#[derive(Component)]
pub struct AnimatedCharacter {
    pub current_animation: AnimationState,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
#[derive(Component)]
pub struct PlayerModel;

/// Маркер завершения настройки анимаций
/// Добавляется к AnimationPlayer entity после успешной инициализации
#[derive(Component)]
pub struct AnimationSetupComplete;
