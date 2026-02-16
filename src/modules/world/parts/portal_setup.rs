use bevy::prelude::*;
use crate::shared::constants::{PORTAL_1_POS, PORTAL_2_POS};
use crate::modules::enemies::components::{SpawnPortal, PortalVortex, PortalLight};
use crate::toolkit::asset_paths;
use super::portal_fill::{PortalVortexMaterial, PortalVortexSettings};

/// Спавнит два визуально разных портала на дальней стене арены (верх экрана)
pub fn setup_portals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut vortex_materials: ResMut<Assets<PortalVortexMaterial>>,
) {
    // Вихрь портала 1 — огненно-фиолетовый
    let fire_vortex = vortex_materials.add(PortalVortexMaterial {
        settings: PortalVortexSettings {
            color: LinearRgba::new(1.5, 0.3, 2.0, 1.0), // HDR фиолетово-красный
            speed: 0.8,
            twist: 6.0,
            _pad1: 0.0,
            _pad2: 0.0,
        },
    });

    // Вихрь портала 2 — зелёно-болотный
    let dark_vortex = vortex_materials.add(PortalVortexMaterial {
        settings: PortalVortexSettings {
            color: LinearRgba::new(0.2, 2.0, 0.8, 1.0), // HDR зелёно-фиолетовый
            speed: 0.6,
            twist: -5.0, // Обратное вращение
            _pad1: 0.0,
            _pad2: 0.0,
        },
    });

    // Портал 1 — "Разлом Огня": каменная арка с рунами
    // Blender analysis: opening center Y=-1.16 (×4), radius=1.4 (half-width 2.8)
    spawn_portal(
        &mut commands,
        &asset_server,
        &mut meshes,
        0,
        PORTAL_1_POS,
        asset_paths::PORTAL_FIRE,
        Color::srgb(0.6, 0.0, 0.8),
        fire_vortex,
        Vec3::new(0.0, -1.16, 0.0),
        2.2,
    );

    // Портал 2 — "Разлом Тьмы": арка из корней и костей
    // Blender analysis: opening center Y=+0.47 (×4), radius=1.75 (half-height 3.49)
    spawn_portal(
        &mut commands,
        &asset_server,
        &mut meshes,
        1,
        PORTAL_2_POS,
        asset_paths::PORTAL_DARK,
        Color::srgb(0.0, 0.6, 0.4),
        dark_vortex,
        Vec3::new(0.0, -0.5, 0.0),
        1.7,
    );

    info!("🌀 Порталы Нави установлены (2 разлома с вихрями)");
}

fn spawn_portal(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    index: u8,
    position: Vec3,
    model_path: &'static str,
    light_color: Color,
    vortex_material: Handle<PortalVortexMaterial>,
    fill_offset: Vec3,
    fill_radius: f32,
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

    // Вихревое заполнение — плоский круг внутри арки с кастомным шейдером
    // Позиция и радиус определены через Blender MCP vertex density analysis
    let fill_mesh = meshes.add(Circle::new(fill_radius));
    let fill = commands.spawn((
        Mesh3d(fill_mesh),
        MeshMaterial3d(vortex_material),
        Transform::from_translation(fill_offset),
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

    commands.entity(portal_root).add_children(&[model, fill, light]);
}
