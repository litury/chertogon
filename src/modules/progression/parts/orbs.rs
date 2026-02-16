use bevy::prelude::*;
use crate::modules::player::components::Player;
use crate::modules::enemies::components::{EnemyType, EnemyDying};
use crate::modules::combat::components::PlayerHealth;
use crate::modules::combat::parts::vfx_assets::HitVfxAssets;
use crate::modules::progression::components::{XpOrb, HpOrb, PlayerXp};
use crate::shared::rand_01;
use super::orb_assets::OrbAssets;

// ── Спавн орбов при смерти врагов ──

/// Спавнит XP (и иногда HP) орбы при смерти врага
/// Использует `Added<EnemyDying>` — реагирует на смерть без зависимости на enemies/parts
pub fn spawn_orbs_on_enemy_death(
    new_dying: Query<(&Transform, &EnemyType), Added<EnemyDying>>,
    mut commands: Commands,
    orb_assets: Option<Res<OrbAssets>>,
) {
    let Some(orb_assets) = orb_assets else { return };

    for (transform, enemy_type) in &new_dying {
        let xp_value = match enemy_type {
            EnemyType::Upyr => 10.0,
            EnemyType::Leshiy => 15.0,
            EnemyType::Volkolak => 12.0,
        };

        let pos = transform.translation;

        // XP орб — всегда
        let offset = Vec3::new(
            (rand_01() - 0.5) * 1.0,
            0.5 + rand_01() * 0.5,
            (rand_01() - 0.5) * 1.0,
        );

        commands.spawn((
            Mesh3d(orb_assets.xp_mesh.clone()),
            MeshMaterial3d(orb_assets.xp_material.clone()),
            Transform::from_translation(pos + Vec3::Y * 0.5),
            XpOrb {
                xp_value,
                magnetized: false,
                age: 0.0,
                spawn_offset: offset,
            },
        ));

        // HP орб — 5% шанс
        if rand_01() < 0.05 {
            let hp_offset = Vec3::new(
                (rand_01() - 0.5) * 1.0,
                0.5 + rand_01() * 0.5,
                (rand_01() - 0.5) * 1.0,
            );

            commands.spawn((
                Mesh3d(orb_assets.hp_mesh.clone()),
                MeshMaterial3d(orb_assets.hp_material.clone()),
                Transform::from_translation(pos + Vec3::Y * 0.5),
                HpOrb {
                    heal_amount: 10.0,
                    magnetized: false,
                    age: 0.0,
                    spawn_offset: hp_offset,
                },
            ));
        }
    }
}

// ── Физика и сбор XP орбов ──

/// Движение XP орбов: выброс → bobbing → магнит → сбор
pub fn xp_orb_physics_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut orbs: Query<(Entity, &mut XpOrb, &mut Transform), Without<Player>>,
    mut commands: Commands,
    mut xp_res: ResMut<PlayerXp>,
    vfx_assets: Res<HitVfxAssets>,
) {
    let dt = time.delta_secs();
    if dt == 0.0 { return; } // Игра на паузе

    let Ok(player_tf) = player_query.single() else { return };
    let player_pos = player_tf.translation;
    let magnet_radius = xp_res.magnet_radius;

    for (entity, mut orb, mut transform) in &mut orbs {
        orb.age += dt;

        if !orb.magnetized {
            // Фаза 1: начальный выброс (первые 0.3с)
            if orb.age < 0.3 {
                let t = orb.age / 0.3;
                let eased = 1.0 - (1.0 - t) * (1.0 - t); // ease out
                transform.translation += orb.spawn_offset * dt * 3.0 * (1.0 - eased);
            }

            // Bobbing на месте
            let bob_y = 0.5 + (orb.age * 3.0).sin() * 0.15;
            transform.translation.y = transform.translation.y * 0.95 + bob_y * 0.05;

            // Проверяем магнит
            let dist = (transform.translation - player_pos).length();
            if dist < magnet_radius {
                orb.magnetized = true;
            }
        } else {
            // Фаза 2: летим к игроку (ускоряющийся lerp)
            let dir = (player_pos - transform.translation).normalize_or_zero();
            let speed = 8.0 + orb.age * 10.0; // Ускоряется с возрастом
            transform.translation += dir * speed * dt;

            // Сбор при контакте
            let dist = (transform.translation - player_pos).length();
            if dist < 0.8 {
                xp_res.add_xp(orb.xp_value);
                // Зелёный "+N XP" floating text
                spawn_xp_text(&mut commands, &vfx_assets.font, player_pos, orb.xp_value);
                commands.entity(entity).despawn();
            }
        }
    }
}

// ── Физика и сбор HP орбов ──

/// Движение HP орбов: выброс → bobbing → магнит → сбор
pub fn hp_orb_physics_system(
    time: Res<Time>,
    player_xp: Res<PlayerXp>,
    player_query: Query<&Transform, With<Player>>,
    mut player_health: Query<&mut PlayerHealth, With<Player>>,
    mut orbs: Query<(Entity, &mut HpOrb, &mut Transform), (Without<Player>, Without<XpOrb>)>,
    mut commands: Commands,
    vfx_assets: Res<HitVfxAssets>,
) {
    let dt = time.delta_secs();
    if dt == 0.0 { return; }

    let Ok(player_tf) = player_query.single() else { return };
    let player_pos = player_tf.translation;
    let magnet_radius = player_xp.magnet_radius;

    for (entity, mut orb, mut transform) in &mut orbs {
        orb.age += dt;

        if !orb.magnetized {
            if orb.age < 0.3 {
                let t = orb.age / 0.3;
                let eased = 1.0 - (1.0 - t) * (1.0 - t);
                transform.translation += orb.spawn_offset * dt * 3.0 * (1.0 - eased);
            }

            let bob_y = 0.5 + (orb.age * 2.5).sin() * 0.15;
            transform.translation.y = transform.translation.y * 0.95 + bob_y * 0.05;

            let dist = (transform.translation - player_pos).length();
            if dist < magnet_radius {
                orb.magnetized = true;
            }
        } else {
            let dir = (player_pos - transform.translation).normalize_or_zero();
            let speed = 8.0 + orb.age * 10.0;
            transform.translation += dir * speed * dt;

            let dist = (transform.translation - player_pos).length();
            if dist < 0.8 {
                // Хилим игрока
                if let Ok(mut health) = player_health.single_mut() {
                    health.current = (health.current + orb.heal_amount).min(health.max);
                }
                spawn_heal_text(&mut commands, &vfx_assets.font, player_pos, orb.heal_amount);
                commands.entity(entity).despawn();
            }
        }
    }
}

// ── Cleanup ──

/// Удаляет все орбы при входе в Playing (новый ран)
pub fn cleanup_orbs(
    mut commands: Commands,
    xp_orbs: Query<Entity, With<XpOrb>>,
    hp_orbs: Query<Entity, With<HpOrb>>,
) {
    for entity in &xp_orbs {
        commands.entity(entity).despawn();
    }
    for entity in &hp_orbs {
        commands.entity(entity).despawn();
    }
}

/// Сброс PlayerXp при новом ране
pub fn reset_player_xp(mut player_xp: ResMut<PlayerXp>) {
    *player_xp = PlayerXp::default();
}

// ── Floating text helpers ──

/// Зелёный "+N XP" текст (переиспользует паттерн DamageNumber)
fn spawn_xp_text(
    commands: &mut Commands,
    font: &Handle<Font>,
    position: Vec3,
    xp_amount: f32,
) {
    use crate::modules::combat::parts::damage_numbers::DamageNumber;

    let font = font.clone();
    let text = format!("+{}", xp_amount as i32);

    let seed = (position.x * 73.7 + position.z * 31.3).sin();
    let x_spread = seed * 1.0;

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            ..default()
        },
        Text::new(text),
        TextFont {
            font,
            font_size: 20.0 * 1.5,
            ..default()
        },
        TextColor(Color::srgb(0.2, 1.0, 0.5)),
        TextShadow {
            offset: Vec2::new(1.5, 1.5),
            color: Color::srgba(0.0, 0.0, 0.0, 0.9),
        },
        Visibility::Hidden,
        DamageNumber {
            timer: Timer::from_seconds(0.6, TimerMode::Once),
            world_position: position + Vec3::new(x_spread * 0.3, 2.2, 0.0),
            velocity: Vec3::new(x_spread, 3.0, 0.0),
            base_font_size: 20.0,
        },
    ));
}

/// Зелёный "+N HP" текст
fn spawn_heal_text(
    commands: &mut Commands,
    font: &Handle<Font>,
    position: Vec3,
    heal_amount: f32,
) {
    use crate::modules::combat::parts::damage_numbers::DamageNumber;

    let font = font.clone();
    let text = format!("+{} HP", heal_amount as i32);

    let seed = (position.x * 53.7 + position.z * 41.3).sin();
    let x_spread = seed * 1.0;

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            ..default()
        },
        Text::new(text),
        TextFont {
            font,
            font_size: 22.0 * 1.5,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.3, 0.3)),
        TextShadow {
            offset: Vec2::new(1.5, 1.5),
            color: Color::srgba(0.0, 0.0, 0.0, 0.9),
        },
        Visibility::Hidden,
        DamageNumber {
            timer: Timer::from_seconds(0.6, TimerMode::Once),
            world_position: position + Vec3::new(x_spread * 0.3, 2.5, 0.0),
            velocity: Vec3::new(x_spread, 3.5, 0.0),
            base_font_size: 22.0,
        },
    ));
}
