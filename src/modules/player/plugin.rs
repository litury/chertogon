use bevy::prelude::*;
use crate::shared::GameState;
use super::parts::{spawner, movement, animation, cleanup, weapon_attachment};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), (
                cleanup::despawn_player,
                cleanup::reset_input,
                spawner::spawn_player,
            ).chain())
            .add_systems(Update, (
                spawner::setup_scene_animation,
                weapon_attachment::attach_weapon_to_hand,
                movement::player_movement_system,
                animation::animation_state_system,
            ).run_if(in_state(GameState::Playing)));
    }
}
