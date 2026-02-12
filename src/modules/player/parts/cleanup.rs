use bevy::prelude::*;
use crate::modules::player::components::Player;
use crate::modules::input::InputState;

/// Удаляет игрока и всех его child-entity (модель, круг, свет)
pub fn despawn_player(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
) {
    for entity in &players {
        commands.entity(entity).despawn();
    }
}

/// Сбрасывает ввод чтобы игрок не двигался с прошлой сессии
pub fn reset_input(mut input: ResMut<InputState>) {
    *input = InputState::default();
}
