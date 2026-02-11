use bevy::prelude::*;
use super::parts::setup_scene;
use super::parts::torch_flicker;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene::setup_scene)
            .add_systems(Update, torch_flicker::torch_flicker_system);
    }
}
