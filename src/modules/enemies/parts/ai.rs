use bevy::prelude::*;
use bevy::light::NotShadowCaster;
use avian3d::prelude::*;
use crate::modules::player::components::Player;
use crate::modules::enemies::components::*;
use crate::modules::combat::components::EnemyAttackCooldown;
use crate::modules::combat::parts::knockback::{Staggered, StaggerRecovery};
use crate::modules::world::GroundCircle;
use crate::modules::combat::parts::game_over::KillCount;
use bevy::ecs::system::Commands;

/// Система AI: враги реагируют на игрока по дистанции
/// - Далеко (> aggro_range): стоит на месте (Idle)
/// - Средне (attack_range*1.3..aggro_range): преследует (Walking/Running)
/// - Близко (<= attack_range*1.3) + есть слот: атакует (Attacking)
/// - Близко (<= attack_range*1.3) + нет слота: кружит (Orbiting → Walking anim)
pub fn enemy_ai_system(
    mut commands: Commands,
    time: Res<Time>,
    mut enemies: Query<
        (Entity, &ChasePlayer, &Health, &Children, &Transform, &mut LinearVelocity, &mut EnemyAnimState, &mut OrbitDirection, Option<&HasAttackSlot>),
        (With<Enemy>, Without<Player>, Without<EnemyDying>, Without<Staggered>, Without<StaggerRecovery>, Without<SpawnScream>)
    >,
    player: Query<&Transform, With<Player>>,
    mut model_query: Query<&mut Transform, (With<EnemyModel>, Without<Enemy>, Without<Player>)>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    let player_pos = player_transform.translation;
    let dt = time.delta_secs();

    for (entity, chase, health, children, enemy_transform, mut velocity, mut anim_state, mut orbit, has_slot) in &mut enemies {
        let enemy_pos = enemy_transform.translation;
        let distance = (player_pos - enemy_pos).length();

        // Вектор направления к игроку (только XZ плоскость)
        let direction = (player_pos - enemy_pos).normalize_or_zero();
        let direction_2d = Vec3::new(direction.x, 0.0, direction.z).normalize_or_zero();

        // Enrage: при HP < 30% — бежит вдвое быстрее
        let is_enraged = health.current / health.max < 0.3;
        let move_speed = if is_enraged { chase.speed * 2.0 } else { chase.speed };

        // Тикаем таймер смены направления орбиты
        orbit.change_timer.tick(time.delta());
        if orbit.change_timer.just_finished() {
            orbit.clockwise = !orbit.clockwise;
        }

        let orbit_radius = chase.attack_range * 1.2;

        let new_state = if distance <= chase.attack_range * 1.3 {
            if has_slot.is_some() {
                // Есть слот → идти к игроку и атаковать
                if distance > chase.attack_range {
                    velocity.0 = direction_2d * move_speed * 0.6;
                    if is_enraged { EnemyAnim::Running } else { EnemyAnim::Walking }
                } else {
                    velocity.0 = Vec3::ZERO;
                    EnemyAnim::Attacking
                }
            } else {
                // Нет слота → ORBIT: кружить вокруг игрока
                let tangent = if orbit.clockwise {
                    Vec3::new(-direction_2d.z, 0.0, direction_2d.x)
                } else {
                    Vec3::new(direction_2d.z, 0.0, -direction_2d.x)
                };

                // Радиальная коррекция: держать дистанцию orbit_radius
                let radial = if distance < orbit_radius * 0.9 {
                    -direction_2d * 2.0 // отойти
                } else if distance > orbit_radius * 1.3 {
                    direction_2d * 2.0  // подойти
                } else {
                    Vec3::ZERO
                };

                let orbit_speed = move_speed * 0.4;
                velocity.0 = (tangent * orbit_speed + radial).clamp_length_max(move_speed * 0.5);
                EnemyAnim::Walking
            }
        } else if distance <= chase.aggro_range {
            // В зоне агро — преследуем
            velocity.0 = direction_2d * move_speed;
            if is_enraged { EnemyAnim::Running } else { EnemyAnim::Walking }
        } else {
            // Далеко — стоим и ждём
            velocity.0 = Vec3::ZERO;
            EnemyAnim::Idle
        };

        // Обновляем состояние только если изменилось (Changed<> фильтр в анимации)
        if anim_state.current != new_state {
            anim_state.current = new_state;

            // Управляем таймером повтора анимации атаки
            if new_state == EnemyAnim::Attacking {
                commands.entity(entity).insert(EnemyAttackAnimTimer {
                    timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                });
            } else {
                commands.entity(entity).remove::<EnemyAttackAnimTimer>();
            }
        }

        // Поворачиваем child модель лицом к игроку (только в агро)
        if distance <= chase.aggro_range && direction_2d.length() > 0.01 {
            let target_rotation = Quat::from_rotation_y(direction_2d.x.atan2(direction_2d.z));
            let t = 1.0 - (-8.0 * dt).exp();

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
        (Entity, &Health, &Children, &mut EnemyAnimState, &mut LinearVelocity),
        (With<Enemy>, Without<EnemyDying>)
    >,
    ground_circles: Query<Entity, With<GroundCircle>>,
    mut kill_count: ResMut<KillCount>,
) {
    for (entity, health, children, mut anim_state, mut velocity) in &mut enemies {
        if health.is_dead() {
            kill_count.total += 1;
            debug!("💀 Enemy dying — playing death animation (kills: {})", kill_count.total);
            anim_state.current = EnemyAnim::Dying;
            velocity.0 = Vec3::ZERO;

            // Деспавним ground circle при смерти
            for child in children.iter() {
                if ground_circles.get(child).is_ok() {
                    commands.entity(child).despawn();
                }
            }

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

/// Тикает таймер смерти, после завершения — замораживает как статичный труп
pub fn process_dying_enemies(
    time: Res<Time>,
    mut commands: Commands,
    mut dying: Query<(Entity, &mut EnemyDying), With<Enemy>>,
) {
    for (entity, mut dying) in &mut dying {
        dying.timer.tick(time.delta());
        if dying.timer.is_finished() {
            commands.entity(entity)
                .remove::<EnemyDying>()
                .remove::<EnemyAnimState>()
                .remove::<Health>()
                .remove::<EnemyAttackCooldown>()
                .remove::<Enemy>()
                .insert(EnemyCorpse);
        }
    }
}

/// Облегчаем трупы: убираем анимацию (CPU) + тени (GPU).
/// Выполняется ОДИН раз при создании трупа (Added<EnemyCorpse>).
/// Результат: статичный меш в death-позе, без shadow pass.
pub fn strip_corpse_system(
    mut commands: Commands,
    new_corpses: Query<&Children, Added<EnemyCorpse>>,
    model_query: Query<Entity, With<EnemyModel>>,
    children_query: Query<&Children>,
    animation_query: Query<Entity, With<AnimationPlayer>>,
    mesh_query: Query<Entity, With<Mesh3d>>,
) {
    for corpse_children in &new_corpses {
        for &child in corpse_children {
            if model_query.get(child).is_ok() {
                // Убираем AnimationGraphHandle с модели
                commands.entity(child).remove::<AnimationGraphHandle>();

                // Обходим всех потомков
                for descendant in children_query.iter_descendants(child) {
                    // Убираем анимацию (CPU: AnimationPlayer больше не тикает)
                    if animation_query.get(descendant).is_ok() {
                        commands.entity(descendant)
                            .remove::<AnimationPlayer>()
                            .remove::<AnimationTransitions>()
                            .remove::<EnemyAnimations>()
                            .remove::<EnemyAnimationSetupComplete>();
                    }
                    // Отключаем тени на мешах (GPU: не рендерится в shadow pass)
                    if mesh_query.get(descendant).is_ok() {
                        commands.entity(descendant).insert(NotShadowCaster);
                    }
                }
            }
        }
    }
}

