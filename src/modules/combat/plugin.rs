use bevy::prelude::*;
use crate::modules::combat::parts::{auto_attack, enemy_damage, camera_shake, slash_vfx, hit_particles};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<camera_shake::CameraShake>()
            .add_systems(Update, (
                auto_attack::player_auto_attack_system,
                auto_attack::attack_animation_reset_system,
                enemy_damage::enemy_contact_damage_system,
                camera_shake::camera_shake_decay_system,
                slash_vfx::slash_vfx_system,
                hit_particles::hit_particle_system,
            ));

        info!("⚔️ CombatPlugin loaded (auto-attack + VFX + camera shake)");
    }
}
