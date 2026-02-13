use bevy::prelude::*;
use crate::modules::enemies::components::*;
use crate::shared::rand_01;
use crate::modules::player::components::Player;
use crate::modules::combat::components::EnemyAttackCooldown;

/// Выделяет attack slots ближайшим врагам (max 4 одновременных атакующих)
pub fn attack_slot_system(
    mut commands: Commands,
    slot_manager: Res<AttackSlotManager>,
    player: Query<&Transform, With<Player>>,
    slotted: Query<Entity, (With<HasAttackSlot>, With<Enemy>, Without<EnemyDying>)>,
    unslotted: Query<
        (Entity, &Transform, &ChasePlayer),
        (With<Enemy>, Without<HasAttackSlot>, Without<EnemyDying>)
    >,
    mut candidates: Local<Vec<(Entity, f32)>>,
) {
    let Ok(player_tf) = player.single() else { return };
    let player_pos = player_tf.translation;

    let active = slotted.iter().count() as u32;
    let available = slot_manager.max_slots.saturating_sub(active);

    if available == 0 {
        return;
    }

    // Кандидаты: враги без слота в радиусе attack_range * 1.5 (squared для скорости)
    // Local<Vec> — capacity переиспользуется между кадрами (0 аллокаций в steady state)
    candidates.clear();
    candidates.extend(unslotted.iter().filter_map(|(entity, tf, chase)| {
        let dist_sq = (player_pos - tf.translation).length_squared();
        let max = chase.attack_range * 1.5;
        if dist_sq <= max * max {
            Some((entity, dist_sq))
        } else {
            None
        }
    }));

    // Partial sort: нужны только top-N ближайших, полная сортировка не нужна
    let n = (available as usize).min(candidates.len());
    if n > 0 {
        candidates.select_nth_unstable_by(n - 1, |a, b| a.1.partial_cmp(&b.1).unwrap());
    }

    // Выдаём слоты
    for &(entity, _) in candidates.iter().take(n) {
        commands.entity(entity).insert(HasAttackSlot);
    }
}

/// Освобождает attack slots: ротация атакующих + cleanup далёких
pub fn release_attack_slot_system(
    mut commands: Commands,
    slotted: Query<
        (Entity, &Transform, &EnemyAttackCooldown),
        (With<HasAttackSlot>, With<Enemy>, Without<EnemyDying>)
    >,
    player: Query<&Transform, With<Player>>,
) {
    let Ok(player_tf) = player.single() else { return };
    let player_pos = player_tf.translation;

    for (entity, tf, cd) in &slotted {
        let dist_sq = (player_pos - tf.translation).length_squared();

        // Отпустить слот если враг далеко (отброшен knockback'ом)
        if dist_sq > 36.0 { // 6.0²
            commands.entity(entity).remove::<HasAttackSlot>();
            continue;
        }

        // После атаки → 30% шанс отпустить слот (ротация атакующих)
        // Проверяем что cooldown только что сброшен (fraction < 0.1 = начало нового цикла)
        if cd.timer.fraction() > 0.05 && cd.timer.fraction() < 0.15 {
            if rand_01() < 0.3 {
                commands.entity(entity).remove::<HasAttackSlot>();
            }
        }
    }
}
