use bevy::prelude::*;

/// Маркер пятна крови на полу (остаётся навсегда)
#[derive(Component)]
pub struct BloodDecal;

/// Спавнит пятно крови на полу в позиции врага.
/// Простой текстурированный квад чуть выше пола с текстурой blood_splatter.png.
pub fn spawn_blood_decal(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &AssetServer,
    position: Vec3,
) {
    let texture = asset_server.load("textures/vfx/blood_splat.png");

    // Псевдослучайность от позиции (детерминистично, без rand)
    let seed = position.x * 17.3 + position.z * 31.7;
    let random_angle = seed;
    let random_scale = 0.8 + (seed.sin().abs()) * 1.0; // 0.8 — 1.8

    let mesh = meshes.add(Plane3d::default().mesh().size(1.5, 1.5));
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture),
        base_color: Color::WHITE,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(position.x, 0.01, position.z)
            .with_rotation(Quat::from_rotation_y(random_angle))
            .with_scale(Vec3::splat(random_scale)),
        BloodDecal,
    ));
}
