use bevy::prelude::*;
use super::parts::{setup, follow_system};
use super::components::CameraZoom;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CameraZoom>()
            .add_systems(Startup, setup::setup_camera)
            .add_systems(Update, (
                follow_system::camera_zoom_system,
                follow_system::follow_player_system,
            ).chain());
    }
}
