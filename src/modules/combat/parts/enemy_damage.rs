use bevy::prelude::*;
use std::time::Duration;
use avian3d::prelude::*;
use crate::modules::player::components::{Player, AnimatedCharacter, AnimationState, PlayerAnimations, PlayerHitStagger};
use crate::modules::player::AnimationSetupComplete;
use crate::modules::enemies::components::{Enemy, EnemyAnimState, EnemyAnim, EnemyDying};
use crate::modules::combat::components::{PlayerHealth, EnemyAttackCooldown, PendingAttack, AttackAnimTimer};
use super::damage_vignette::DamageVignette;
use super::camera_shake::CameraShake;

/// Враг наносит контактный урон игроку когда в состоянии Attacking
/// Diablo 2 Hit Recovery: урон ВСЕГДА проходит, стаггер только если не уже в стаггере
pub fn enemy_contact_damage_system(
    time: Res<Time>,
    mut commands: Commands,
    mut enemies: Query<(&Transform, &EnemyAnimState, &mut EnemyAttackCooldown), (With<Enemy>, Without<EnemyDying>)>,
    mut player: Query<(Entity, &Transform, &mut PlayerHealth, &mut AnimatedCharacter, &mut LinearVelocity), With<Player>>,
    mut animation_query: Query<
        (&PlayerAnimations, &mut AnimationPlayer, &mut AnimationTransitions),
        With<AnimationSetupComplete>
    >,
    mut vignette: ResMut<DamageVignette>,
    mut camera_shake: ResMut<CameraShake>,
) {
    let Ok((player_entity, player_tf, mut player_health, mut character, mut velocity)) = player.single_mut() else { return };
    let player_pos = player_tf.translation;
    let already_staggered = character.current_animation == AnimationState::HitReaction;

    for (enemy_tf, anim_state, mut attack_cd) in &mut enemies {
        if anim_state.current == EnemyAnim::Attacking {
            attack_cd.timer.tick(time.delta());

            if attack_cd.timer.is_finished() {
                // Diablo 2: урон ВСЕГДА проходит
                player_health.take_damage(attack_cd.damage);

                let hit_dir = (player_pos - enemy_tf.translation).normalize_or_zero();

                // Виньетка + тряска при КАЖДОМ ударе
                vignette.trigger(0.6, 0.2);
                camera_shake.trigger(0.12, 0.12, hit_dir);

                // Стаггер только если НЕ уже в стаггере (Diablo 2 Hit Recovery)
                if !already_staggered {
                    character.current_animation = AnimationState::HitReaction;
                    // Diablo 4: удар прерывает текущую атаку — чистый рестарт после стаггера
                    commands.entity(player_entity)
                        .remove::<PendingAttack>()
                        .remove::<AttackAnimTimer>()
                        .insert(PlayerHitStagger {
                            timer: Timer::from_seconds(0.3, TimerMode::Once),
                            emissive_applied: false,
                        });

                    // Knockback
                    let knockback_dir = Vec3::new(hit_dir.x, 0.0, hit_dir.z).normalize_or_zero();
                    velocity.0 = knockback_dir * 8.0;

                    // Проиграть hit анимацию
                    if let Ok((animations, mut anim_player, mut transitions)) = animation_query.single_mut() {
                        transitions.play(&mut anim_player, animations.hit, Duration::from_millis(100));
                    }
                }

                attack_cd.timer.reset();
            }
        } else {
            attack_cd.timer.finish();
        }
    }
}
