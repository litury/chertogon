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

    // Нормализация вектора движения
    if movement.length() > 0.0 {
        movement = movement.normalize();
    }

    input_state.movement = movement;
    input_state.is_running = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    // Mouse wheel для зума камеры
    // Положительное значение = scroll up = zoom in (приближение)
    // Отрицательное значение = scroll down = zoom out (отдаление)
    input_state.zoom_delta = 0.0;
    for event in mouse_wheel.read() {
        input_state.zoom_delta += event.y;  // y = вертикальный scroll
    }
}
