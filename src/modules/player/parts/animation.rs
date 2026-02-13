use bevy::prelude::*;
use std::time::Duration;
use crate::modules::{Player, AnimatedCharacter, InputState};
use crate::modules::player::components::{AnimationState, PlayerAnimations, PlayerHitStagger, PlayerModel, StaggerCooldown};
use crate::modules::player::AnimationSetupComplete;
use crate::modules::combat::components::AttackCooldown;

// Пороги для предотвращения мерцания (hysteresis)
const MOVEMENT_START_THRESHOLD: f32 = 0.05;  // Начать движение
const MOVEMENT_STOP_THRESHOLD: f32 = 0.02;   // Остановиться

/// Система переключения анимаций на основе ввода
pub fn animation_state_system(
    input_state: Res<InputState>,
    mut player: Query<&mut AnimatedCharacter, With<Player>>,
    mut animation_query: Query<
        (&PlayerAnimations, &mut AnimationPlayer, &mut AnimationTransitions),
        With<AnimationSetupComplete>
    >,
) {
    if let Ok(mut character) = player.single_mut() {
        if let Ok((animations, mut anim_player, mut transitions)) = animation_query.single_mut() {
            let movement_magnitude = input_state.movement.length();
            let current_state = character.current_animation;

            // Не прерываем атаку или hit reaction движением
            if current_state == AnimationState::Attacking || current_state == AnimationState::HitReaction {
                return;
            }

            let movement_threshold = match current_state {
                AnimationState::Idle => MOVEMENT_START_THRESHOLD,
                _ => MOVEMENT_STOP_THRESHOLD,
            };

            let new_state = if movement_magnitude > movement_threshold {
                if input_state.is_running {
                    AnimationState::Running
                } else {
                    AnimationState::Walking
                }
            } else {
                AnimationState::Idle
            };

            if character.current_animation != new_state {
                character.current_animation = new_state;

                let animation_index = match new_state {
                    AnimationState::Idle => animations.idle,
                    AnimationState::Walking => animations.walk,
                    AnimationState::Running => animations.run,
                    AnimationState::Attacking => animations.attack,
                    AnimationState::HitReaction => animations.hit,
                };

                let transition = transitions
                    .play(&mut anim_player, animation_index, Duration::from_millis(200));
                if new_state != AnimationState::Attacking && new_state != AnimationState::HitReaction {
                    transition.repeat();
                }
            }
        }
    }
}

/// Diablo 4 Hit Recovery:
/// 1. Тикает таймер стаггера (0.3s)
/// 2. Emissive glow — set-once в начале, сброс в конце
/// 3. По завершении: idle анимация + reset cooldown → auto_attack подхватит сразу
pub fn player_hit_stagger_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut PlayerHitStagger, &mut AnimatedCharacter, &Children), With<Player>>,
    mut animation_query: Query<
        (&PlayerAnimations, &mut AnimationPlayer, &mut AnimationTransitions),
        With<AnimationSetupComplete>
    >,
    mut cooldown_query: Query<&mut AttackCooldown, With<Player>>,
    model_query: Query<Entity, With<PlayerModel>>,
    children_query: Query<&Children>,
    mesh_query: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut stagger, mut character, children) in &mut query {
        stagger.timer.tick(time.delta());

        // Emissive glow — выставляем ОДИН РАЗ при первом тике
        if !stagger.emissive_applied {
            stagger.emissive_applied = true;
            let glow = LinearRgba::new(6.0, 1.5, 0.5, 1.0);
            for child in children.iter() {
                if model_query.get(child).is_ok() {
                    for descendant in children_query.iter_descendants(child) {
                        if let Ok(mat_handle) = mesh_query.get(descendant) {
                            if let Some(material) = materials.get_mut(&mat_handle.0) {
                                material.emissive = glow;
                            }
                        }
                    }
                }
            }
        }

        if stagger.timer.is_finished() {
            character.current_animation = AnimationState::Idle;
            commands.entity(entity)
                .remove::<PlayerHitStagger>()
                .insert(StaggerCooldown {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                });

            // Запустить idle анимацию — чистое состояние для blend в attack
            if let Ok((animations, mut anim_player, mut transitions)) = animation_query.single_mut() {
                transitions.play(&mut anim_player, animations.idle, Duration::from_millis(150))
                    .repeat();
            }

            // Сбросить cooldown атаки → auto_attack подхватит сразу
            if let Ok(mut cooldown) = cooldown_query.single_mut() {
                cooldown.timer.finish();
            }

            // Сброс emissive ОДИН РАЗ при завершении
            for child in children.iter() {
                if model_query.get(child).is_ok() {
                    for descendant in children_query.iter_descendants(child) {
                        if let Ok(mat_handle) = mesh_query.get(descendant) {
                            if let Some(material) = materials.get_mut(&mat_handle.0) {
                                material.emissive = LinearRgba::BLACK;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Тикает таймер иммунитета к стаггеру, убирает по завершении
pub fn stagger_cooldown_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut StaggerCooldown), With<Player>>,
) {
    for (entity, mut cd) in &mut query {
        cd.timer.tick(time.delta());
        if cd.timer.is_finished() {
            commands.entity(entity).remove::<StaggerCooldown>();
        }
    }
}
