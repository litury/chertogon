use bevy::prelude::*;
use crate::modules::player::components::{Player, PlayerModel};
use super::blood_decals::{BloodDecal, BloodDecalAssets, BloodColor, Footprint, spawn_blood_footprint};

/// Компонент на игроке: ноги в крови, оставляет следы
#[derive(Component)]
pub struct BloodyFeet {
    /// Таймер жизни эффекта (следы пока не истёк)
    pub duration: Timer,
    /// Таймер интервала между следами
    pub step_timer: Timer,
    /// Цвет последней наступленной крови
    pub color: BloodColor,
}

/// Детект контакта героя с лужами крови.
/// Если расстояние до BloodDecal < 1м → обновить/добавить BloodyFeet.
pub fn detect_blood_contact_system(
    mut commands: Commands,
    player: Query<(Entity, &Transform), With<Player>>,
    blood_decals: Query<(&Transform, &BloodDecal), Without<Footprint>>,
    mut bloody_feet: Query<&mut BloodyFeet>,
) {
    let Ok((player_entity, player_tf)) = player.single() else { return };
    let player_pos = player_tf.translation;

    for (decal_tf, decal) in &blood_decals {
        let decal_pos = decal_tf.translation;
        let dist = Vec2::new(player_pos.x - decal_pos.x, player_pos.z - decal_pos.z).length();

        if dist < 1.0 {
            // Наступили на кровь — обновляем или добавляем BloodyFeet
            if let Ok(mut feet) = bloody_feet.get_mut(player_entity) {
                feet.duration.reset();
                feet.color = decal.color;
            } else {
                commands.entity(player_entity).insert(BloodyFeet {
                    duration: Timer::from_seconds(3.0, TimerMode::Once),
                    step_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    color: decal.color,
                });
            }
            break; // Достаточно одной лужи
        }
    }
}

/// Спавнит маленькие кровавые следы пока BloodyFeet активен.
pub fn spawn_footprints_system(
    time: Res<Time>,
    mut commands: Commands,
    mut player: Query<(Entity, &Transform, &mut BloodyFeet, &Children), With<Player>>,
    model_query: Query<&Transform, With<PlayerModel>>,
    blood_assets: Res<BloodDecalAssets>,
) {
    let Ok((player_entity, player_tf, mut feet, children)) = player.single_mut() else { return };

    feet.duration.tick(time.delta());
    feet.step_timer.tick(time.delta());

    if feet.duration.is_finished() {
        commands.entity(player_entity).remove::<BloodyFeet>();
        return;
    }

    if feet.step_timer.just_finished() {
        // Угол направления из модели
        let mut facing = 0.0;
        for child in children.iter() {
            if let Ok(model_tf) = model_query.get(child) {
                let forward = model_tf.rotation * Vec3::Z;
                facing = forward.x.atan2(forward.z);
                break;
            }
        }

        spawn_blood_footprint(
            &mut commands, &blood_assets,
            player_tf.translation, feet.color, facing,
        );
    }
}
