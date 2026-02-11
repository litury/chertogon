use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct InputState {
    pub movement: Vec3,
    pub is_running: bool,
    pub zoom_delta: f32,  // Mouse wheel для зума камеры

    // Touch-specific state
    pub touch_start: Option<Vec2>,  // Стартовая позиция касания для "invisible joystick"
    pub is_touch_active: bool,      // Активно ли касание в данный момент
}
