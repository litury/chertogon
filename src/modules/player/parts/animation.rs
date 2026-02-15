use bevy::prelude::*;
use std::time::Duration;
use crate::modules::{Player, PlayerAnimState, InputState};
use crate::modules::player::components::{AnimationState, PlayerAnimations, PlayerHitStagger, PlayerModel, StaggerCooldown};
use crate::modules::player::AnimationSetupComplete;
use crate::modules::combat::components::AttackCooldown;

// Пороги для предотвращения мерцания (hysteresis)
const MOVEMENT_START_THRESHOLD: f32 = 0.05;  // Начать движение
const MOVEMENT_STOP_THRESHOLD: f32 = 0.02;   // Остановиться

/// Вычисляет желаемое состояние анимации по вводу.
/// Только меняет `current` — центральная система применит переход.
pub fn animation_state_system(
    input_state: Res<InputState>,
    mut player: Query<&mut PlayerAnimState, With<Player>>,
) {
    let Ok(mut state) = player.single_mut() else { return };

    // Не прерываем атаку или hit reaction движением
    if state.current == AnimationState::Attacking || state.current == AnimationState::HitReaction {
        return;
    }

    let movement_magnitude = input_state.movement.length();

    let movement_threshold = match state.current {
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

    if state.current != new_state {
        state.current = new_state;
    }
}

/// Центральная система переходов анимации игрока.
/// Единственное место где вызывается `transitions.play()`.
/// Аналог `enemy_animation_state_system` для врагов.
pub fn player_animation_transition_system(
    mut player: Query<&mut PlayerAnimState, With<Player>>,
    mut animation_query: Query<
        (&PlayerAnimations, &mut AnimationPlayer, &mut AnimationTransitions),
        With<AnimationSetupComplete>
    >,
) {
    let Ok(mut state) = player.single_mut() else { return };
    if !state.needs_transition() { return; }
    let Ok((animations, mut anim_player, mut transitions)) = animation_query.single_mut() else { return };

    let (index, should_loop, speed, blend_ms) = match state.current {
        AnimationState::Idle       => (animations.idle,   true,  1.0, 200),
        AnimationState::Walking    => (animations.walk,   true,  1.0, 200),
        AnimationState::Running    => (animations.run,    true,  1.0, 200),
        AnimationState::Attacking  => (animations.attack, false, 2.5, 200),
        AnimationState::HitReaction => (animations.hit,   false, 1.0, 100),
    };

    let transition = transitions.play(&mut anim_player, index, Duration::from_millis(blend_ms));
    if should_loop {
        transition.repeat();
    }
    if let Some(active) = anim_player.animation_mut(index) {
        active.set_speed(speed);
    }

    state.mark_applied();
}

/// Diablo 4 Hit Recovery:
/// 1. Тикает таймер стаггера (0.3s)
/// 2. Emissive glow — set-once в начале, сброс в конце
/// 3. По завершении: state → Idle + reset cooldown → auto_attack подхватит сразу
pub fn player_hit_stagger_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut PlayerHitStagger, &mut PlayerAnimState, &Children), With<Player>>,
    mut cooldown_query: Query<&mut AttackCooldown, With<Player>>,
    model_query: Query<Entity, With<PlayerModel>>,
    children_query: Query<&Children>,
    mesh_query: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut stagger, mut state, children) in &mut query {
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
            state.current = AnimationState::Idle;
            commands.entity(entity)
                .remove::<PlayerHitStagger>()
                .insert(StaggerCooldown {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                });

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
