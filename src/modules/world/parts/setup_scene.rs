use bevy::prelude::*;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::mesh::VertexAttributeValues;
use avian3d::prelude::*;
use bevy::light::DirectionalLightShadowMap;
use crate::toolkit::asset_paths;


pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // === Global Ambient Light (слабый заполняющий — для видимых теней) ===
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.35, 0.38, 0.42),
        brightness: 300.0,
        ..default()
    });

    // === DirectionalLight (солнце — сильный для глубоких теней) ===
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.95, 0.9, 0.75),
            illuminance: 12000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 25.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Shadow map 2048 (чёткие тени на арене 50×50м)
    commands.insert_resource(DirectionalLightShadowMap { size: 2048 });

    // === Пол: зелёная трава + грязь (Polyhaven forrest_ground_01) ===
    // Меш 150×150м — трава уходит далеко за скалы, камера не видит чёрных краёв при max zoom
    let tile_repeat = 24.0;
    let mut floor_mesh: Mesh = Plane3d::default().mesh().size(150.0, 150.0).into();

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
            base_color: Color::srgb(0.85, 1.0, 0.8),
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

    // === СКАЛЬНЫЕ СТЕНЫ (3 варианта GLB, плотная стена по периметру с коллайдерами) ===
    spawn_cliff_boundaries(&mut commands, &asset_server);

    // === РУННЫЕ КАМНИ (свет + коллайдеры) ===
    spawn_rune_stones(&mut commands, &asset_server);

    // === ДЕКОР (камни, деревья, кости — с коллайдерами) ===
    spawn_arena_props(&mut commands, &asset_server);

    info!("Forest arena setup complete: 50x50m with cliffs, rune stones, props");
}

/// Плотная скальная стена по периметру арены.
/// Двухслойная: 8 основных + 7 промежуточных секций на стену = 60 всего.
/// Промежуточные сдвинуты вглубь и увеличены — полностью закрывают щели.
fn spawn_cliff_boundaries(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let cliff_scenes = [
        asset_server.load(asset_paths::CLIFF_WALL_A),
        asset_server.load(asset_paths::CLIFF_WALL_B),
        asset_server.load(asset_paths::CLIFF_WALL_C),
    ];

    let cliff_height = 6.0;
    let collider_half_w = 4.5;
    let collider_half_d = 2.0;

    // (position, rotation_y, scale, variant_index)
    let sections: &[(Vec3, f32, Vec3, usize)] = &[
        // ============ СЕВЕРНАЯ СТЕНА (z ≈ -25.5) ============
        // Основные 8 секций
        (Vec3::new(-24.5, 0.0, -25.5), 0.0, Vec3::new(3.0, 2.8, 2.5), 0),
        (Vec3::new(-17.5, 0.0, -25.8), 0.0, Vec3::new(2.8, 2.5, 2.5), 1),
        (Vec3::new(-10.5, 0.0, -25.3), 0.0, Vec3::new(3.0, 2.6, 2.5), 2),
        (Vec3::new(-3.5, 0.0, -25.6), 0.0, Vec3::new(2.8, 2.8, 2.5), 0),
        (Vec3::new(3.5, 0.0, -25.4), 0.0, Vec3::new(3.0, 2.5, 2.5), 1),
        (Vec3::new(10.5, 0.0, -25.7), 0.0, Vec3::new(2.8, 2.7, 2.5), 2),
        (Vec3::new(17.5, 0.0, -25.5), 0.0, Vec3::new(3.0, 2.6, 2.5), 0),
        (Vec3::new(24.5, 0.0, -25.8), 0.0, Vec3::new(2.8, 2.8, 2.5), 1),
        // Промежуточные 7 секций (заполняют щели, сдвинуты вглубь)
        (Vec3::new(-21.0, 0.0, -26.3), 0.0, Vec3::new(3.3, 2.6, 2.8), 2),
        (Vec3::new(-14.0, 0.0, -26.1), 0.0, Vec3::new(3.5, 2.7, 2.8), 0),
        (Vec3::new(-7.0,  0.0, -26.4), 0.0, Vec3::new(3.2, 2.5, 2.8), 1),
        (Vec3::new(0.0,   0.0, -26.2), 0.0, Vec3::new(3.4, 2.8, 2.8), 2),
        (Vec3::new(7.0,   0.0, -26.3), 0.0, Vec3::new(3.3, 2.6, 2.8), 0),
        (Vec3::new(14.0,  0.0, -26.1), 0.0, Vec3::new(3.5, 2.7, 2.8), 1),
        (Vec3::new(21.0,  0.0, -26.4), 0.0, Vec3::new(3.2, 2.5, 2.8), 2),

        // ============ ЮЖНАЯ СТЕНА (z ≈ +25.5) ============
        // Основные 8 секций
        (Vec3::new(-24.5, 0.0, 25.5), std::f32::consts::PI, Vec3::new(3.0, 2.8, 2.5), 2),
        (Vec3::new(-17.5, 0.0, 25.3), std::f32::consts::PI, Vec3::new(2.8, 2.5, 2.5), 0),
        (Vec3::new(-10.5, 0.0, 25.6), std::f32::consts::PI, Vec3::new(3.0, 2.7, 2.5), 1),
        (Vec3::new(-3.5, 0.0, 25.4), std::f32::consts::PI, Vec3::new(2.8, 2.6, 2.5), 2),
        (Vec3::new(3.5, 0.0, 25.7), std::f32::consts::PI, Vec3::new(3.0, 2.8, 2.5), 0),
        (Vec3::new(10.5, 0.0, 25.5), std::f32::consts::PI, Vec3::new(2.8, 2.5, 2.5), 1),
        (Vec3::new(17.5, 0.0, 25.8), std::f32::consts::PI, Vec3::new(3.0, 2.6, 2.5), 2),
        (Vec3::new(24.5, 0.0, 25.4), std::f32::consts::PI, Vec3::new(2.8, 2.7, 2.5), 0),
        // Промежуточные 7 секций
        (Vec3::new(-21.0, 0.0, 26.3), std::f32::consts::PI, Vec3::new(3.3, 2.6, 2.8), 1),
        (Vec3::new(-14.0, 0.0, 26.1), std::f32::consts::PI, Vec3::new(3.5, 2.7, 2.8), 2),
        (Vec3::new(-7.0,  0.0, 26.4), std::f32::consts::PI, Vec3::new(3.2, 2.5, 2.8), 0),
        (Vec3::new(0.0,   0.0, 26.2), std::f32::consts::PI, Vec3::new(3.4, 2.8, 2.8), 1),
        (Vec3::new(7.0,   0.0, 26.3), std::f32::consts::PI, Vec3::new(3.3, 2.6, 2.8), 2),
        (Vec3::new(14.0,  0.0, 26.1), std::f32::consts::PI, Vec3::new(3.5, 2.7, 2.8), 0),
        (Vec3::new(21.0,  0.0, 26.4), std::f32::consts::PI, Vec3::new(3.2, 2.5, 2.8), 1),

        // ============ ЗАПАДНАЯ СТЕНА (x ≈ -25.5) ============
        // Основные 8 секций
        (Vec3::new(-25.5, 0.0, -24.5), std::f32::consts::FRAC_PI_2, Vec3::new(3.0, 2.8, 2.5), 1),
        (Vec3::new(-25.3, 0.0, -17.5), std::f32::consts::FRAC_PI_2, Vec3::new(2.8, 2.5, 2.5), 2),
        (Vec3::new(-25.6, 0.0, -10.5), std::f32::consts::FRAC_PI_2, Vec3::new(3.0, 2.7, 2.5), 0),
        (Vec3::new(-25.4, 0.0, -3.5), std::f32::consts::FRAC_PI_2, Vec3::new(2.8, 2.6, 2.5), 1),
        (Vec3::new(-25.7, 0.0, 3.5), std::f32::consts::FRAC_PI_2, Vec3::new(3.0, 2.8, 2.5), 2),
        (Vec3::new(-25.5, 0.0, 10.5), std::f32::consts::FRAC_PI_2, Vec3::new(2.8, 2.5, 2.5), 0),
        (Vec3::new(-25.8, 0.0, 17.5), std::f32::consts::FRAC_PI_2, Vec3::new(3.0, 2.6, 2.5), 1),
        (Vec3::new(-25.4, 0.0, 24.5), std::f32::consts::FRAC_PI_2, Vec3::new(2.8, 2.7, 2.5), 2),
        // Промежуточные 7 секций
        (Vec3::new(-26.3, 0.0, -21.0), std::f32::consts::FRAC_PI_2, Vec3::new(3.3, 2.6, 2.8), 0),
        (Vec3::new(-26.1, 0.0, -14.0), std::f32::consts::FRAC_PI_2, Vec3::new(3.5, 2.7, 2.8), 1),
        (Vec3::new(-26.4, 0.0, -7.0),  std::f32::consts::FRAC_PI_2, Vec3::new(3.2, 2.5, 2.8), 2),
        (Vec3::new(-26.2, 0.0, 0.0),   std::f32::consts::FRAC_PI_2, Vec3::new(3.4, 2.8, 2.8), 0),
        (Vec3::new(-26.3, 0.0, 7.0),   std::f32::consts::FRAC_PI_2, Vec3::new(3.3, 2.6, 2.8), 1),
        (Vec3::new(-26.1, 0.0, 14.0),  std::f32::consts::FRAC_PI_2, Vec3::new(3.5, 2.7, 2.8), 2),
        (Vec3::new(-26.4, 0.0, 21.0),  std::f32::consts::FRAC_PI_2, Vec3::new(3.2, 2.5, 2.8), 0),

        // ============ ВОСТОЧНАЯ СТЕНА (x ≈ +25.5) ============
        // Основные 8 секций
        (Vec3::new(25.5, 0.0, -24.5), -std::f32::consts::FRAC_PI_2, Vec3::new(3.0, 2.8, 2.5), 2),
        (Vec3::new(25.8, 0.0, -17.5), -std::f32::consts::FRAC_PI_2, Vec3::new(2.8, 2.5, 2.5), 0),
        (Vec3::new(25.3, 0.0, -10.5), -std::f32::consts::FRAC_PI_2, Vec3::new(3.0, 2.7, 2.5), 1),
        (Vec3::new(25.6, 0.0, -3.5), -std::f32::consts::FRAC_PI_2, Vec3::new(2.8, 2.6, 2.5), 2),
        (Vec3::new(25.4, 0.0, 3.5), -std::f32::consts::FRAC_PI_2, Vec3::new(3.0, 2.8, 2.5), 0),
        (Vec3::new(25.7, 0.0, 10.5), -std::f32::consts::FRAC_PI_2, Vec3::new(2.8, 2.5, 2.5), 1),
        (Vec3::new(25.5, 0.0, 17.5), -std::f32::consts::FRAC_PI_2, Vec3::new(3.0, 2.6, 2.5), 2),
        (Vec3::new(25.8, 0.0, 24.5), -std::f32::consts::FRAC_PI_2, Vec3::new(2.8, 2.7, 2.5), 0),
        // Промежуточные 7 секций
        (Vec3::new(26.3, 0.0, -21.0), -std::f32::consts::FRAC_PI_2, Vec3::new(3.3, 2.6, 2.8), 1),
        (Vec3::new(26.1, 0.0, -14.0), -std::f32::consts::FRAC_PI_2, Vec3::new(3.5, 2.7, 2.8), 2),
        (Vec3::new(26.4, 0.0, -7.0),  -std::f32::consts::FRAC_PI_2, Vec3::new(3.2, 2.5, 2.8), 0),
        (Vec3::new(26.2, 0.0, 0.0),   -std::f32::consts::FRAC_PI_2, Vec3::new(3.4, 2.8, 2.8), 1),
        (Vec3::new(26.3, 0.0, 7.0),   -std::f32::consts::FRAC_PI_2, Vec3::new(3.3, 2.6, 2.8), 2),
        (Vec3::new(26.1, 0.0, 14.0),  -std::f32::consts::FRAC_PI_2, Vec3::new(3.5, 2.7, 2.8), 0),
        (Vec3::new(26.4, 0.0, 21.0),  -std::f32::consts::FRAC_PI_2, Vec3::new(3.2, 2.5, 2.8), 1),
    ];

    for &(pos, rot_y, scale, variant) in sections {
        commands.spawn((
            SceneRoot(cliff_scenes[variant].clone()),
            Transform::from_translation(pos)
                .with_scale(scale)
                .with_rotation(Quat::from_rotation_y(rot_y)),
            RigidBody::Static,
            Collider::cuboid(collider_half_w, cliff_height / 2.0, collider_half_d),
            crate::shared::GameLayer::static_layers(),
        ));
    }
}

/// Рунные камни — замена факелов. Стоячие камни с рунами дают холодный свет.
fn spawn_rune_stones(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let rune_scene = asset_server.load(asset_paths::RUNE_STONE);

    // (позиция, угол поворота Y)
    let stones: &[(Vec3, f32)] = &[
        (Vec3::new(-15.0, 0.0, -15.0), 0.0),
        (Vec3::new(15.0, 0.0, -15.0), 1.2),
        (Vec3::new(-15.0, 0.0, 15.0), 2.4),
        (Vec3::new(15.0, 0.0, 15.0), 3.6),
    ];

    for &(pos, rot_y) in stones {
        let parent = commands.spawn((
            Transform::from_translation(pos),
            RigidBody::Static,
            Collider::cylinder(0.6, 1.5),
            crate::shared::GameLayer::static_layers(),
        )).id();

        let model = commands.spawn((
            SceneRoot(rune_scene.clone()),
            Transform::from_scale(Vec3::splat(1.5))
                .with_rotation(Quat::from_rotation_y(rot_y)),
        )).id();

        // Холодный голубой свет рун
        let light = commands.spawn((
            PointLight {
                color: Color::srgb(0.4, 0.8, 1.0),
                intensity: 150_000.0,
                range: 12.0,
                shadows_enabled: false,
                ..default()
            },
            Transform::from_xyz(0.0, 2.5, 0.0),
        )).id();

        commands.entity(parent).add_children(&[model, light]);
    }
}

/// Декоративные элементы: валуны, мёртвые деревья, кости (с коллайдерами)
fn spawn_arena_props(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let rock_scene = asset_server.load(asset_paths::ROCK_LARGE);
    let tree_scene = asset_server.load(asset_paths::DEAD_TREE);
    let bones_scene = asset_server.load(asset_paths::BONE_PILE);

    // Валуны внутри арены (pos, rot_y, scale) — с коллайдерами
    let rocks: &[(Vec3, f32, f32)] = &[
        (Vec3::new(-22.0, 0.0, -18.0), 0.3, 1.8),
        (Vec3::new(21.0, 0.0, 12.0), 1.5, 2.0),
        (Vec3::new(-18.0, 0.0, 20.0), 2.8, 1.5),
        (Vec3::new(16.0, 0.0, -20.0), 4.0, 1.7),
    ];
    for &(pos, rot, scale) in rocks {
        commands.spawn((
            SceneRoot(rock_scene.clone()),
            Transform::from_translation(pos)
                .with_scale(Vec3::splat(scale))
                .with_rotation(Quat::from_rotation_y(rot)),
            RigidBody::Static,
            Collider::cylinder(1.0 * scale, 1.0),
            crate::shared::GameLayer::static_layers(),
        ));
    }

    // Камни у основания стен — маскируют стыки скальных секций (без коллайдеров, под стеной)
    let wall_rocks: &[(Vec3, f32, f32)] = &[
        // Северная стена
        (Vec3::new(-21.0, 0.0, -24.5), 0.5, 1.3),
        (Vec3::new(-7.0,  0.0, -24.8), 2.1, 1.1),
        (Vec3::new(14.0,  0.0, -24.3), 3.8, 1.4),
        // Южная стена
        (Vec3::new(-14.0, 0.0, 24.6), 1.2, 1.2),
        (Vec3::new(7.0,   0.0, 24.3), 4.5, 1.3),
        (Vec3::new(21.0,  0.0, 24.8), 0.8, 1.1),
        // Западная стена
        (Vec3::new(-24.5, 0.0, -14.0), 2.3, 1.3),
        (Vec3::new(-24.8, 0.0, 7.0),  0.6, 1.2),
        (Vec3::new(-24.3, 0.0, 21.0), 3.5, 1.1),
        // Восточная стена
        (Vec3::new(24.6, 0.0, -7.0),  1.8, 1.4),
        (Vec3::new(24.3, 0.0, 14.0), 4.2, 1.2),
        (Vec3::new(24.8, 0.0, -21.0), 0.3, 1.3),
    ];
    for &(pos, rot, scale) in wall_rocks {
        commands.spawn((
            SceneRoot(rock_scene.clone()),
            Transform::from_translation(pos)
                .with_scale(Vec3::splat(scale))
                .with_rotation(Quat::from_rotation_y(rot)),
        ));
    }

    // Мёртвые деревья внутри арены — с коллайдерами (ствол)
    let trees: &[(Vec3, f32, f32)] = &[
        (Vec3::new(-20.0, 0.0, 8.0), 0.5, 2.0),
        (Vec3::new(22.0, 0.0, -8.0), 2.0, 1.8),
        (Vec3::new(5.0, 0.0, 22.0), 3.5, 2.2),
    ];
    for &(pos, rot, scale) in trees {
        commands.spawn((
            SceneRoot(tree_scene.clone()),
            Transform::from_translation(pos)
                .with_scale(Vec3::splat(scale))
                .with_rotation(Quat::from_rotation_y(rot)),
            RigidBody::Static,
            Collider::cylinder(0.5, 2.0),
            crate::shared::GameLayer::static_layers(),
        ));
    }

    // Деревья вдоль стен — маскируют верхнюю часть стыков (без коллайдеров, декор на фоне)
    let wall_trees: &[(Vec3, f32, f32)] = &[
        // Северная стена
        (Vec3::new(-10.0, 0.0, -24.0), 1.3, 2.5),
        (Vec3::new(10.0,  0.0, -24.5), 3.7, 2.2),
        // Южная стена
        (Vec3::new(-5.0,  0.0, 24.2), 0.9, 2.3),
        (Vec3::new(18.0,  0.0, 24.7), 2.6, 2.0),
        // Западная стена
        (Vec3::new(-24.2, 0.0, -5.0), 4.1, 2.4),
        (Vec3::new(-24.6, 0.0, 18.0), 1.8, 2.1),
        // Восточная стена
        (Vec3::new(24.5, 0.0, 5.0), 0.4, 2.3),
        (Vec3::new(24.2, 0.0, -18.0), 3.0, 2.5),
    ];
    for &(pos, rot, scale) in wall_trees {
        commands.spawn((
            SceneRoot(tree_scene.clone()),
            Transform::from_translation(pos)
                .with_scale(Vec3::splat(scale))
                .with_rotation(Quat::from_rotation_y(rot)),
        ));
    }

    // Кости на земле (по арене) — без коллайдеров (плоские, можно пройти)
    let bones: &[(Vec3, f32, f32)] = &[
        (Vec3::new(-8.0, 0.0, -10.0), 1.0, 1.2),
        (Vec3::new(10.0, 0.0, 5.0), 2.5, 1.0),
        (Vec3::new(-5.0, 0.0, 12.0), 4.0, 1.3),
        (Vec3::new(12.0, 0.0, -15.0), 0.7, 1.1),
    ];
    for &(pos, rot, scale) in bones {
        commands.spawn((
            SceneRoot(bones_scene.clone()),
            Transform::from_translation(pos)
                .with_scale(Vec3::splat(scale))
                .with_rotation(Quat::from_rotation_y(rot)),
        ));
    }
}
