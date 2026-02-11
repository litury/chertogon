use bevy::prelude::*;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::mesh::VertexAttributeValues;
use avian3d::prelude::*;
use crate::toolkit::asset_paths;
use super::torch_flicker::TorchFlicker;

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Directional Light (холодное солнце, по дизайн-доку)
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.9, 0.9, 1.0),
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Global Ambient Light (мрачная Gothic атмосфера, по дизайн-доку)
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.25, 0.25, 0.3),
        brightness: 80.0,
        ..default()
    });

    // Пол арены: один Plane3d 50x50м + seamless PBR текстура (Polyhaven stone_tiles)
    let tile_repeat = 10.0; // текстура повторяется 10x10 раз (каждый тайл ~5x5м)
    let mut floor_mesh: Mesh = Plane3d::default().mesh().size(50.0, 50.0).into();

    // Масштабируем UV для тайлинга
    if let Some(VertexAttributeValues::Float32x2(uvs)) = floor_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        for uv in uvs.iter_mut() {
            uv[0] *= tile_repeat;
            uv[1] *= tile_repeat;
        }
    }

    // Загружаем PBR текстуры с режимом Repeat
    let sampler_repeat = |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            ..default()
        });
    };

    let floor_diff = asset_server.load_with_settings(asset_paths::FLOOR_DIFF, sampler_repeat);
    let floor_normal = asset_server.load_with_settings(asset_paths::FLOOR_NORMAL, sampler_repeat);

    commands.spawn((
        Mesh3d(meshes.add(floor_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(floor_diff),
            normal_map_texture: Some(floor_normal),
            metallic: 0.05,
            perceptual_roughness: 0.85,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(25.0, 0.01, 25.0),
        crate::shared::GameLayer::static_layers(),
    ));

    // === СТЕНЫ АРЕНЫ: модульные GLB панели из Meshy ===
    info!("🏗️ Creating arena walls...");

    let wall_scene = asset_server.load(asset_paths::WALL_PANEL);
    let half = 25.0;
    let panel_size = 5.0;
    let num_panels = 10; // 50м / 5м = 10 панелей на стену
    let wall_height = 5.0;
    // GLB bbox: ~2.0×1.5×0.32 → масштаб до 5×5×1м
    let wall_scale = Vec3::new(2.5, 3.33, 3.0);

    // Северная стена (Z+) — панели вдоль X
    for i in 0..num_panels {
        let x = -half + panel_size * 0.5 + i as f32 * panel_size;
        commands.spawn((
            SceneRoot(wall_scene.clone()),
            Transform::from_xyz(x, 0.0, half).with_scale(wall_scale),
            RigidBody::Static,
            Collider::cuboid(panel_size / 2.0, wall_height / 2.0, 0.5),
            crate::shared::GameLayer::static_layers(),
        ));
    }

    // Южная стена (Z-)
    for i in 0..num_panels {
        let x = -half + panel_size * 0.5 + i as f32 * panel_size;
        commands.spawn((
            SceneRoot(wall_scene.clone()),
            Transform::from_xyz(x, 0.0, -half).with_scale(wall_scale),
            RigidBody::Static,
            Collider::cuboid(panel_size / 2.0, wall_height / 2.0, 0.5),
            crate::shared::GameLayer::static_layers(),
        ));
    }

    // Западная стена (X-) — панели вдоль Z, повёрнуты на 90°
    for i in 0..num_panels {
        let z = -half + panel_size * 0.5 + i as f32 * panel_size;
        commands.spawn((
            SceneRoot(wall_scene.clone()),
            Transform::from_xyz(-half, 0.0, z)
                .with_scale(wall_scale)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            RigidBody::Static,
            Collider::cuboid(panel_size / 2.0, wall_height / 2.0, 0.5),
            crate::shared::GameLayer::static_layers(),
        ));
    }

    // Восточная стена (X+)
    for i in 0..num_panels {
        let z = -half + panel_size * 0.5 + i as f32 * panel_size;
        commands.spawn((
            SceneRoot(wall_scene.clone()),
            Transform::from_xyz(half, 0.0, z)
                .with_scale(wall_scale)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            RigidBody::Static,
            Collider::cuboid(panel_size / 2.0, wall_height / 2.0, 0.5),
            crate::shared::GameLayer::static_layers(),
        ));
    }

    // === ФАКЕЛЫ: 4 PointLight + 3D модель + мерцание огня ===
    // Прижаты к стенам, повёрнуты лицом внутрь арены
    let torch_scene = asset_server.load(asset_paths::TORCH);

    // (позиция на стене, поворот модели лицом внутрь)
    let torches: [(Vec3, f32); 4] = [
        (Vec3::new(-23.0, 3.0, -half), 0.0),                        // SW → южная стена, лицом Z+
        (Vec3::new(23.0, 3.0, -half), 0.0),                         // SE → южная стена, лицом Z+
        (Vec3::new(-23.0, 3.0, half), std::f32::consts::PI),        // NW → северная стена, лицом Z-
        (Vec3::new(23.0, 3.0, half), std::f32::consts::PI),         // NE → северная стена, лицом Z-
    ];

    for (i, (pos, angle)) in torches.iter().enumerate() {
        let torch_entity = commands.spawn((
            PointLight {
                color: Color::srgb(1.0, 0.6, 0.2),
                intensity: 200_000.0,
                range: 15.0,
                shadows_enabled: true,
                ..default()
            },
            TorchFlicker {
                base_intensity: 200_000.0,
                flicker_amount: 40_000.0,
                speed: 4.0,
                phase: i as f32 * 1.5,
            },
            Transform::from_translation(*pos),
        )).id();

        // Child: визуальная модель факела, повёрнута лицом внутрь арены
        let model = commands.spawn((
            SceneRoot(torch_scene.clone()),
            Transform::from_xyz(0.0, -0.5, 0.0)
                .with_scale(Vec3::splat(0.8))
                .with_rotation(Quat::from_rotation_y(*angle)),
        )).id();

        commands.entity(torch_entity).add_child(model);
    }

    info!("✅ Arena setup complete: 50x50m with walls, torches, Gothic lighting");
}
