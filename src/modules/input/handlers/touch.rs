use bevy::prelude::*;
use bevy::input::touch::Touches;
use crate::modules::input::data::input_state::InputState;

// Константы для Touch & Drag control
const DEAD_ZONE: f32 = 5.0;        // Минимальное движение для регистрации (pixels)
const RUN_THRESHOLD: f32 = 80.0;   // После этого расстояния — бег
const PINCH_SENSITIVITY: f32 = 0.05; // Чувствительность pinch-to-zoom

/// Система обработки touch input: drag-to-move + pinch-to-zoom
pub fn handle_touch_input(
    touches: Res<Touches>,
    mut input_state: ResMut<InputState>,
) {
    let active_touches: Vec<_> = touches.iter().collect();
    let touch_count = active_touches.len();

    // === PINCH ZOOM (2+ пальца) ===
    if touch_count >= 2 {
        let dist = active_touches[0].position()
            .distance(active_touches[1].position());

        if let Some(prev_dist) = input_state.pinch_distance {
            let delta = dist - prev_dist;
            // Положительный delta = пальцы раздвигаются = zoom in
            input_state.zoom_delta = delta * PINCH_SENSITIVITY;
        }
        input_state.pinch_distance = Some(dist);

        // Заморозить движение во время pinch
        input_state.movement = Vec3::ZERO;
        input_state.is_running = false;
        return;
    }

    // Сброс pinch при возврате к 1 пальцу
    input_state.pinch_distance = None;

    // === SINGLE TOUCH MOVEMENT (1 палец) ===

    // Начало касания — запоминаем ID пальца
    if let Some(touch) = touches.iter_just_pressed().next() {
        input_state.primary_touch_id = Some(touch.id());
        input_state.touch_start = Some(touch.position());
        input_state.is_touch_active = true;
    }

    // Движение — ТОЛЬКО по primary finger (защита от multi-touch)
    if let Some(primary_id) = input_state.primary_touch_id {
        if let Some(touch) = touches.iter().find(|t| t.id() == primary_id) {
            if let Some(start_pos) = input_state.touch_start {
                let delta = touch.position() - start_pos;
                let distance = delta.length();

                if distance > DEAD_ZONE {
                    let direction = delta.normalize();
                    // Screen X → World X, Screen Y → World -Z (инвертируем)
                    input_state.movement = Vec3::new(direction.x, 0.0, direction.y);
                    input_state.is_running = distance > RUN_THRESHOLD;
                }
            }
        }
    }

    // Отпускание primary finger
    for touch in touches.iter_just_released() {
        if Some(touch.id()) == input_state.primary_touch_id {
            input_state.movement = Vec3::ZERO;
            input_state.is_running = false;
            input_state.is_touch_active = false;
            input_state.touch_start = None;
            input_state.primary_touch_id = None;
        }
    }

    // Отмена primary finger (палец ушёл за пределы экрана)
    for touch in touches.iter_just_canceled() {
        if Some(touch.id()) == input_state.primary_touch_id {
            input_state.movement = Vec3::ZERO;
            input_state.is_running = false;
            input_state.is_touch_active = false;
            input_state.touch_start = None;
            input_state.primary_touch_id = None;
        }
    }

    // Fallback: нет активных касаний — сброс
    if touch_count == 0 && input_state.is_touch_active {
        input_state.movement = Vec3::ZERO;
        input_state.is_touch_active = false;
        input_state.touch_start = None;
        input_state.primary_touch_id = None;
    }
}
