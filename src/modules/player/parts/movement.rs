use bevy::prelude::*;
use avian3d::prelude::*;
use crate::modules::{Player, PlayerModel, InputState};
use crate::modules::player::components::PlayerStats;
use crate::shared::constants::{WALK_SPEED, RUN_SPEED};

/// Движение через LinearVelocity с учётом модификатора скорости от апгрейдов
pub fn player_movement_system(
    input_state: Res<InputState>,
    time: Res<Time>,
    mut player_query: Query<(&Children, &mut LinearVelocity, &mut Transform, &PlayerStats), With<Player>>,
    mut model_query: Query<&mut Transform, (With<PlayerModel>, Without<Player>)>,
) {
    if let Ok((children, mut velocity, mut player_transform, stats)) = player_query.single_mut() {
        // Страховка: сбрасываем rotation parent entity (physics body не должен вращаться)
        player_transform.rotation = Quat::IDENTITY;
        if input_state.movement.length() > 0.02 {
            let speed = if input_state.is_running {
                RUN_SPEED
            } else {
                WALK_SPEED
            } * stats.move_speed_multiplier;

            velocity.0 = input_state.movement.normalize() * speed;

            // Поворачиваем ТОЛЬКО визуальную модель (PlayerModel child)
            let target_rotation = Quat::from_rotation_y(
                input_state.movement.x.atan2(input_state.movement.z)
            );

            // Frame-rate independent slerp: экспоненциальное сглаживание
            let t = 1.0 - (-10.0 * time.delta_secs()).exp();

            for &child in children {
                if let Ok(mut model_transform) = model_query.get_mut(child) {
                    model_transform.rotation = model_transform.rotation.slerp(target_rotation, t);
                }
            }
        } else {
            velocity.0 = Vec3::ZERO;
        }
    }
}
