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
                let setup_entity = |entity: Entity,
                                   player: &mut AnimationPlayer,
                                   commands: &mut Commands| {
                    info!("✅ Enemy AnimationPlayer found!");

                    let animations = EnemyAnimations {
                        idle: anim_indices.idle,
                        walk: anim_indices.walk,
                        attack: anim_indices.attack,
                        death: anim_indices.death,
                    };

                    commands.entity(entity).insert(animations);
                    commands.entity(entity).insert(graph_handle.clone());

                    // Запускаем анимацию на основе текущего состояния AI
                    let (anim_index, should_loop) = match current_anim {
                        EnemyAnim::Idle => (animations.idle, true),
                        EnemyAnim::Walking => (animations.walk, true),
                        EnemyAnim::Attacking => (animations.attack, false),
                        EnemyAnim::Dying => (animations.death, false),
                    };
                    let mut transitions = AnimationTransitions::new();
                    let transition = transitions
                        .play(player, anim_index, Duration::ZERO);
                    if should_loop {
                        transition.repeat();
                    }

                    commands.entity(entity).insert(transitions);
                    commands.entity(entity).insert(EnemyAnimationSetupComplete);
                    commands.entity(model_child).remove::<EnemyAnimationIndices>();

                    info!("🎬 Enemy animation initialized (state: {:?})", current_anim);
                };

                // Ищем AnimationPlayer в children и grandchildren
                for &child in model_children {
                    if let Ok((entity, mut player)) = animation_players.get_mut(child) {
                        setup_entity(entity, &mut player, &mut commands);
                        return;
                    }
                    if let Ok(grandchildren) = children.get(child) {
                        for &grandchild in grandchildren {
                            if let Ok((entity, mut player)) = animation_players.get_mut(grandchild) {
                                setup_entity(entity, &mut player, &mut commands);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Переключение анимации врага на основе состояния
pub fn enemy_animation_state_system(
    enemies: Query<&EnemyAnimState, (With<Enemy>, Changed<EnemyAnimState>)>,
    mut animation_query: Query<
        (&EnemyAnimations, &mut AnimationPlayer, &mut AnimationTransitions),
        With<EnemyAnimationSetupComplete>
    >,
) {
    for anim_state in &enemies {
        if let Ok((animations, mut anim_player, mut transitions)) = animation_query.single_mut() {
            let (animation_index, should_loop) = match anim_state.current {
                EnemyAnim::Idle => (animations.idle, true),
                EnemyAnim::Walking => (animations.walk, true),
                EnemyAnim::Attacking => (animations.attack, false),
                EnemyAnim::Dying => (animations.death, false),
            };

            let transition = transitions
                .play(&mut anim_player, animation_index, Duration::from_millis(200));
            if should_loop {
                transition.repeat();
            }
        }
    }
}
