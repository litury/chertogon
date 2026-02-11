use bevy::prelude::*;

/// Маркер slash-эффекта
#[derive(Component)]
pub struct SlashVfx {
    pub timer: Timer,
    pub initial_alpha: f32,
}

/// Спавнит slash VFX перед игроком в направлении врага
pub fn spawn_slash(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    player_pos: Vec3,
    direction: Vec3,
) {
    let slash_pos = player_pos + direction * 1.2 + Vec3::Y * 1.0;

    // Поворот: плоскость перпендикулярна направлению атаки
    let rotation = if direction.length() > 0.01 {
        Quat::from_rotation_y(direction.x.atan2(direction.z))
            * Quat::from_rotation_x(-0.3) // Лёгкий наклон
    } else {
        Quat::IDENTITY
    };

    let mesh = meshes.add(Plane3d::default().mesh().size(1.5, 0.6));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.7, 0.2, 0.8),
        emissive: LinearRgba::new(6.0, 3.0, 0.5, 1.0),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(slash_pos)
            .with_rotation(rotation)
            .with_scale(Vec3::splat(0.5)),
        SlashVfx {
            timer: Timer::from_seconds(0.25, TimerMode::Once),
            initial_alpha: 0.8,
        },
    ));
}

/// Анимация и despawn slash VFX
pub fn slash_vfx_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut SlashVfx, &mut Transform, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut vfx, mut transform, material_handle) in &mut query {
        vfx.timer.tick(time.delta());

        let progress = vfx.timer.fraction();

        // Scale up: 0.5 → 2.0
        let scale = 0.5 + progress * 1.5;
        transform.scale = Vec3::splat(scale);

        // Fade out alpha
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let alpha = vfx.initial_alpha * (1.0 - progress);
            material.base_color = Color::srgba(1.0, 0.7, 0.2, alpha);
        }

        if vfx.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
