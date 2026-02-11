use bevy::prelude::*;
use super::parts::{spawner, movement, animation};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawner::spawn_player)
            .add_systems(Update, (
                spawner::setup_scene_animation,
                movement::player_movement_system,
                animation::animation_state_system,
            ));
    }
}
