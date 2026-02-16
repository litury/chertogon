use bevy::prelude::*;
use crate::modules::world::parts::stylized_material::StylizedMaterial;

/// Маркер «враг получил удар» — scale-pop + эмиссивная вспышка
#[derive(Component)]
pub struct HitFlash {
    pub timer: Timer,
    /// Emissive уже выставлен (set-once оптимизация)
    pub emissive_applied: bool,
    /// Кэш entity потомков с материалами (заполняется при первом обходе, переиспользуется при reset)
    pub cached_descendants: Vec<Entity>,
}

impl HitFlash {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.12, TimerMode::Once),
            emissive_applied: false,
            cached_descendants: Vec::new(),
        }
    }
}

/// Система: scale-pop + эмиссивная вспышка на материалах при ударе
/// Emissive выставляется ОДИН РАЗ в начале и сбрасывается ОДИН РАЗ в конце
/// Scale-pop — единственная per-frame операция (дешёвая Transform мутация)
pub fn hit_flash_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut HitFlash, &mut Transform)>,
    children_query: Query<&Children>,
    mesh_stylized: Query<&MeshMaterial3d<StylizedMaterial>>,
    mesh_standard: Query<&MeshMaterial3d<StandardMaterial>>,
    mut stylized_materials: ResMut<Assets<StylizedMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for (entity, mut flash, mut transform) in &mut query {
        flash.timer.tick(time.delta());

        let progress = flash.timer.fraction();

        // Scale-pop: раздувается до 1.15× и обратно (дешёвая per-frame операция)
        let scale_factor = if progress < 0.5 {
            1.0 + 0.15 * (progress / 0.5)
        } else {
            1.0 + 0.15 * (1.0 - (progress - 0.5) / 0.5)
        };
        transform.scale = Vec3::splat(scale_factor);

        // Emissive flash: выставляем ОДИН РАЗ при первом тике + кэшируем потомков
        if !flash.emissive_applied {
            flash.emissive_applied = true;
            let flash_color = LinearRgba::new(8.0, 6.0, 3.0, 1.0);
            for descendant in children_query.iter_descendants(entity) {
                let has_material = if let Ok(mat_handle) = mesh_stylized.get(descendant) {
                    if let Some(material) = stylized_materials.get_mut(&mat_handle.0) {
                        material.base.emissive = flash_color;
                    }
                    true
                } else if let Ok(mat_handle) = mesh_standard.get(descendant) {
                    if let Some(material) = standard_materials.get_mut(&mat_handle.0) {
                        material.emissive = flash_color;
                    }
                    true
                } else {
                    false
                };
                if has_material {
                    flash.cached_descendants.push(descendant);
                }
            }
        }

        if flash.timer.is_finished() {
            transform.scale = Vec3::ONE;

            // Сброс emissive ОДИН РАЗ при завершении — используем кэш вместо DFS
            for &descendant in &flash.cached_descendants {
                if let Ok(mat_handle) = mesh_stylized.get(descendant) {
                    if let Some(material) = stylized_materials.get_mut(&mat_handle.0) {
                        material.base.emissive = LinearRgba::BLACK;
                    }
                } else if let Ok(mat_handle) = mesh_standard.get(descendant) {
                    if let Some(material) = standard_materials.get_mut(&mat_handle.0) {
                        material.emissive = LinearRgba::BLACK;
                    }
                }
            }

            commands.entity(entity).remove::<HitFlash>();
        }
    }
}
