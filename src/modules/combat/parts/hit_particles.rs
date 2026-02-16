use bevy::prelude::*;
use super::vfx_assets::HitVfxAssets;

/// Маркер частицы попадания
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HitParticle {
    pub velocity: Vec3,
    pub timer: Timer,
}

/// Спавнит искры при попадании по врагу (использует кэшированные ассеты)
pub fn spawn_hit_particles(
    commands: &mut Commands,
    vfx_assets: &HitVfxAssets,
    hit_pos: Vec3,
) {
    // 3 частицы (было 6 — оптимизация: меньше entity churn при массовых боях)
    let directions = [
        Vec3::new(1.0, 2.0, 0.5),
        Vec3::new(-0.8, 1.5, 0.3),
        Vec3::new(0.3, 2.5, -0.7),
    ];

    for dir in directions {
        let speed = 3.0 + dir.y * 0.5;
        commands.spawn((
            Mesh3d(vfx_assets.particle_mesh.clone()),
            MeshMaterial3d(vfx_assets.particle_material.clone()),
            Transform::from_translation(hit_pos + Vec3::Y * 1.0),
            HitParticle {
                velocity: dir.normalize() * speed,
                timer: Timer::from_seconds(0.4, TimerMode::Once),
            },
        ));
    }
}

/// Движение и despawn частиц
pub fn hit_particle_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut HitParticle, &mut Transform)>,
) {
    let dt = time.delta_secs();
    let gravity = Vec3::new(0.0, -12.0, 0.0);

    for (entity, mut particle, mut transform) in &mut query {
        particle.timer.tick(time.delta());

        // Движение + гравитация
        particle.velocity += gravity * dt;
        transform.translation += particle.velocity * dt;

        // Уменьшение scale
        let progress = particle.timer.fraction();
        let scale = 1.0 - progress;
        transform.scale = Vec3::splat(scale.max(0.01));

        if particle.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
