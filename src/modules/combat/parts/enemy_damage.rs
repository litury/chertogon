use bevy::prelude::*;
use std::time::Duration;
use avian3d::prelude::*;
use crate::modules::player::components::{Player, AnimatedCharacter, AnimationState, PlayerAnimations, PlayerHitStagger, PlayerModel, StaggerCooldown};
use crate::modules::player::AnimationSetupComplete;
use crate::modules::enemies::components::{Enemy, EnemyAnimState, EnemyAnim, EnemyDying};
use crate::modules::combat::components::{PlayerHealth, EnemyAttackCooldown, PendingAttack, AttackAnimTimer};
use super::damage_vignette::DamageVignette;
use super::camera_shake::CameraShake;
use super::hit_flash::HitFlash;
use super::hit_particles;
use super::damage_numbers;
use super::vfx_assets::HitVfxAssets;

/// Враг наносит контактный урон игроку когда в состоянии Attacking
/// Diablo 2 Hit Recovery: урон ВСЕГДА проходит, стаггер только если не уже в стаггере
pub fn enemy_contact_damage_system(
    time: Res<Time>,
    mut commands: Commands,
    mut enemies: Query<(&Transform, &EnemyAnimState, &mut EnemyAttackCooldown), (With<Enemy>, Without<EnemyDying>)>,
    mut player: Query<(Entity, &Transform, &mut PlayerHealth, &mut AnimatedCharacter, &mut LinearVelocity, &Children, Has<StaggerCooldown>), With<Player>>,
    mut animation_query: Query<
        (&PlayerAnimations, &mut AnimationPlayer, &mut AnimationTransitions),
        With<AnimationSetupComplete>
    >,
    player_model_query: Query<Entity, With<PlayerModel>>,
    mut vignette: ResMut<DamageVignette>,
    mut camera_shake: ResMut<CameraShake>,
    vfx_assets: Res<HitVfxAssets>,
    asset_server: Res<AssetServer>,
) {
    let Ok((player_entity, player_tf, mut player_health, mut character, mut velocity, children, has_stagger_cooldown)) = player.single_mut() else { return };
    let player_pos = player_tf.translation;
    let already_staggered = character.current_animation == AnimationState::HitReaction;

    for (enemy_tf, anim_state, mut attack_cd) in &mut enemies {
        if anim_state.current == EnemyAnim::Attacking {
            attack_cd.timer.tick(time.delta());

            if attack_cd.timer.is_finished() {
                // Проверка дистанции: промах если игрок убежал
                let distance = (player_pos - enemy_tf.translation).length();
                if distance > attack_cd.max_range {
                    damage_numbers::spawn_miss_text(
                        &mut commands, &asset_server,
                        player_pos,
                    );
                    attack_cd.timer.reset();
                    continue;
                }

                // Diablo 2: урон ВСЕГДА проходит (если в радиусе)
                player_health.take_damage(attack_cd.damage);

                let hit_dir = (player_pos - enemy_tf.translation).normalize_or_zero();

                // Усиленная виньетка + тряска при КАЖДОМ ударе
                vignette.trigger(0.7, 0.35);
                camera_shake.trigger(0.20, 0.15, hit_dir);

                // Hit particles — искры при ударе по герою
                hit_particles::spawn_hit_particles(
                    &mut commands, &vfx_assets,
                    player_pos,
                );

                // HitFlash (scale-pop + emissive) на модели героя
                for child in children.iter() {
                    if player_model_query.get(child).is_ok() {
                        commands.entity(child).insert(HitFlash::new());
                        break;
                    }
                }

                // Стаггер только если НЕ в стаггере и НЕ в окне иммунитета (Diablo 2 Hit Recovery)
                if !already_staggered && !has_stagger_cooldown {
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
        }
    }
}
