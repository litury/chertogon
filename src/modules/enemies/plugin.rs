use bevy::prelude::*;
use crate::shared::GameState;
use crate::modules::enemies::parts::{spawner, ai, animation, cleanup};
use crate::modules::enemies::components::WaveState;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WaveState>()
            .add_systems(OnEnter(GameState::Playing), (
                cleanup::despawn_enemies,
                cleanup::reset_wave_state,
                cleanup::reset_kill_count,
            ).chain())
            .add_systems(Update, spawner::wave_spawner_system
                .run_if(in_state(GameState::Playing)))
            .add_systems(Update, (
                ai::enemy_ai_system,
                ai::start_enemy_death,
                ai::process_dying_enemies,
                animation::enemy_animation_state_system,
                animation::enemy_attack_anim_replay_system,
            ).chain().run_if(in_state(GameState::Playing)))
            .add_systems(Update, (
                animation::setup_enemy_animation,
                animation::spawn_scream_decay_system,
            ).run_if(in_state(GameState::Playing)));

        info!("👾 EnemiesPlugin loaded (wave system + animations)");
    }
}
