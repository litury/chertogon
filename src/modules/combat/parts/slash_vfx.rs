use bevy::prelude::*;
use bevy::math::Mat3;

/// Пути к кадрам анимации огненной дуги (CC0, OpenGameArt)
const SLASH_FRAMES: [&str; 6] = [
    "textures/vfx/slash/Alternative_3_01.png",
    "textures/vfx/slash/Alternative_3_02.png",
    "textures/vfx/slash/Alternative_3_03.png",
    "textures/vfx/slash/Alternative_3_04.png",
    "textures/vfx/slash/Alternative_3_05.png",
    "textures/vfx/slash/Alternative_3_06.png",
];

/// Маркер slash-эффекта с покадровой анимацией
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SlashVfx {
    pub timer: Timer,
}

/// Маркер: квад всегда повёрнут лицом к камере
#[derive(Component)]
pub struct VfxBillboard;

/// Поворачивает VFX квады лицом к камере (billboard)
pub fn vfx_billboard_system(
    camera_q: Query<&Transform, With<Camera3d>>,
    mut billboards: Query<&mut Transform, (With<VfxBillboard>, Without<Camera3d>)>,
) {
    let Ok(cam) = camera_q.single() else { return };
    for mut t in &mut billboards {
        let dir = (cam.translation - t.translation).normalize();
        let up = Vec3::Y;
        let right = dir.cross(up).normalize();
        let corrected_up = right.cross(dir);
        t.rotation = Quat::from_mat3(&Mat3::from_cols(right, corrected_up, dir));
    }
}

/// Спавнит slash VFX перед игроком в направлении врага
pub fn spawn_slash(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &AssetServer,
    player_pos: Vec3,
    direction: Vec3,
) {
    let slash_pos = player_pos + direction * 0.8 + Vec3::Y * 0.8;

    // Первый кадр анимации
    let texture = asset_server.load(SLASH_FRAMES[0]);

    // Квад с соотношением сторон текстуры (126x150 → ~0.84:1)
    let mesh = meshes.add(Plane3d::default().mesh().size(1.7, 2.0));
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture),
        base_color: Color::WHITE,
        emissive: LinearRgba::new(5.0, 3.0, 0.5, 1.0),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(slash_pos),
        SlashVfx {
            timer: Timer::from_seconds(0.25, TimerMode::Once),
        },
        VfxBillboard,
    ));
}

/// Покадровая анимация + fade out + despawn
pub fn slash_vfx_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut SlashVfx, &mut Transform, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut vfx, mut transform, material_handle) in &mut query {
        vfx.timer.tick(time.delta());

        let progress = vfx.timer.fraction();

        // Покадровая анимация: 6 кадров за 0.25с
        let frame_index = ((progress * SLASH_FRAMES.len() as f32) as usize)
            .min(SLASH_FRAMES.len() - 1);

        // Небольшое увеличение масштаба (1.0 → 1.3)
        let scale = 1.0 + progress * 0.3;
        transform.scale = Vec3::splat(scale);

        if let Some(material) = materials.get_mut(&material_handle.0) {
            // Смена кадра
            material.base_color_texture = Some(asset_server.load(SLASH_FRAMES[frame_index]));

            // Fade out в последние 30% анимации
            let alpha = if progress > 0.7 {
                1.0 - (progress - 0.7) / 0.3
            } else {
                1.0
            };
            material.base_color = Color::srgba(1.0, 1.0, 1.0, alpha);

            // Emissive тоже затухает
            let em = 1.0 - progress * 0.5;
            material.emissive = LinearRgba::new(5.0 * em, 3.0 * em, 0.5 * em, 1.0);
        }

        if vfx.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
