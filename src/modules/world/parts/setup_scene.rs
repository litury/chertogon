use bevy::prelude::*;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::mesh::VertexAttributeValues;
use avian3d::prelude::*;
use bevy_firework::core::*;
use bevy_firework::curve::*;
use bevy_firework::emission_shape::EmissionShape;
use bevy_utilitarian::prelude::*;
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

    // Global Ambient Light (Gothic атмосфера с достаточной читаемостью персонажей)
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.3, 0.3, 0.35),
        brightness: 150.0,
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
        Collider::cuboid(50.0, 0.01, 50.0),
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

    // === ФАКЕЛЫ: 3D модель + частицы огня + PointLight ===
    let torch_scene = asset_server.load(asset_paths::TORCH);

    // (позиция на стене, поворот модели лицом внутрь)
    let torches: [(Vec3, f32); 4] = [
        (Vec3::new(-23.0, 3.0, -half), 0.0),
        (Vec3::new(23.0, 3.0, -half), 0.0),
        (Vec3::new(-23.0, 3.0, half), std::f32::consts::PI),
        (Vec3::new(23.0, 3.0, half), std::f32::consts::PI),
    ];

    for (i, (pos, angle)) in torches.iter().enumerate() {
        // Parent: позиция на стене
        let torch_parent = commands.spawn(
            Transform::from_translation(*pos),
        ).id();

        // Child 1: 3D модель факела (палка/кронштейн, без огня)
        let model = commands.spawn((
            SceneRoot(torch_scene.clone()),
            Transform::from_xyz(0.0, -0.5, 0.0)
                .with_scale(Vec3::splat(0.8))
                .with_rotation(Quat::from_rotation_y(*angle)),
        )).id();

        // Child 2: частицы огня + PointLight + мерцание (наверху факела)
        let fire = commands.spawn((
            ParticleSpawner {
                particle_settings: vec![ParticleSettings {
                    lifetime: RandF32 { min: 0.3, max: 0.8 },
                    initial_scale: RandF32 { min: 0.03, max: 0.1 },
                    scale_curve: FireworkCurve::uneven_samples(vec![
                        (0.0, 0.8), (0.2, 1.2), (1.0, 0.0),
                    ]),
                    acceleration: Vec3::new(0., 2.0, 0.),
                    linear_drag: 1.5,
                    base_color: FireworkGradient::uneven_samples(vec![
                        (0.0, LinearRgba::new(50., 40., 5., 1.0)),
                        (0.3, LinearRgba::new(10., 5., 0.5, 0.9)),
                        (0.6, LinearRgba::new(3., 0.8, 0.1, 0.7)),
                        (0.8, LinearRgba::new(1., 0.2, 0.05, 0.4)),
                        (1.0, LinearRgba::new(0.2, 0.1, 0.1, 0.0)),
                    ]),
                    emissive_color: FireworkGradient::uneven_samples(vec![
                        (0.0, LinearRgba::new(30., 20., 2., 1.0)),
                        (0.5, LinearRgba::new(5., 1., 0.1, 1.0)),
                        (1.0, LinearRgba::BLACK),
                    ]),
                    blend_mode: BlendMode::Add,
                    fade_edge: 0.8,
                    pbr: false,
                    ..default()
                }],
                emission_settings: vec![EmissionSettings {
                    emission_pacing: EmissionPacing::rate(300.),
                    emission_shape: EmissionShape::Sphere(0.12),
                    initial_velocity: RandVec3 {
                        magnitude: RandF32 { min: 0.2, max: 1.0 },
                        direction: Vec3::Y,
                        spread: 45_f32.to_radians(),
                    },
                    initial_velocity_radial: RandF32 { min: 0.1, max: 0.5 },
                    ..default()
                }],
                ..default()
            },
            PointLight {
                color: Color::srgb(1.0, 0.6, 0.2),
                intensity: 200_000.0,
                range: 15.0,
                shadows_enabled: false,
                ..default()
            },
            TorchFlicker {
                base_intensity: 200_000.0,
                flicker_amount: 40_000.0,
                speed: 4.0,
                phase: i as f32 * 1.5,
            },
            Transform::from_xyz(0.0, -0.1, 0.0),
        )).id();

        commands.entity(torch_parent).add_children(&[model, fire]);
    }

    info!("✅ Arena setup complete: 50x50m with walls, torches, Gothic lighting");
}
