use bevy::prelude::*;
use avian3d::prelude::*;
use crate::modules::player::components::Player;
use crate::modules::enemies::components::*;

/// Система AI: враги реагируют на игрока по дистанции
/// - Далеко (> aggro_range): стоит на месте (Idle)
/// - Средне (attack_range..aggro_range): преследует (Walking)
/// - Близко (<= attack_range): атакует (Attacking)
pub fn enemy_ai_system(
    time: Res<Time>,
    mut enemies: Query<
        (&ChasePlayer, &Children, &Transform, &mut LinearVelocity, &mut EnemyAnimState),
        (With<Enemy>, Without<Player>, Without<EnemyDying>)
    >,
    player: Query<&Transform, With<Player>>,
    mut model_query: Query<&mut Transform, (With<EnemyModel>, Without<Enemy>, Without<Player>)>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    let player_pos = player_transform.translation;

    for (chase, children, enemy_transform, mut velocity, mut anim_state) in &mut enemies {
        let enemy_pos = enemy_transform.translation;
        let distance = (player_pos - enemy_pos).length();

        // Вектор направления к игроку (только XZ плоскость)
        let direction = (player_pos - enemy_pos).normalize_or_zero();
        let direction_2d = Vec3::new(direction.x, 0.0, direction.z).normalize_or_zero();

        let new_state = if distance <= chase.attack_range {
            // Близко — атакуем, стоим на месте
            velocity.0 = Vec3::ZERO;
            EnemyAnim::Attacking
        } else if distance <= chase.aggro_range {
            // В зоне агро — преследуем
            velocity.0 = direction_2d * chase.speed;
            EnemyAnim::Walking
        } else {
            // Далеко — стоим и ждём
            velocity.0 = Vec3::ZERO;
            EnemyAnim::Idle
        };

        // Обновляем состояние только если изменилось (Changed<> фильтр в анимации)
        if anim_state.current != new_state {
            anim_state.current = new_state;
        }

        // Поворачиваем child модель лицом к игроку (только в агро)
        if distance <= chase.aggro_range && direction_2d.length() > 0.01 {
            let target_rotation = Quat::from_rotation_y(direction_2d.x.atan2(direction_2d.z));
            let t = 1.0 - (-8.0 * time.delta_secs()).exp();

            for &child in children {
                if let Ok(mut model_transform) = model_query.get_mut(child) {
                    model_transform.rotation = model_transform.rotation.slerp(target_rotation, t);
                }
            }
        }
    }
}

/// Когда HP <= 0 — запускаем анимацию смерти (не despawn сразу)
pub fn start_enemy_death(
    mut commands: Commands,
    mut enemies: Query<
        (Entity, &Health, &mut EnemyAnimState, &mut LinearVelocity),
        (With<Enemy>, Without<EnemyDying>)
    >,
) {
    for (entity, health, mut anim_state, mut velocity) in &mut enemies {
        if health.is_dead() {
            info!("💀 Enemy dying — playing death animation");
            anim_state.current = EnemyAnim::Dying;
            velocity.0 = Vec3::ZERO;
            commands.entity(entity)
                .insert(EnemyDying {
                    timer: Timer::from_seconds(3.0, TimerMode::Once),
                })
                .remove::<ChasePlayer>()
                .remove::<RigidBody>()
                .remove::<Collider>()
                .remove::<LinearVelocity>()
                .remove::<LinearDamping>()
                .remove::<AngularDamping>();
        }
    }
}

