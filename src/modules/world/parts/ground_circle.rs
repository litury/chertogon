use bevy::prelude::*;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;
use crate::modules::enemies::components::{Enemy, Health, EnemyDying, EnemyModel};
use crate::modules::player::components::{Player, PlayerModel};
use crate::modules::combat::components::{PlayerHealth, AttackCooldown};

/// Ground ring — HP-бар в виде дуги + индикатор направления.
/// Дуга сжимается с потерей HP (разрыв сзади = куда бить).
/// Поворачивается синхронно с моделью персонажа.
#[derive(Component)]
pub struct GroundCircle {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub base_alpha: f32,
    pub pulse_speed: f32,
    pub material_handle: Handle<StandardMaterial>,
    /// Последняя доля HP для которой был построен меш (избегаем пересборки каждый кадр)
    pub last_hp_fraction: f32,
    /// Угол направления (радианы, из rotation модели)
    pub last_facing: f32,
    /// Последнее значение alpha (избегаем get_mut каждый кадр)
    pub last_alpha: f32,
}

/// Кольцо перезарядки оружия (тонкая дуга внутри HP ring)
#[derive(Component)]
pub struct CooldownRing {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub material_handle: Handle<StandardMaterial>,
    /// Последняя доля кулдауна (0..1)
    pub last_fraction: f32,
    /// Направление (синхронизируется с PlayerModel)
    pub last_facing: f32,
}

/// HP-дуга + направление: обновляет меш и вращение кольца
pub fn health_ring_system(
    time: Res<Time>,
    enemies: Query<(&Health, &Children), (With<Enemy>, Without<EnemyDying>)>,
    player: Query<(&PlayerHealth, &Children), With<Player>>,
    mut circle_query: Query<(&mut GroundCircle, &mut Transform, &Mesh3d)>,
    model_query: Query<&Transform, (Or<(With<PlayerModel>, With<EnemyModel>)>, Without<GroundCircle>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let t = time.elapsed_secs();

    // Враги
    for (health, children) in &enemies {
        let hp_pct = health.current / health.max;
        update_ring(&mut circle_query, &mut meshes, &mut materials, children, hp_pct, &model_query, t);
    }

    // Игрок
    for (health, children) in &player {
        let hp_pct = health.current / health.max;
        update_ring(&mut circle_query, &mut meshes, &mut materials, children, hp_pct, &model_query, t);
    }
}

fn update_ring(
    circle_query: &mut Query<(&mut GroundCircle, &mut Transform, &Mesh3d)>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    children: &Children,
    hp_pct: f32,
    model_query: &Query<&Transform, (Or<(With<PlayerModel>, With<EnemyModel>)>, Without<GroundCircle>)>,
    t: f32,
) {
    // Направление из rotation модели-сиблинга (PlayerModel / EnemyModel)
    let mut facing = None;
    for child in children.iter() {
        if let Ok(model_transform) = model_query.get(child) {
            let forward = model_transform.rotation * Vec3::Z;
            facing = Some(forward.x.atan2(forward.z));
            break;
        }
    }

    for child in children.iter() {
        let Ok((mut circle, mut transform, mesh3d)) = circle_query.get_mut(child) else {
            continue;
        };

        // Синхронизируем направление кольца с моделью
        if let Some(angle) = facing {
            circle.last_facing = angle;
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

        // Лёгкий пульс alpha — обновляем материал ТОЛЬКО при заметном изменении
        let pulse = 0.9 + 0.1 * (t * circle.pulse_speed).sin();
        let new_alpha = circle.base_alpha * pulse;
        if (new_alpha - circle.last_alpha).abs() > 0.02 {
            circle.last_alpha = new_alpha;
            if let Some(mat) = materials.get_mut(&circle.material_handle) {
                mat.base_color = mat.base_color.with_alpha(new_alpha);
            }
        }
    }
}

/// Обновляет кольцо перезарядки оружия игрока
pub fn cooldown_ring_system(
    player: Query<(&AttackCooldown, &Children), With<Player>>,
    mut ring_query: Query<(&mut CooldownRing, &mut Transform, &Mesh3d), Without<GroundCircle>>,
    model_query: Query<&Transform, (With<PlayerModel>, Without<CooldownRing>, Without<GroundCircle>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    let Ok((cooldown, children)) = player.single() else { return };

    // Направление из PlayerModel
    let mut facing = 0.0_f32;
    for child in children.iter() {
        if let Ok(model_tf) = model_query.get(child) {
            let forward = model_tf.rotation * Vec3::Z;
            facing = forward.x.atan2(forward.z);
            break;
        }
    }

    let cd_fraction = cooldown.timer.fraction();

    for child in children.iter() {
        let Ok((mut ring, mut transform, mesh3d)) = ring_query.get_mut(child) else {
            continue;
        };

        ring.last_facing = facing;

        // Обновляем меш при заметном изменении fraction
        if (ring.last_fraction - cd_fraction).abs() > 0.01 {
            ring.last_fraction = cd_fraction;
            // Полное кольцо = 95% окружности (как HP ring, gap сзади)
            let arc_fraction = cd_fraction * 0.90 + 0.05;
            if let Some(mesh) = meshes.get_mut(&mesh3d.0) {
                *mesh = create_annular_arc(ring.inner_radius, ring.outer_radius, arc_fraction, 24);
            }
        }

        // Вращение синхронно с HP ring
        transform.rotation = Quat::from_rotation_y(ring.last_facing)
            * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);

        // Яркость: ярче когда готов, тусклее на кулдауне
        let t = time.elapsed_secs();
        let ready = cooldown.timer.is_finished();
        let alpha = if ready {
            0.7 + 0.15 * (t * 4.0).sin() // Пульс когда готов
        } else {
            0.35 + cd_fraction * 0.25 // Растёт с прогрессом кулдауна
        };
        if let Some(mat) = materials.get_mut(&ring.material_handle) {
            mat.base_color = mat.base_color.with_alpha(alpha);
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
