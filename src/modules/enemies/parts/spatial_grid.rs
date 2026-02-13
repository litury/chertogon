use bevy::prelude::*;
use std::collections::HashMap;
use crate::modules::enemies::components::*;

/// Spatial hash grid для O(k) поиска соседних врагов.
/// Ячейка 3×3м — арена 50×50м → сетка ~17×17.
#[derive(Resource)]
pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<(Entity, Vec3)>>,
}

impl Default for SpatialGrid {
    fn default() -> Self {
        Self {
            cell_size: 3.0,
            cells: HashMap::new(),
        }
    }
}

impl SpatialGrid {
    fn cell_key(&self, pos: Vec3) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.z / self.cell_size).floor() as i32,
        )
    }

    /// Вызывает callback для каждого соседа в радиусе (0 аллокаций)
    pub fn for_each_in_radius(&self, pos: Vec3, radius: f32, mut f: impl FnMut(Entity, Vec3)) {
        let r_sq = radius * radius;
        let (cx, cz) = self.cell_key(pos);

        for dx in -1..=1 {
            for dz in -1..=1 {
                if let Some(cell) = self.cells.get(&(cx + dx, cz + dz)) {
                    for &(entity, other_pos) in cell {
                        let diff = pos - other_pos;
                        let dist_sq = diff.x * diff.x + diff.z * diff.z;
                        if dist_sq < r_sq && dist_sq > 0.0001 {
                            f(entity, other_pos);
                        }
                    }
                }
            }
        }
    }
}

/// Перестраивает spatial grid каждый кадр (O(n), 0 аллокаций при стабильном кол-ве врагов)
pub fn rebuild_spatial_grid_system(
    mut grid: ResMut<SpatialGrid>,
    enemies: Query<(Entity, &Transform), (With<Enemy>, Without<EnemyDying>)>,
) {
    // Очищаем Vecs без деаллокации (capacity сохраняется)
    for cell in grid.cells.values_mut() {
        cell.clear();
    }
    for (entity, transform) in &enemies {
        let pos = transform.translation;
        let key = grid.cell_key(pos);
        grid.cells.entry(key).or_default().push((entity, pos));
    }
}
