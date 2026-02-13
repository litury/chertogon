use bevy::prelude::*;
use crate::shared::GameState;
use crate::modules::combat::parts::{
    auto_attack, enemy_damage, camera_shake, slash_vfx, hit_particles,
    game_over, game_timer, knockback, hit_flash, damage_numbers,
    impact_flash, damage_vignette, vfx_assets,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<camera_shake::CameraShake>()
            .init_resource::<game_over::KillCount>()
            .init_resource::<game_timer::GameTimer>()
            .init_resource::<damage_vignette::DamageVignette>()
            .add_systems(Update, (
                auto_attack::player_auto_attack_system,
                auto_attack::apply_pending_attack_system,
                auto_attack::attack_animation_reset_system,
                enemy_damage::enemy_contact_damage_system,
                camera_shake::camera_shake_decay_system,
                slash_vfx::slash_vfx_system,
                slash_vfx::vfx_billboard_system,
                game_over::check_game_over_system,
                game_timer::tick_game_timer,
                knockback::stagger_decay_system,
                knockback::recovery_decay_system,
                hit_flash::hit_flash_system,
                hit_particles::hit_particle_system,
                impact_flash::impact_flash_system,
                damage_numbers::damage_number_system,
                damage_vignette::damage_vignette_decay_system,
                damage_vignette::damage_vignette_apply_system,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Playing), (
                game_over::reset_on_enter,
                game_timer::reset_game_timer,
                vfx_assets::init_hit_vfx_assets,
            ))
            .add_systems(OnEnter(GameState::GameOver),
                damage_vignette::reset_color_grading,
            );

        info!("⚔️ CombatPlugin loaded (auto-attack + VFX + hit effects)");
    }
}
