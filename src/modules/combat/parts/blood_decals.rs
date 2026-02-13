use bevy::prelude::*;

/// Максимум декалей крови на сцене (без учёта footprints)
const MAX_BLOOD_DECALS: usize = 30;

/// Цвет крови (для разных типов врагов)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BloodColor {
    Red,
    Green,
    Yellow,
}

/// Маркер пятна крови на полу (остаётся навсегда)
#[derive(Component)]
pub struct BloodDecal {
    pub color: BloodColor,
}

/// Кэшированные ассеты для blood decals — shared mesh + material на цвет.
/// Bevy батчит entities с одинаковым mesh+material → сотни декалей ≈ 1 draw call.
#[derive(Resource)]
pub struct BloodDecalAssets {
    pub mesh: Handle<Mesh>,
    pub red_material: Handle<StandardMaterial>,
    pub green_material: Handle<StandardMaterial>,
    pub yellow_material: Handle<StandardMaterial>,
}

/// Инициализация shared ассетов при старте раунда
pub fn init_blood_decal_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture: Handle<Image> = asset_server.load("textures/vfx/blood_splat.png");

    let mesh = meshes.add(Plane3d::default().mesh().size(1.5, 1.5));

    let red_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture.clone()),
        base_color: Color::srgb(0.6, 0.05, 0.05),
        emissive: LinearRgba::new(0.8, 0.05, 0.02, 1.0),
        alpha_mode: AlphaMode::Mask(0.5),
        unlit: false,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    let green_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture.clone()),
        base_color: Color::srgb(0.05, 0.5, 0.1),
        emissive: LinearRgba::new(0.05, 0.6, 0.1, 1.0),
        alpha_mode: AlphaMode::Mask(0.5),
        unlit: false,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    let yellow_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture),
        base_color: Color::srgb(0.6, 0.55, 0.05),
        emissive: LinearRgba::new(0.7, 0.6, 0.05, 1.0),
        alpha_mode: AlphaMode::Mask(0.5),
        unlit: false,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    commands.insert_resource(BloodDecalAssets {
        mesh,
        red_material,
        green_material,
        yellow_material,
    });
}

/// Спавнит пятно крови на полу (используя shared ассеты).
/// Y=0.005 — ниже ground circles (Y=0.01), чтобы кровь была ПОД кольцами.
pub fn spawn_blood_decal(
    commands: &mut Commands,
    assets: &BloodDecalAssets,
    position: Vec3,
    color: BloodColor,
) {
    // Псевдослучайность от позиции (детерминистично, без rand)
    let seed = position.x * 17.3 + position.z * 31.7;
    let random_angle = seed;
    let random_scale = 0.8 + (seed.sin().abs()) * 1.0; // 0.8 — 1.8

    let material = match color {
        BloodColor::Red => assets.red_material.clone(),
        BloodColor::Green => assets.green_material.clone(),
        BloodColor::Yellow => assets.yellow_material.clone(),
    };

    commands.spawn((
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(material),
        Transform::from_xyz(position.x, 0.005, position.z)
            .with_rotation(Quat::from_rotation_y(random_angle))
            .with_scale(Vec3::splat(random_scale)),
        BloodDecal { color },
    ));
}

/// Спавнит маленький след (для bloody footprints)
pub fn spawn_blood_footprint(
    commands: &mut Commands,
    assets: &BloodDecalAssets,
    position: Vec3,
    color: BloodColor,
    facing_angle: f32,
) {
    let material = match color {
        BloodColor::Red => assets.red_material.clone(),
        BloodColor::Green => assets.green_material.clone(),
        BloodColor::Yellow => assets.yellow_material.clone(),
    };

    commands.spawn((
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(material),
        Transform::from_xyz(position.x, 0.004, position.z)
            .with_rotation(Quat::from_rotation_y(facing_angle))
            .with_scale(Vec3::splat(0.25)),
        BloodDecal { color },
        Footprint {
            timer: Timer::from_seconds(5.0, TimerMode::Once),
        },
    ));
}

/// Маркер следа — исчезает через таймер
#[derive(Component)]
pub struct Footprint {
    pub timer: Timer,
}

/// Ограничивает количество декалей крови (удаляет старейшие при превышении MAX_BLOOD_DECALS)
pub fn blood_decal_limit_system(
    mut commands: Commands,
    decals: Query<Entity, (With<BloodDecal>, Without<Footprint>)>,
    mut buf: Local<Vec<Entity>>,
) {
    // Local<Vec> — capacity переиспользуется между кадрами (0 аллокаций в steady state)
    buf.clear();
    buf.extend(decals.iter());
    if buf.len() > MAX_BLOOD_DECALS {
        for &entity in &buf[..buf.len() - MAX_BLOOD_DECALS] {
            commands.entity(entity).despawn();
        }
    }
}

/// Система: удаляет следы по истечении таймера
pub fn footprint_decay_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Footprint)>,
) {
    for (entity, mut footprint) in &mut query {
        footprint.timer.tick(time.delta());
        if footprint.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
