use bevy::prelude::*;
use crate::modules::{Player, InputState};
use crate::modules::camera::CameraZoom;
use crate::modules::combat::CameraShake;
use crate::shared::constants::{
    CAMERA_FOLLOW_SPEED, CAMERA_OFFSET_Y,
    CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX, CAMERA_ZOOM_SPEED
};

/// Система обработки зума камеры (mouse wheel)
pub fn camera_zoom_system(
    input_state: Res<InputState>,
    mut camera_zoom: ResMut<CameraZoom>,
    _time: Res<Time>,
) {
    // Обрабатываем mouse wheel input
    if input_state.zoom_delta.abs() > 0.01 {
        // Уменьшаем расстояние при scroll up (zoom in)
        // Увеличиваем расстояние при scroll down (zoom out)
        camera_zoom.target_distance -= input_state.zoom_delta * CAMERA_ZOOM_SPEED;

        // Clamp между min и max
        camera_zoom.target_distance = camera_zoom.target_distance.clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX);
    }

    // Плавная интерполяция к целевому расстоянию (10% каждый фрейм)
    camera_zoom.current_distance = camera_zoom.current_distance.lerp(
        camera_zoom.target_distance,
        0.1
    );

    // Дополнительная защита - clamp current distance
    camera_zoom.current_distance = camera_zoom.current_distance.clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX);
}

/// ✅ ИСПРАВЛЕНО: Камера следует за Player entity (с физикой), а не за PlayerModel
pub fn follow_player_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    camera_zoom: Res<CameraZoom>,
    shake: Res<CameraShake>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let zoom_ratio = camera_zoom.current_distance / 14.0;

            // Shake offset (затухающий шум при ударе)
            let shake_offset = shake.offset(time.elapsed_secs());

            let target_position = player_transform.translation + Vec3::new(
                0.0,
                CAMERA_OFFSET_Y * zoom_ratio,
                camera_zoom.current_distance
            ) + shake_offset;

            camera_transform.translation = camera_transform.translation.lerp(target_position, CAMERA_FOLLOW_SPEED);
            camera_transform.look_at(player_transform.translation + Vec3::Y * 2.0, Vec3::Y);
        }
    }
}
