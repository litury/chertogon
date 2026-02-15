use bevy::prelude::*;
use crate::shared::constants::{PORTAL_1_POS, PORTAL_2_POS};
use crate::modules::enemies::components::{SpawnPortal, PortalVortex, PortalLight};
use crate::toolkit::asset_paths;

/// Спавнит два визуально разных портала на дальней стене арены (верх экрана)
pub fn setup_portals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Портал 1 — "Разлом Огня": каменная арка с рунами
    spawn_portal(
        &mut commands,
        &asset_server,
        0,
        PORTAL_1_POS,
        asset_paths::PORTAL_FIRE,
        Color::srgb(0.6, 0.0, 0.8), // Фиолетово-красный свет
    );

    // Портал 2 — "Разлом Тьмы": арка из корней и костей
    spawn_portal(
        &mut commands,
        &asset_server,
        1,
        PORTAL_2_POS,
        asset_paths::PORTAL_DARK,
        Color::srgb(0.0, 0.6, 0.4), // Зелёно-фиолетовый свет
    );

    info!("🌀 Порталы Нави установлены (2 разлома на дальней стене)");
}

fn spawn_portal(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    index: u8,
    position: Vec3,
    model_path: &'static str,
    light_color: Color,
) {
    // Root entity портала
    let portal_root = commands.spawn((
        SpawnPortal { index },
        Transform::from_translation(position),
    )).id();

    // 3D модель портала (Meshy GLB) — scale 4x (~8м арка, вписывается в арену)
    let model = commands.spawn((
        SceneRoot(asset_server.load(model_path)),
        Transform::from_scale(Vec3::splat(4.0)),
        PortalVortex,
    )).id();

    // Точечный свет портала
    let light = commands.spawn((
        PointLight {
            color: light_color,
            intensity: 200_000.0,
            range: 15.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 1.0, 2.0),
        PortalLight,
    )).id();

    commands.entity(portal_root).add_children(&[model, light]);
}
