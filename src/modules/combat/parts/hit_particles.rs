use bevy::prelude::*;

/// Маркер частицы попадания
#[derive(Component)]
pub struct HitParticle {
    pub velocity: Vec3,
    pub timer: Timer,
}

/// Спавнит искры при попадании по врагу
pub fn spawn_hit_particles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    hit_pos: Vec3,
) {
    let mesh = meshes.add(Sphere::new(0.06));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.4, 0.1),
        emissive: LinearRgba::new(8.0, 2.0, 0.3, 1.0),
        unlit: true,
        ..default()
    });

    // 6 частиц в случайных направлениях
    let directions = [
        Vec3::new(1.0, 2.0, 0.5),
        Vec3::new(-0.8, 1.5, 0.3),
        Vec3::new(0.3, 2.5, -0.7),
        Vec3::new(-0.5, 1.8, -0.4),
        Vec3::new(0.7, 1.2, 0.8),
        Vec3::new(-0.3, 2.2, -0.6),
    ];

    for dir in directions {
        let speed = 3.0 + dir.y * 0.5;
        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
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
