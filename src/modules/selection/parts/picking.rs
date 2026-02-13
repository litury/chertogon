use bevy::prelude::*;
use crate::modules::enemies::components::{Enemy, EnemyDying};
use crate::modules::player::Player;
use crate::modules::combat::PlayerHealth;
use crate::modules::selection::components::{Selected, SelectionState, SelectionTapEvent};

const TAP_RADIUS_PX: f32 = 50.0;

/// Проецирует позиции персонажей в screen-space и выбирает ближайшего к точке тапа.
pub fn pick_character_at_screen_pos(
    mut tap_events: MessageReader<SelectionTapEvent>,
    mut selection: ResMut<SelectionState>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    enemies: Query<(Entity, &Transform), (With<Enemy>, Without<EnemyDying>)>,
    player: Query<(Entity, &Transform), With<Player>>,
    mut commands: Commands,
) {
    let Some(tap) = tap_events.read().last() else { return };
    let Ok((camera, cam_transform)) = camera_query.single() else { return };

    let mut best: Option<(Entity, f32)> = None;

    // Враги
    for (entity, transform) in &enemies {
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

    // Игрок
    if let Ok((player_entity, player_transform)) = player.single() {
        let world_pos = player_transform.translation + Vec3::Y * 0.9;
        if let Ok(screen_pos) = camera.world_to_viewport(cam_transform, world_pos) {
            let dist = (screen_pos - tap.screen_pos).length();
            if dist < TAP_RADIUS_PX {
                if best.is_none() || dist < best.unwrap().1 {
                    best = Some((player_entity, dist));
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

/// Сбрасывает выделение если персонаж умер или был despawned.
pub fn clear_dead_selection(
    mut selection: ResMut<SelectionState>,
    enemies: Query<(), (With<Enemy>, Without<EnemyDying>)>,
    player_health: Query<&PlayerHealth, With<Player>>,
) {
    let Some(entity) = selection.selected_entity else { return };

    // Враг умер или despawned
    if enemies.get(entity).is_ok() {
        return; // жив — оставляем
    }
    // Игрок
    if let Ok(health) = player_health.get(entity) {
        if !health.is_dead() {
            return; // жив — оставляем
        }
    }
    // Entity не враг и не живой игрок — сбрасываем
    selection.selected_entity = None;
}
