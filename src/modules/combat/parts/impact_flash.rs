use bevy::prelude::*;
use super::vfx_assets::HitVfxAssets;

/// Кратковременная вспышка в точке удара (emissive mesh вместо PointLight)
#[derive(Component)]
pub struct ImpactFlash {
    pub timer: Timer,
}

/// Спавнит яркую emissive сферу на 0.1с в точке удара
pub fn spawn_impact_flash(
    commands: &mut Commands,
    vfx_assets: &HitVfxAssets,
    hit_pos: Vec3,
) {
    commands.spawn((
        Mesh3d(vfx_assets.flash_mesh.clone()),
        MeshMaterial3d(vfx_assets.flash_material.clone()),
        Transform::from_translation(hit_pos + Vec3::Y * 1.0),
        ImpactFlash {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
    ));
}

/// Быстрое затухание через scale и despawn
pub fn impact_flash_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut ImpactFlash, &mut Transform)>,
) {
    for (entity, mut flash, mut transform) in &mut query {
        flash.timer.tick(time.delta());

        let progress = flash.timer.fraction();
        // Быстрое квадратичное затухание через scale (дешёвая Transform мутация)
        let fade = (1.0 - progress) * (1.0 - progress);
        transform.scale = Vec3::splat(fade.max(0.01));

        if flash.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
