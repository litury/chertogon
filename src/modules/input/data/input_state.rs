use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct InputState {
    pub movement: Vec3,
    pub is_running: bool,
    pub zoom_delta: f32,  // Mouse wheel / pinch для зума камеры

    // Touch-specific state
    pub touch_start: Option<Vec2>,       // Стартовая позиция касания для "invisible joystick"
    pub is_touch_active: bool,           // Активно ли касание в данный момент
    pub primary_touch_id: Option<u64>,   // ID пальца для движения (защита от multi-touch)
    pub pinch_distance: Option<f32>,     // Расстояние между двумя пальцами (для зума)
}
