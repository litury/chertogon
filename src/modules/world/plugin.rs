use bevy::prelude::*;
use bevy::pbr::MaterialPlugin;
use crate::shared::GameState;
use super::parts::setup_scene;
use super::parts::torch_flicker;
use super::parts::ground_circle;
use super::parts::stylized_material;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<stylized_material::StylizedMaterial>::default())
            .add_systems(Startup, setup_scene::setup_scene)
            .add_systems(Update, torch_flicker::torch_flicker_system)
            .add_systems(Update, stylized_material::replace_character_materials)
            .add_systems(Update, ground_circle::health_ring_system
                .run_if(in_state(GameState::Playing)));
    }
}
