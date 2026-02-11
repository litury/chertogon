use bevy::prelude::*;
use std::time::Duration;
use crate::modules::{Player, AnimatedCharacter, InputState};
use crate::modules::player::components::{AnimationState, PlayerAnimations};
use crate::modules::player::AnimationSetupComplete;

// Пороги для предотвращения мерцания (hysteresis)
const MOVEMENT_START_THRESHOLD: f32 = 0.05;  // Начать движение
const MOVEMENT_STOP_THRESHOLD: f32 = 0.02;   // Остановиться

/// Система переключения анимаций на основе ввода
pub fn animation_state_system(
    input_state: Res<InputState>,
    mut player: Query<&mut AnimatedCharacter, With<Player>>,
    mut animation_query: Query<
        (&PlayerAnimations, &mut AnimationPlayer, &mut AnimationTransitions),
        With<AnimationSetupComplete>  // Только initialized players
    >,
) {
    if let Ok(mut character) = player.single_mut() {
        if let Ok((animations, mut anim_player, mut transitions)) = animation_query.single_mut() {
            // Определяем новое состояние с hysteresis
            let movement_magnitude = input_state.movement.length();
            let current_state = character.current_animation;

            // Не прерываем атаку движением — атака доиграет сама
            if current_state == AnimationState::Attacking {
                return;
            }

            // Hysteresis: разные пороги для начала и остановки
            let movement_threshold = match current_state {
                AnimationState::Idle => MOVEMENT_START_THRESHOLD,  // Нужно >0.05 чтобы начать
                _ => MOVEMENT_STOP_THRESHOLD,  // Нужно <0.02 чтобы остановиться
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

            // Переключаем ТОЛЬКО при изменении
            if character.current_animation != new_state {
                // Обновляем состояние
                character.current_animation = new_state;

                // Выбираем анимацию
                let animation_index = match new_state {
                    AnimationState::Idle => {
                        info!("🧍 Switching to Idle animation");
                        animations.idle
                    },
                    AnimationState::Walking => {
                        info!("🚶 Switching to Walking animation");
                        animations.walk
                    },
                    AnimationState::Running => {
                        info!("🏃 Switching to Running animation");
                        animations.run
                    },
                    AnimationState::Attacking => {
                        info!("⚔️ Switching to Attack animation");
                        animations.attack
                    },
                };

                // Плавный переход через AnimationTransitions (0.2 секунды)
                // Атака проигрывается один раз, остальные зацикливаются
                let transition = transitions
                    .play(&mut anim_player, animation_index, Duration::from_millis(200));
                if new_state != AnimationState::Attacking {
                    transition.repeat();
                }
            }
        }
    }
}
