use bevy::prelude::*;
use std::time::Duration;
use avian3d::prelude::*;
use crate::modules::enemies::components::*;
use crate::modules::enemies::parts::spawner::EnemyAnimationIndices;

/// Система настройки AnimationPlayer после загрузки GLB.
/// Бежит каждый кадр пока AnimationPlayer не будет найден в иерархии.
/// Кэширует Entity AnimationPlayer на parent Enemy (CachedAnimPlayer).
pub fn setup_enemy_animation(
    enemies: Query<(Entity, &Children, &EnemyAnimState), With<Enemy>>,
    model_query: Query<(&Children, &EnemyAnimationIndices, &AnimationGraphHandle), With<EnemyModel>>,
    mut animation_players: Query<
        (Entity, &mut AnimationPlayer),
        (Without<EnemyAnimationSetupComplete>, Without<EnemyModel>)
    >,
    children: Query<&Children>,
    mut commands: Commands,
) {
    for (enemy_entity, enemy_children, anim_state) in &enemies {
        for &model_child in enemy_children {
            if let Ok((model_children, anim_indices, graph_handle)) = model_query.get(model_child) {
                let current_anim = anim_state.current;
                'search: for &child in model_children {
                    if let Ok((entity, mut player)) = animation_players.get_mut(child) {
                        setup_anim_player(entity, &mut player, anim_indices, graph_handle, current_anim, enemy_entity, &mut commands);
                        break 'search;
                    }
                    if let Ok(grandchildren) = children.get(child) {
                        for &grandchild in grandchildren {
                            if let Ok((entity, mut player)) = animation_players.get_mut(grandchild) {
                                setup_anim_player(entity, &mut player, anim_indices, graph_handle, current_anim, enemy_entity, &mut commands);
                                break 'search;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn setup_anim_player(
    entity: Entity,
    player: &mut AnimationPlayer,
    anim_indices: &EnemyAnimationIndices,
    graph_handle: &AnimationGraphHandle,
    current_anim: EnemyAnim,
    enemy_entity: Entity,
    commands: &mut Commands,
) {
    debug!("✅ Enemy AnimationPlayer found on {:?}! idle={:?}, walk={:?}, run={:?}, attack={:?}",
          entity, anim_indices.idle, anim_indices.walk, anim_indices.run, anim_indices.attack);

    let animations = EnemyAnimations {
        idle: anim_indices.idle,
        walk: anim_indices.walk,
        run: anim_indices.run,
        attack: anim_indices.attack,
        death: anim_indices.death,
        hit: anim_indices.hit,
        scream: anim_indices.scream,
    };

    commands.entity(entity).insert(animations);
    commands.entity(entity).insert(graph_handle.clone());

    // Запускаем начальную анимацию сразу (player.start() — немедленная мутация)
    let (anim_index, should_loop) = match current_anim {
        EnemyAnim::Idle => (anim_indices.idle, true),
        EnemyAnim::Walking => (anim_indices.walk, true),
        EnemyAnim::Running => (anim_indices.run, true),
        EnemyAnim::Attacking => (anim_indices.attack, false),
        EnemyAnim::HitReaction => (anim_indices.hit, false),
        EnemyAnim::Screaming => (anim_indices.scream, false),
        EnemyAnim::Dying => (anim_indices.death, false),
    };
    let mut transitions = AnimationTransitions::new();
    let transition = transitions.play(player, anim_index, Duration::ZERO);
    if should_loop {
        transition.repeat();
    }
    commands.entity(entity).insert(transitions);
    commands.entity(entity).insert(EnemyAnimationSetupComplete);

    // Кэшируем Entity AnimationPlayer на parent Enemy для O(1) доступа
    commands.entity(enemy_entity).insert(CachedAnimPlayer { entity });

    debug!("🎬 Enemy animation setup done (state: {:?})", current_anim);
}

/// Центральная система переходов анимации врагов.
/// Единственное место где вызывается `transitions.play()`.
/// Использует CachedAnimPlayer для O(1) доступа вместо обхода иерархии.
pub fn enemy_animation_state_system(
    mut enemies: Query<(&mut EnemyAnimState, &CachedAnimPlayer), With<Enemy>>,
    mut animation_query: Query<
        (&EnemyAnimations, &mut AnimationPlayer, &mut AnimationTransitions),
        With<EnemyAnimationSetupComplete>
    >,
) {
    for (mut anim_state, cached) in &mut enemies {
        if !anim_state.needs_transition() { continue; }

        let Ok((animations, mut player, mut transitions)) = animation_query.get_mut(cached.entity) else {
            continue;
        };

        let (animation_index, should_loop) = match anim_state.current {
            EnemyAnim::Idle => (animations.idle, true),
            EnemyAnim::Walking => (animations.walk, true),
            EnemyAnim::Running => (animations.run, true),
            EnemyAnim::Attacking => (animations.attack, false),
            EnemyAnim::HitReaction => (animations.hit, false),
            EnemyAnim::Screaming => (animations.scream, false),
            EnemyAnim::Dying => (animations.death, false),
        };

        let transition = transitions.play(&mut player, animation_index, Duration::from_millis(200));
        if should_loop {
            transition.repeat();
        }
        anim_state.mark_applied();
    }
}

/// Динамическая скорость анимации walk/run на основе реальной скорости движения.
/// Без этого быстрые враги (Волколак 7.0, Леший 6.0) "скользят" — ноги не успевают за телом.
/// Использует CachedAnimPlayer для O(1) доступа вместо обхода иерархии.
pub fn enemy_anim_speed_system(
    mut enemies: Query<
        (&EnemyAnimState, &LinearVelocity, &ChasePlayer, &CachedAnimPlayer, &EnemyLod, &mut CachedAnimSpeed),
        (With<Enemy>, Without<EnemyDying>)
    >,
    mut animation_query: Query<
        (&EnemyAnimations, &mut AnimationPlayer),
        With<EnemyAnimationSetupComplete>
    >,
) {
    for (anim_state, velocity, chase, cached, lod, mut anim_speed) in &mut enemies {
        // Minimal LOD: анимация заморожена, не обновляем скорость
        if *lod == EnemyLod::Minimal { continue; }

        let reference_speed = match anim_state.current {
            EnemyAnim::Walking => chase.anim_base_speed,
            EnemyAnim::Running => chase.anim_base_speed * 1.8,
            _ => continue,
        };

        let actual_speed = velocity.0.length();
        if actual_speed < 0.1 {
            continue;
        }

        let speed_factor = (actual_speed / reference_speed).clamp(0.3, 4.0);

        // Throttle: пропускаем если изменение < 5%
        if (speed_factor - anim_speed.last_factor).abs() < anim_speed.last_factor * 0.05 {
            continue;
        }
        anim_speed.last_factor = speed_factor;

        // O(1) прямой lookup через кэш
        let Ok((animations, mut player)) = animation_query.get_mut(cached.entity) else {
            continue;
        };

        let index = match anim_state.current {
            EnemyAnim::Walking => animations.walk,
            EnemyAnim::Running => animations.run,
            _ => continue,
        };

        if let Some(active) = player.animation_mut(index) {
            active.set_speed(speed_factor);
        }
    }
}

/// Повторяет анимацию атаки пока враг в Attacking состоянии.
/// Синхронизирован с EnemyAttackCooldown (1.0с) — каждый удар имеет визуальный фидбек.
/// Вместо прямого transitions.play() вызывает request_replay() —
/// центральная система подхватит и переиграет без self-transition.
pub fn enemy_attack_anim_replay_system(
    time: Res<Time>,
    mut enemies: Query<
        (&mut EnemyAttackAnimTimer, &mut EnemyAnimState),
        (With<Enemy>, Without<EnemyDying>)
    >,
) {
    for (mut anim_timer, mut anim_state) in &mut enemies {
        anim_timer.timer.tick(time.delta());
        if anim_timer.timer.just_finished() {
            anim_state.request_replay();
        }
    }
}

/// Система: тикает таймер крика при спавне, по завершении переводит в Idle
pub fn spawn_scream_decay_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpawnScream, &mut EnemyAnimState), With<Enemy>>,
    mut commands: Commands,
) {
    for (entity, mut scream, mut anim_state) in &mut query {
        scream.timer.tick(time.delta());
        if scream.timer.is_finished() {
            anim_state.current = EnemyAnim::Idle;
            commands.entity(entity).remove::<SpawnScream>();
        }
    }
}
