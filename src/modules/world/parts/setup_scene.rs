use bevy::prelude::*;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::mesh::VertexAttributeValues;
use avian3d::prelude::*;
use bevy::light::DirectionalLightShadowMap;
use crate::toolkit::asset_paths;
use super::torch_flicker::TorchFlicker;

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // === Directional Light (луна / холодный свет) ===
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.85, 0.85, 1.0),
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        bevy::light::CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 20.0,
            ..default()
        }.build(),
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // === Fill Light (rim подсветка персонажей) ===
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.35, 0.35, 0.6),
            illuminance: 400.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(-5.0, 15.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // === Global Ambient Light ===
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.2, 0.18, 0.22),
        brightness: 200.0,
        ..default()
    });

    // Пол арены: Plane3d 50x50м + seamless PBR текстура
    let tile_repeat = 10.0;
    let mut floor_mesh: Mesh = Plane3d::default().mesh().size(50.0, 50.0).into();

    if let Some(VertexAttributeValues::Float32x2(uvs)) = floor_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        for uv in uvs.iter_mut() {
            uv[0] *= tile_repeat;
            uv[1] *= tile_repeat;
        }
    }

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
            metallic: 0.0,
            perceptual_roughness: 1.0,
            reflectance: 0.0,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(50.0, 0.01, 50.0),
        crate::shared::GameLayer::static_layers(),
    ));

    // === СТЕНЫ АРЕНЫ ===
    let wall_scene = asset_server.load(asset_paths::WALL_PANEL);
    let half = 25.0;
    let panel_size = 5.0;
    let num_panels = 10;
    let wall_height = 5.0;
    let wall_scale = Vec3::new(2.5, 3.33, 3.0);

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

    // === ФАКЕЛЫ: 3D модель + PointLight + мерцание ===
    let torch_scene = asset_server.load(asset_paths::TORCH);

    let torches: [(Vec3, f32); 4] = [
        (Vec3::new(-23.0, 3.0, -half), 0.0),
        (Vec3::new(23.0, 3.0, -half), 0.0),
        (Vec3::new(-23.0, 3.0, half), std::f32::consts::PI),
        (Vec3::new(23.0, 3.0, half), std::f32::consts::PI),
    ];

    for (i, (pos, angle)) in torches.iter().enumerate() {
        let torch_parent = commands.spawn(
            Transform::from_translation(*pos),
        ).id();

        let model = commands.spawn((
            SceneRoot(torch_scene.clone()),
            Transform::from_xyz(0.0, -0.5, 0.0)
                .with_scale(Vec3::splat(0.8))
                .with_rotation(Quat::from_rotation_y(*angle)),
        )).id();

        let fire = commands.spawn((
            PointLight {
                color: Color::srgb(1.0, 0.6, 0.2),
                intensity: 250_000.0,
                range: 18.0,
                shadows_enabled: false,
                ..default()
            },
            TorchFlicker {
                base_intensity: 250_000.0,
                flicker_amount: 40_000.0,
                speed: 4.0,
                phase: i as f32 * 1.5,
            },
            Transform::from_xyz(0.0, -0.1, 0.0),
        )).id();

        commands.entity(torch_parent).add_children(&[model, fire]);
    }

    // Shadow map 1024 (вместо дефолтных 2048) — экономия памяти
    commands.insert_resource(DirectionalLightShadowMap { size: 1024 });

    info!("Arena setup complete: 50x50m with walls, torches, lighting");
}
