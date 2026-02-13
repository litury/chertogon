use bevy::prelude::*;
use crate::modules::enemies::components::{Enemy, EnemyDying};
use crate::modules::selection::components::{Selected, SelectionState, SelectionTapEvent, PortraitCamera};

const TAP_RADIUS_PX: f32 = 50.0;

/// Проецирует позиции врагов в screen-space и выбирает ближайшего к точке тапа.
pub fn pick_enemy_at_screen_pos(
    mut tap_events: MessageReader<SelectionTapEvent>,
    mut selection: ResMut<SelectionState>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera3d>, Without<PortraitCamera>)>,
    enemies: Query<(Entity, &Transform), (With<Enemy>, Without<EnemyDying>)>,
    mut commands: Commands,
) {
    // Берём только последний тап (если было несколько за кадр)
    let Some(tap) = tap_events.read().last() else { return };
    let Ok((camera, cam_transform)) = camera_query.single() else { return };

    let mut best: Option<(Entity, f32)> = None;

    for (entity, transform) in &enemies {
        // Проецируем центр масс врага (поднимаем на ~0.9 от пола)
        let world_pos = transform.translation + Vec3::Y * 0.9;
        if let Ok(screen_pos) = camera.world_to_viewport(cam_transform, world_pos) {
            let dist = (screen_pos - tap.screen_pos).length();
            if dist < TAP_RADIUS_PX {
                if best.is_none() || dist < best.unwrap().1 {
                    best = Some((entity, dist));
                }
            }
        }
    }

    // Снимаем старое выделение
    if let Some(old_entity) = selection.selected_entity {
        if let Ok(mut cmd) = commands.get_entity(old_entity) {
            cmd.remove::<Selected>();
        }
    }

    // Новое выделение
    if let Some((entity, _)) = best {
        selection.selected_entity = Some(entity);
        commands.entity(entity).insert(Selected);
    } else {
        selection.selected_entity = None;
    }
}

/// Сбрасывает выделение если враг умер или был despawned.
pub fn clear_dead_selection(
    mut selection: ResMut<SelectionState>,
    enemies: Query<(), (With<Enemy>, Without<EnemyDying>)>,
) {
    if let Some(entity) = selection.selected_entity {
        if enemies.get(entity).is_err() {
            selection.selected_entity = None;
        }
    }
}
