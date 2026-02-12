use bevy::prelude::*;
use std::time::Duration;
use crate::modules::enemies::components::*;
use crate::modules::enemies::parts::spawner::EnemyAnimationIndices;

/// Система настройки AnimationPlayer после загрузки GLB
/// Бежит каждый кадр пока AnimationPlayer не будет найден в иерархии
pub fn setup_enemy_animation(
    enemies: Query<(&Children, &EnemyAnimState), With<Enemy>>,
    model_query: Query<(&Children, &EnemyAnimationIndices, &AnimationGraphHandle), With<EnemyModel>>,
    mut animation_players: Query<
        (Entity, &mut AnimationPlayer),
        (Without<EnemyAnimationSetupComplete>, Without<EnemyModel>)
    >,
    children: Query<&Children>,
    mut commands: Commands,
) {
    for (enemy_children, anim_state) in &enemies {
        for &model_child in enemy_children {
            if let Ok((model_children, anim_indices, graph_handle)) = model_query.get(model_child) {
                let current_anim = anim_state.current;
                'search: for &child in model_children {
                    if let Ok((entity, mut player)) = animation_players.get_mut(child) {
                        setup_anim_player(entity, &mut player, anim_indices, graph_handle, current_anim, model_child, &mut commands);
                        break 'search;
                    }
                    if let Ok(grandchildren) = children.get(child) {
                        for &grandchild in grandchildren {
                            if let Ok((entity, mut player)) = animation_players.get_mut(grandchild) {
                                setup_anim_player(entity, &mut player, anim_indices, graph_handle, current_anim, model_child, &mut commands);
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
    model_child: Entity,
    commands: &mut Commands,
) {
    info!("✅ Enemy AnimationPlayer found!");

    let animations = EnemyAnimations {
        idle: anim_indices.idle,
        walk: anim_indices.walk,
        attack: anim_indices.attack,
        death: anim_indices.death,
    };

    commands.entity(entity).insert(animations);
    commands.entity(entity).insert(graph_handle.clone());

    let (anim_index, should_loop) = match current_anim {
        EnemyAnim::Idle => (animations.idle, true),
        EnemyAnim::Walking => (animations.walk, true),
        EnemyAnim::Attacking => (animations.attack, false),
        EnemyAnim::Dying => (animations.death, false),
    };
    let mut transitions = AnimationTransitions::new();
    let transition = transitions.play(player, anim_index, Duration::ZERO);
    if should_loop {
        transition.repeat();
    }

    commands.entity(entity).insert(transitions);
    commands.entity(entity).insert(EnemyAnimationSetupComplete);
    commands.entity(model_child).remove::<EnemyAnimationIndices>();

    info!("🎬 Enemy animation initialized (state: {:?})", current_anim);
}

/// Переключение анимации врага на основе состояния
/// Обходит иерархию: Enemy → EnemyModel children → AnimationPlayer
pub fn enemy_animation_state_system(
    enemies: Query<(&EnemyAnimState, &Children), (With<Enemy>, Changed<EnemyAnimState>)>,
    model_query: Query<&Children, With<EnemyModel>>,
    children_query: Query<&Children>,
    mut animation_query: Query<
        (&EnemyAnimations, &mut AnimationPlayer, &mut AnimationTransitions),
        With<EnemyAnimationSetupComplete>
    >,
) {
    for (anim_state, enemy_children) in &enemies {
        'enemy: for &child in enemy_children {
            if let Ok(model_children) = model_query.get(child) {
                for &mc in model_children {
                    if try_update_animation(&mut animation_query, mc, anim_state) {
                        break 'enemy;
                    }
                    if let Ok(grandchildren) = children_query.get(mc) {
                        for &gc in grandchildren {
                            if try_update_animation(&mut animation_query, gc, anim_state) {
                                break 'enemy;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn try_update_animation(
    animation_query: &mut Query<
        (&EnemyAnimations, &mut AnimationPlayer, &mut AnimationTransitions),
        With<EnemyAnimationSetupComplete>
    >,
    entity: Entity,
    anim_state: &EnemyAnimState,
) -> bool {
    let Ok((animations, mut player, mut transitions)) = animation_query.get_mut(entity) else {
        return false;
    };

    let (animation_index, should_loop) = match anim_state.current {
        EnemyAnim::Idle => (animations.idle, true),
        EnemyAnim::Walking => (animations.walk, true),
        EnemyAnim::Attacking => (animations.attack, false),
        EnemyAnim::Dying => (animations.death, false),
    };

    let transition = transitions.play(&mut player, animation_index, Duration::from_millis(200));
    if should_loop {
        transition.repeat();
    }

    true
}
