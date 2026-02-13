use bevy::prelude::*;
use avian3d::prelude::*;
use crate::modules::enemies::components::*;
use crate::modules::enemies::parts::spatial_grid::SpatialGrid;
use crate::modules::combat::parts::knockback::{Staggered, StaggerRecovery};

const SEPARATION_RADIUS: f32 = 2.5;
const SEPARATION_FORCE: f32 = 4.0;

/// Boid-like separation: враги отталкиваются от соседей (O(n×k) через SpatialGrid, 0 аллокаций)
pub fn enemy_separation_system(
    grid: Res<SpatialGrid>,
    mut enemies: Query<
        (Entity, &Transform, &mut LinearVelocity),
        (With<Enemy>, Without<EnemyDying>, Without<Staggered>, Without<StaggerRecovery>)
    >,
) {
    // Позиция читается напрямую из Transform — без промежуточного HashMap
    for (entity, transform, mut velocity) in &mut enemies {
        let pos = transform.translation;
        let mut repulsion = Vec3::ZERO;

        // Callback: 0 аллокаций (без возврата Vec)
        grid.for_each_in_radius(pos, SEPARATION_RADIUS, |other_entity, other_pos| {
            if other_entity == entity {
                return;
            }
            let diff = pos - other_pos;
            let dist_sq = diff.x * diff.x + diff.z * diff.z;
            if dist_sq > 0.0001 {
                let dist = dist_sq.sqrt();
                let strength = (1.0 - dist / SEPARATION_RADIUS) * SEPARATION_FORCE;
                repulsion += Vec3::new(diff.x, 0.0, diff.z).normalize_or_zero() * strength;
            }
        });

        velocity.0 += repulsion;
    }
}
