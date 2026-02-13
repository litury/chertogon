use bevy::prelude::*;
use crate::shared::constants::{PORTAL_1_POS, PORTAL_2_POS, PORTAL_BASE_RADIUS};
use crate::modules::enemies::components::{SpawnPortal, PortalVortex, PortalLight};

/// Спавнит два визуально разных портала на дальней стене арены (верх экрана)
pub fn setup_portals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Портал 1 — "Разлом Огня": вертикальная трещина, фиолетово-красный
    spawn_portal(
        &mut commands,
        &mut meshes,
        &mut materials,
        0,
        PORTAL_1_POS,
        PortalStyle {
            vortex_mesh: PortalShape::Ellipse { half_x: 1.2, half_y: 1.8 },
            base_color: Color::srgba(0.55, 0.0, 1.0, 0.7),
            emissive: LinearRgba::new(3.0, 0.3, 5.0, 1.0),
            light_color: Color::srgb(0.6, 0.0, 0.8),
        },
    );

    // Портал 2 — "Разлом Тьмы": круглая воронка, зелёно-фиолетовый
    spawn_portal(
        &mut commands,
        &mut meshes,
        &mut materials,
        1,
        PORTAL_2_POS,
        PortalStyle {
            vortex_mesh: PortalShape::Circle { radius: PORTAL_BASE_RADIUS },
            base_color: Color::srgba(0.0, 0.6, 0.3, 0.7),
            emissive: LinearRgba::new(0.3, 3.0, 1.5, 1.0),
            light_color: Color::srgb(0.0, 0.6, 0.4),
        },
    );

    info!("🌀 Порталы Нави установлены (2 разлома на дальней стене)");
}

enum PortalShape {
    Ellipse { half_x: f32, half_y: f32 },
    Circle { radius: f32 },
}

struct PortalStyle {
    vortex_mesh: PortalShape,
    base_color: Color,
    emissive: LinearRgba,
    light_color: Color,
}

fn spawn_portal(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    index: u8,
    position: Vec3,
    style: PortalStyle,
) {
    // Воронка — плоский меш повёрнутый лицом в арену (+Z направление)
    let vortex_mesh_handle = match style.vortex_mesh {
        PortalShape::Ellipse { half_x, half_y } => {
            meshes.add(Ellipse::new(half_x, half_y))
        }
        PortalShape::Circle { radius } => {
            meshes.add(Circle::new(radius))
        }
    };

    let vortex_material = materials.add(StandardMaterial {
        base_color: style.base_color,
        emissive: style.emissive,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    // Root entity портала
    let portal_root = commands.spawn((
        SpawnPortal { index },
        Transform::from_translation(position),
    )).id();

    // Воронка — повёрнута лицом внутрь арены (нормаль меша по умолч. +Y, крутим на +Z)
    let vortex = commands.spawn((
        Mesh3d(vortex_mesh_handle),
        MeshMaterial3d(vortex_material),
        Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        PortalVortex,
    )).id();

    // Точечный свет портала
    let light = commands.spawn((
        PointLight {
            color: style.light_color,
            intensity: 150_000.0,
            range: 12.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.5), // Немного вглубь арены (+Z)
        PortalLight,
    )).id();

    commands.entity(portal_root).add_children(&[vortex, light]);
}
