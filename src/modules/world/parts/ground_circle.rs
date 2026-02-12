use bevy::prelude::*;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;
use avian3d::prelude::LinearVelocity;
use crate::modules::enemies::components::{Enemy, Health, EnemyDying};
use crate::modules::player::components::Player;
use crate::modules::combat::components::PlayerHealth;

/// Ground ring — HP-бар в виде дуги + индикатор направления.
/// Дуга сжимается с потерей HP (разрыв сзади = куда бить).
/// Поворачивается в направлении движения.
#[derive(Component)]
pub struct GroundCircle {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub base_alpha: f32,
    pub pulse_speed: f32,
    pub material_handle: Handle<StandardMaterial>,
    /// Последняя доля HP для которой был построен меш (избегаем пересборки каждый кадр)
    pub last_hp_fraction: f32,
    /// Угол направления (радианы, atan2 от velocity)
    pub last_facing: f32,
}

/// HP-дуга + направление: обновляет меш и вращение кольца
pub fn health_ring_system(
    time: Res<Time>,
    enemies: Query<(&Health, &LinearVelocity, &Children), (With<Enemy>, Without<EnemyDying>)>,
    player: Query<(&PlayerHealth, &LinearVelocity, &Children), With<Player>>,
    mut circle_query: Query<(&mut GroundCircle, &mut Transform, &Mesh3d)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let t = time.elapsed_secs();

    // Враги
    for (health, velocity, children) in &enemies {
        let hp_pct = health.current / health.max;
        update_ring(&mut circle_query, &mut meshes, &mut materials, children, hp_pct, velocity, t);
    }

    // Игрок
    for (health, velocity, children) in &player {
        let hp_pct = health.current / health.max;
        update_ring(&mut circle_query, &mut meshes, &mut materials, children, hp_pct, velocity, t);
    }
}

fn update_ring(
    circle_query: &mut Query<(&mut GroundCircle, &mut Transform, &Mesh3d)>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    children: &Children,
    hp_pct: f32,
    velocity: &LinearVelocity,
    t: f32,
) {
    for child in children.iter() {
        let Ok((mut circle, mut transform, mesh3d)) = circle_query.get_mut(child) else {
            continue;
        };

        // Направление из velocity (обновляем только при движении)
        let vel_xz = Vec2::new(velocity.0.x, velocity.0.z);
        if vel_xz.length() > 0.1 {
            circle.last_facing = velocity.0.x.atan2(velocity.0.z);
        }

        // Обновляем меш только когда HP реально изменился
        if (circle.last_hp_fraction - hp_pct).abs() > 0.005 {
            circle.last_hp_fraction = hp_pct;
            // min 5% видимой дуги (чтобы было видно при почти 0 HP), max 95% (всегда есть зазор для направления)
            let fraction = hp_pct * 0.90 + 0.05;
            if let Some(mesh) = meshes.get_mut(&mesh3d.0) {
                *mesh = create_annular_arc(circle.inner_radius, circle.outer_radius, fraction, 32);
            }
        }

        // Вращение: направление движения + укладка на пол
        // Центр дуги (angle=0 в меше) → direction.z через rotation_y
        transform.rotation = Quat::from_rotation_y(circle.last_facing)
            * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);

        // Лёгкий пульс alpha (без тускления цвета!)
        let pulse = 0.9 + 0.1 * (t * circle.pulse_speed).sin();
        if let Some(mat) = materials.get_mut(&circle.material_handle) {
            mat.base_color = mat.base_color.with_alpha(circle.base_alpha * pulse);
        }
    }
}

/// Строит меш кольцевой дуги (annular arc) в XY плоскости.
/// `fraction`: 0.0..1.0 — какая часть полного кольца видна.
/// Дуга центрирована вокруг +Y оси (угол PI/2 от +X).
fn create_annular_arc(inner_r: f32, outer_r: f32, fraction: f32, segments: u32) -> Mesh {
    let half_angle = fraction * std::f32::consts::PI;
    // Центр дуги на +Y (PI/2), чтобы после rotation_y + rotation_x лежала в нужном направлении
    let center_angle = std::f32::consts::FRAC_PI_2;

    let seg = segments.max(3);
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity((seg as usize + 1) * 2);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((seg as usize + 1) * 2);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity((seg as usize + 1) * 2);
    let mut indices: Vec<u32> = Vec::with_capacity(seg as usize * 6);

    for i in 0..=seg {
        let t = i as f32 / seg as f32;
        let angle = center_angle - half_angle + t * 2.0 * half_angle;
        let (sin, cos) = angle.sin_cos();

        // Внутренняя вершина
        positions.push([inner_r * cos, inner_r * sin, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([t, 0.0]);

        // Внешняя вершина
        positions.push([outer_r * cos, outer_r * sin, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([t, 1.0]);
    }

    for i in 0..seg {
        let base = i * 2;
        indices.push(base);
        indices.push(base + 1);
        indices.push(base + 2);

        indices.push(base + 1);
        indices.push(base + 3);
        indices.push(base + 2);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}
