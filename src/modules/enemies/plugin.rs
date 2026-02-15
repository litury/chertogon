use bevy::prelude::*;
use crate::shared::GameState;
use crate::modules::enemies::parts::{spawner, ai, animation, cleanup, preload, spatial_grid, separation, attack_slots, portal_spawn, portal_vfx};
use crate::modules::enemies::components::{WaveState, AttackSlotManager};
use crate::modules::enemies::parts::spatial_grid::SpatialGrid;
use crate::modules::enemies::parts::portal_vfx::PortalEmitTimer;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WaveState>()
            .init_resource::<AttackSlotManager>()
            .init_resource::<SpatialGrid>()
            .init_resource::<PortalEmitTimer>()
            .add_systems(OnEnter(GameState::Playing), (
                cleanup::despawn_enemies,
                cleanup::reset_wave_state,
                cleanup::reset_kill_count,
                preload::preload_enemy_assets,
                portal_vfx::init_portal_vfx_assets,
            ).chain())
            .add_systems(Update, spawner::wave_spawner_system
                .run_if(in_state(GameState::Playing)))
            .add_systems(Update, (
                spatial_grid::rebuild_spatial_grid_system,
                ai::enemy_ai_system,
                separation::enemy_separation_system,
                attack_slots::attack_slot_system,
                attack_slots::release_attack_slot_system,
                ai::start_enemy_death,
                ai::process_dying_enemies,
                ai::strip_corpse_system,
                animation::enemy_animation_state_system,
                animation::enemy_attack_anim_replay_system,
            ).chain().in_set(crate::modules::enemies::components::EnemyCoreSet).run_if(in_state(GameState::Playing)))
            .add_systems(Update, (
                animation::setup_enemy_animation,
                animation::spawn_scream_decay_system,
                portal_spawn::portal_spawn_anim_system,
                portal_vfx::portal_particle_emitter_system,
                portal_vfx::portal_smoke_system,
                portal_vfx::portal_spark_system,
            ).after(spawner::wave_spawner_system)
             .run_if(in_state(GameState::Playing)));

        info!("👾 EnemiesPlugin loaded (wave system + portals + animations)");
    }
}
