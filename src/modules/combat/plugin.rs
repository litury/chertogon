use bevy::prelude::*;
use crate::shared::GameState;
use crate::modules::combat::parts::{
    auto_attack, enemy_damage, camera_shake, slash_vfx,
    game_over, game_timer, hitstop, knockback, hit_flash, damage_numbers,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<camera_shake::CameraShake>()
            .init_resource::<game_over::KillCount>()
            .init_resource::<game_timer::GameTimer>()
            .init_resource::<hitstop::Hitstop>()
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
                hitstop::hitstop_system,
                knockback::stagger_decay_system,
                hit_flash::hit_flash_system,
                damage_numbers::damage_number_system,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Playing), (
                game_over::reset_on_enter,
                game_timer::reset_game_timer,
            ));

        info!("⚔️ CombatPlugin loaded (auto-attack + VFX + hit effects)");
    }
}
