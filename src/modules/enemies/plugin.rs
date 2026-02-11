use bevy::prelude::*;
use crate::modules::enemies::parts::{spawner, ai, animation};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            // Спавн тестового врага при старте
            .add_systems(Startup, spawner::spawn_test_enemy)
            // AI → Death → Animation (строгий порядок через chain)
            .add_systems(Update, (
                ai::enemy_ai_system,
                ai::start_enemy_death,
                animation::enemy_animation_state_system,
            ).chain())
            // Независимые системы
            .add_systems(Update, (
                animation::setup_enemy_animation,
            ));

        info!("👾 EnemiesPlugin loaded (with animations)");
    }
}
