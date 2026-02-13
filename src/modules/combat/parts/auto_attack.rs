use bevy::prelude::*;
use std::time::Duration;
use avian3d::prelude::*;
use crate::modules::player::components::{Player, AnimatedCharacter, AnimationState, PlayerModel, PlayerAnimations, AnimationSetupComplete};
use crate::modules::enemies::components::{Enemy, Health, EnemyDying, EnemyModel, EnemyAnimState, EnemyAnim};
use crate::modules::combat::components::{Weapon, AttackCooldown, AttackAnimTimer, PendingAttack, MISS_RANGE_MULTIPLIER};
use super::camera_shake::CameraShake;
use super::knockback::Staggered;
use super::hit_flash::HitFlash;
use super::vfx_assets::HitVfxAssets;
use super::slash_vfx;
use super::damage_numbers;
use super::blood_decals;
use super::hit_particles;
use super::impact_flash;

/// Автоатака игрока: находит ближайшего врага → запускает замах → урон по таймеру
pub fn player_auto_attack_system(
    time: Res<Time>,
    mut player_query: Query<
        (Entity, &Weapon, &mut AttackCooldown, &Children, &Transform, &mut AnimatedCharacter),
        (With<Player>, Without<PendingAttack>)
    >,
    enemies: Query<(Entity, &Transform, &Health), (With<Enemy>, Without<EnemyDying>)>,
    mut model_query: Query<&mut Transform, (With<PlayerModel>, Without<Player>, Without<Enemy>)>,
    mut animation_query: Query<
        (&PlayerAnimations, &mut AnimationPlayer, &mut AnimationTransitions),
        With<AnimationSetupComplete>
    >,
    mut commands: Commands,
) {
    let Ok((player_entity, weapon, mut cooldown, children, player_transform, mut character)) =
        player_query.single_mut() else { return };

    // Во время стаггера нельзя атаковать (ARPG стандарт: action lock)
    if character.current_animation == AnimationState::HitReaction {
        return;
    }

    // Тикаем таймер
    cooldown.timer.tick(time.delta());

    if !cooldown.timer.is_finished() {
        return;
    }

    let player_pos = player_transform.translation;

    // Ищем ближайшего ЖИВОГО врага в радиусе
    let mut closest: Option<(Entity, f32, Vec3)> = None;
    for (entity, enemy_transform, health) in &enemies {
        if health.is_dead() { continue; } // Пропускаем уже мёртвых
        let enemy_pos = enemy_transform.translation;
        let distance = (enemy_pos - player_pos).length();
        if distance <= weapon.range {
            if closest.is_none() || distance < closest.unwrap().1 {
                closest = Some((entity, distance, enemy_pos));
            }
        }
    }

    let Some((target_entity, _distance, target_pos)) = closest else { return };

    // Нашли цель — запускаем замах!

    // Поворачиваем модель к врагу
    let direction = (target_pos - player_pos).normalize_or_zero();
    let direction_2d = Vec3::new(direction.x, 0.0, direction.z).normalize_or_zero();
    if direction_2d.length() > 0.01 {
        let target_rotation = Quat::from_rotation_y(direction_2d.x.atan2(direction_2d.z));
        for &child in children {
            if let Ok(mut model_transform) = model_query.get_mut(child) {
                model_transform.rotation = target_rotation;
            }
        }
    }

    // Анимация атаки — замах начинается (2.5× скорость: impact 1.05s → 0.42s)
    character.current_animation = AnimationState::Attacking;
    if let Ok((animations, mut anim_player, mut transitions)) = animation_query.single_mut() {
        transitions.play(&mut anim_player, animations.attack, Duration::from_millis(200));
        if let Some(active_anim) = anim_player.animation_mut(animations.attack) {
            active_anim.set_speed(2.5);
        }
    }

    // Откладываем урон до момента удара (0.25с в анимацию)
    commands.entity(player_entity).insert(PendingAttack {
        target: target_entity,
        damage: weapon.damage,
        direction: direction_2d,
        timer: Timer::from_seconds(0.42, TimerMode::Once),
        max_range: weapon.range * MISS_RANGE_MULTIPLIER,
    });

    // Сбрасываем cooldown
    cooldown.timer.reset();

    // Таймер для сброса анимации атаки (0.5с)
    commands.entity(player_entity).insert(
        AttackAnimTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        }
    );
}

/// Наносит урон при ударе анимации (после задержки замаха)
pub fn apply_pending_attack_system(
    time: Res<Time>,
    mut player_query: Query<(Entity, &Transform, &mut PendingAttack), With<Player>>,
    mut enemies: Query<(&Transform, &mut Health, &mut LinearVelocity, &Children, &mut EnemyAnimState), (With<Enemy>, Without<EnemyDying>)>,
    enemy_model_query: Query<Entity, With<EnemyModel>>,
    mut commands: Commands,
    mut camera_shake: ResMut<CameraShake>,
    vfx_assets: Res<HitVfxAssets>,
    blood_assets: Res<blood_decals::BloodDecalAssets>,
    slash_assets: Res<slash_vfx::SlashVfxAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for (player_entity, player_transform, mut pending) in &mut player_query {
        pending.timer.tick(time.delta());

        if !pending.timer.is_finished() {
            continue;
        }

        let player_pos = player_transform.translation;

        // Наносим урон — проверяем что враг ещё жив и существует
        if let Ok((enemy_transform, mut health, mut velocity, children, mut anim_state)) = enemies.get_mut(pending.target) {
            let enemy_pos = enemy_transform.translation;
            let distance = (enemy_pos - player_pos).length();

            if distance > pending.max_range {
                // MISS: враг увернулся — только slash VFX, без попадания
                slash_vfx::spawn_slash(
                    &mut commands, &slash_assets, &mut materials,
                    player_pos, pending.direction,
                );
                damage_numbers::spawn_miss_text(
                    &mut commands, &asset_server,
                    enemy_pos,
                );
                debug!(
                    "⚔️ Player MISSES enemy! (distance {:.1} > max_range {:.1})",
                    distance, pending.max_range
                );
            } else if !health.is_dead() {
                health.take_damage(pending.damage);

                // VFX: Slash огненная дуга перед игроком
                slash_vfx::spawn_slash(
                    &mut commands, &slash_assets, &mut materials,
                    player_pos, pending.direction,
                );

                // Hit particles — искры при попадании (кэшированные ассеты)
                hit_particles::spawn_hit_particles(
                    &mut commands, &vfx_assets,
                    enemy_pos,
                );

                // Impact flash — вспышка света в точке удара
                impact_flash::spawn_impact_flash(
                    &mut commands,
                    enemy_pos,
                );

                // Camera shake — направленный толчок камеры
                camera_shake.trigger(0.15, 0.15, pending.direction);

                // Knockback — толкаем врага от игрока
                let knockback_dir = pending.direction;
                velocity.0 = knockback_dir * 8.0;
                commands.entity(pending.target).insert(Staggered::new(0.35));

                // Hit reaction анимация
                anim_state.current = EnemyAnim::HitReaction;

                // Hit flash — scale-pop на модели врага (не на parent, чтобы круг не двигался)
                for child in children.iter() {
                    if enemy_model_query.get(child).is_ok() {
                        commands.entity(child).insert(HitFlash::new());
                        break;
                    }
                }

                // Damage number — всплывающее число урона
                damage_numbers::spawn_damage_number(
                    &mut commands, &asset_server,
                    enemy_pos, pending.damage,
                );

                // Blood decal — пятно крови на полу (остаётся навсегда)
                blood_decals::spawn_blood_decal(
                    &mut commands, &blood_assets,
                    enemy_pos, blood_decals::BloodColor::Red,
                );

                debug!(
                    "⚔️ Player hits enemy for {} damage! (HP: {}/{})",
                    pending.damage, health.current, health.max
                );
            }
        }

        // Убираем PendingAttack (удар выполнен или цель исчезла)
        commands.entity(player_entity).remove::<PendingAttack>();
    }
}

/// Сброс анимации атаки обратно в idle после проигрывания
pub fn attack_animation_reset_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut AttackAnimTimer, &mut AnimatedCharacter), With<Player>>,
    mut animation_query: Query<(&PlayerAnimations, &mut AnimationPlayer), With<AnimationSetupComplete>>,
    mut commands: Commands,
) {
    for (entity, mut anim_timer, mut character) in &mut query {
        anim_timer.timer.tick(time.delta());
        if anim_timer.timer.is_finished() {
            if character.current_animation == AnimationState::Attacking {
                character.current_animation = AnimationState::Idle;
                // Сбрасываем скорость анимации с 2.5× обратно на 1.0
                if let Ok((animations, mut anim_player)) = animation_query.single_mut() {
                    if let Some(active_anim) = anim_player.animation_mut(animations.attack) {
                        active_anim.set_speed(1.0);
                    }
                }
            }
            commands.entity(entity).remove::<AttackAnimTimer>();
        }
    }
}
