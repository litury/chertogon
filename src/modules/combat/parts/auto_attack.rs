use bevy::prelude::*;
use std::time::Duration;
use crate::modules::player::components::{Player, AnimatedCharacter, AnimationState, PlayerModel, PlayerAnimations, AnimationSetupComplete};
use crate::modules::enemies::components::{Enemy, Health, EnemyDying};
use crate::modules::combat::components::{Weapon, AttackCooldown, AttackAnimTimer};
use super::camera_shake::CameraShake;
use super::slash_vfx;
use super::hit_particles;

/// Автоатака игрока: находит ближайшего врага в радиусе и бьёт
pub fn player_auto_attack_system(
    time: Res<Time>,
    mut player_query: Query<
        (Entity, &Weapon, &mut AttackCooldown, &Children, &Transform, &mut AnimatedCharacter),
        With<Player>
    >,
    mut enemies: Query<(Entity, &Transform, &mut Health), (With<Enemy>, Without<EnemyDying>)>,
    mut model_query: Query<&mut Transform, (With<PlayerModel>, Without<Player>, Without<Enemy>)>,
    mut animation_query: Query<
        (&PlayerAnimations, &mut AnimationPlayer, &mut AnimationTransitions),
        With<AnimationSetupComplete>
    >,
    mut commands: Commands,
    mut camera_shake: ResMut<CameraShake>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok((player_entity, weapon, mut cooldown, children, player_transform, mut character)) =
        player_query.single_mut() else { return };

    // Тикаем таймер
    cooldown.timer.tick(time.delta());

    if !cooldown.timer.is_finished() {
        return;
    }

    let player_pos = player_transform.translation;

    // Ищем ближайшего врага в радиусе (исключая умирающих)
    let mut closest: Option<(Entity, f32, Vec3)> = None;
    for (entity, enemy_transform, _health) in &enemies {
        let enemy_pos = enemy_transform.translation;
        let distance = (enemy_pos - player_pos).length();
        if distance <= weapon.range {
            if closest.is_none() || distance < closest.unwrap().1 {
                closest = Some((entity, distance, enemy_pos));
            }
        }
    }

    let Some((target_entity, _distance, target_pos)) = closest else { return };

    // Нашли цель — атакуем!

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

    // Анимация атаки — играем transition напрямую
    character.current_animation = AnimationState::Attacking;
    if let Ok((animations, mut anim_player, mut transitions)) = animation_query.single_mut() {
        transitions.play(&mut anim_player, animations.attack, Duration::from_millis(200));
    }

    // Наносим урон
    if let Ok((_entity, enemy_transform, mut health)) = enemies.get_mut(target_entity) {
        health.take_damage(weapon.damage);

        let enemy_pos = enemy_transform.translation;

        // VFX: Slash дуга перед игроком
        slash_vfx::spawn_slash(
            &mut commands, &mut meshes, &mut materials,
            player_pos, direction_2d,
        );

        // VFX: Искры при попадании
        hit_particles::spawn_hit_particles(
            &mut commands, &mut meshes, &mut materials,
            enemy_pos,
        );

        // Camera shake
        camera_shake.trigger(0.15, 0.15);

        info!(
            "⚔️ Player hits enemy for {} damage! (HP: {}/{})",
            weapon.damage, health.current, health.max
        );
    }

    // Сбрасываем cooldown
    cooldown.timer.reset();

    // Ставим таймер для сброса анимации атаки (0.5с)
    commands.entity(player_entity).insert(
        AttackAnimTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        }
    );
}

/// Сброс анимации атаки обратно в idle после проигрывания
pub fn attack_animation_reset_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut AttackAnimTimer, &mut AnimatedCharacter), With<Player>>,
    mut commands: Commands,
) {
    for (entity, mut anim_timer, mut character) in &mut query {
        anim_timer.timer.tick(time.delta());
        if anim_timer.timer.is_finished() {
            if character.current_animation == AnimationState::Attacking {
                character.current_animation = AnimationState::Idle;
            }
            commands.entity(entity).remove::<AttackAnimTimer>();
        }
    }
}
