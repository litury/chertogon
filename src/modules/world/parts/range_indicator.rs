use bevy::prelude::*;
use crate::modules::enemies::components::{Enemy, ChasePlayer, EnemyDying};
use crate::modules::player::Player;
use crate::modules::combat::components::Weapon;
use crate::modules::selection::components::Selected;
use super::ground_circle::create_annular_arc;

/// Маркер кольца радиуса (aggro / attack range)
#[derive(Component)]
pub struct RangeIndicator {
    pub base_alpha: f32,
    pub material_handle: Handle<StandardMaterial>,
    /// Последнее значение alpha (избегаем materials.get_mut() каждый кадр)
    pub last_alpha: f32,
}

const RING_THICKNESS: f32 = 0.12;
const RING_Y: f32 = -0.88; // Parent на Y=0.9, итого world Y=0.02 (на полу)
const SEGMENTS: u32 = 48;

/// Спавнит кольцо радиуса при выделении юнита
pub fn spawn_range_indicator(
    mut commands: Commands,
    enemies: Query<
        (Entity, &ChasePlayer),
        (With<Enemy>, Added<Selected>, Without<EnemyDying>),
    >,
    players: Query<(Entity, &Weapon), (With<Player>, Added<Selected>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Враг: aggro range (красный)
    for (entity, chase) in &enemies {
        let r = chase.aggro_range;
        let base_alpha = 0.2;
        let mesh = meshes.add(create_annular_arc(
            r - RING_THICKNESS, r + RING_THICKNESS, 1.0, SEGMENTS,
        ));
        let material_handle = materials.add(StandardMaterial {
            base_color: Color::srgba(0.9, 0.15, 0.1, base_alpha),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            ..default()
        });

        let child = commands.spawn((
            RangeIndicator { base_alpha, material_handle: material_handle.clone(), last_alpha: base_alpha },
            Mesh3d(mesh),
            MeshMaterial3d(material_handle),
            Transform {
                translation: Vec3::new(0.0, RING_Y, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                ..default()
            },
        )).id();

        commands.entity(entity).add_child(child);
    }

    // Игрок: attack range (голубой)
    for (entity, weapon) in &players {
        let r = weapon.range;
        let base_alpha = 0.25;
        let mesh = meshes.add(create_annular_arc(
            r - RING_THICKNESS, r + RING_THICKNESS, 1.0, SEGMENTS,
        ));
        let material_handle = materials.add(StandardMaterial {
            base_color: Color::srgba(0.2, 0.6, 0.95, base_alpha),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            ..default()
        });

        let child = commands.spawn((
            RangeIndicator { base_alpha, material_handle: material_handle.clone(), last_alpha: base_alpha },
            Mesh3d(mesh),
            MeshMaterial3d(material_handle),
            Transform {
                translation: Vec3::new(0.0, RING_Y, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                ..default()
            },
        )).id();

        commands.entity(entity).add_child(child);
    }
}

/// Удаляет кольцо при снятии выделения
pub fn despawn_range_indicator(
    mut commands: Commands,
    mut removed: RemovedComponents<Selected>,
    children_query: Query<&Children>,
    indicator_query: Query<Entity, With<RangeIndicator>>,
) {
    for entity in removed.read() {
        let Ok(children) = children_query.get(entity) else { continue };
        for child in children.iter() {
            if indicator_query.get(child).is_ok() {
                commands.entity(child).insert(Visibility::Hidden);
            }
        }
    }
}

/// Лёгкий пульс alpha на кольце (обновляет material только при заметном изменении)
pub fn pulse_range_indicator(
    time: Res<Time>,
    mut indicators: Query<&mut RangeIndicator>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let t = time.elapsed_secs();
    let pulse = 0.85 + 0.15 * (t * 2.0).sin();

    for mut indicator in &mut indicators {
        let alpha = indicator.base_alpha * pulse;
        if (alpha - indicator.last_alpha).abs() > 0.02 {
            indicator.last_alpha = alpha;
            if let Some(mat) = materials.get_mut(&indicator.material_handle) {
                mat.base_color = mat.base_color.with_alpha(alpha);
            }
        }
    }
}

/// Удаляет все индикаторы при выходе из Playing
pub fn cleanup_range_indicators(
    mut commands: Commands,
    query: Query<Entity, With<RangeIndicator>>,
) {
    for entity in &query {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}
