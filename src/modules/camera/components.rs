use bevy::prelude::*;
use crate::shared::constants::CAMERA_ZOOM_DEFAULT;

#[derive(Component)]
pub struct CameraTarget;

/// Resource для хранения текущего расстояния зума камеры
#[derive(Resource)]
pub struct CameraZoom {
    pub current_distance: f32,  // Текущее расстояние
    pub target_distance: f32,   // Целевое расстояние
}

impl Default for CameraZoom {
    fn default() -> Self {
        Self {
            current_distance: CAMERA_ZOOM_DEFAULT,
            target_distance: CAMERA_ZOOM_DEFAULT,
        }
    }
}
