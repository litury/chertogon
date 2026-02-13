use bevy::prelude::*;
use crate::shared::GameState;
use crate::modules::player::parts::movement::player_movement_system;
use super::parts::{setup, follow_system, menu_camera};
use super::components::CameraZoom;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CameraZoom>()
            .add_systems(PreStartup, setup::setup_camera)
            .add_systems(Update, (
                follow_system::camera_zoom_system,
                follow_system::follow_player_system,
            ).chain()
                .after(player_movement_system)
                .run_if(in_state(GameState::Playing)))
            .add_systems(Update, menu_camera::menu_camera_orbit_system
                .run_if(in_state(GameState::TitleScreen)));
    }
}
