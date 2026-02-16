use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::modules::input::data::input_state::InputState;

pub fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut input_state: ResMut<InputState>,
) {
    let mut movement = Vec3::ZERO;

    // WASD движение
    if keyboard.pressed(KeyCode::KeyW) {
        movement.z -= 1.0;  // Вперед от камеры (отрицательный Z)
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement.z += 1.0;  // Назад к камере (положительный Z)
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }

    // Флаг для WC3/Dota override: WASD > auto-play
    input_state.has_keyboard_input = movement.length() > 0.0;

    // Клавиатура пишет movement ТОЛЬКО если есть нажатые клавиши
    // или touch не активен (чтобы не перезаписать touch drag нулём)
    // Auto-play перезапишет в свою очередь (запускается .after())
    if movement.length() > 0.0 {
        input_state.movement = movement.normalize();
        input_state.is_running = keyboard.pressed(KeyCode::ShiftLeft)
            || keyboard.pressed(KeyCode::ShiftRight);
    } else if !input_state.is_touch_active {
        input_state.movement = Vec3::ZERO;
        input_state.is_running = false;
    }

    // Mouse wheel для зума камеры (zoom_delta сбрасывается в camera_zoom_system)
    for event in mouse_wheel.read() {
        input_state.zoom_delta += event.y;
    }
}
