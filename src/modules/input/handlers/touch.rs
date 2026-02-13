use bevy::prelude::*;
use bevy::input::touch::Touches;
use crate::modules::input::data::input_state::InputState;

// Константы для Touch & Drag control
const DEAD_ZONE: f32 = 5.0;        // Минимальное движение для регистрации (pixels)
const WALK_THRESHOLD: f32 = 30.0;  // До этого расстояния - медленная ходьба
const RUN_THRESHOLD: f32 = 80.0;   // После этого расстояния - бег

/// Система обработки touch input по методу "Touch & Drag Anywhere"
///
/// Как работает:
/// 1. Touch anywhere = захват управления
/// 2. Drag в направлении = движение
/// 3. Drag distance определяет скорость (walk vs run)
/// 4. Release = остановка
pub fn handle_touch_input(
    touches: Res<Touches>,
    mut input_state: ResMut<InputState>,
) {
    // Проверяем есть ли активные touch events
    if let Some(touch) = touches.iter_just_pressed().next() {
        // Started - начало касания
        input_state.touch_start = Some(touch.position());
        input_state.is_touch_active = true;
        debug!("👆 Touch started at {:?}", touch.position());
    }

    // Обрабатываем активные касания (движение)
    if let Some(touch) = touches.iter().next() {
        if let Some(start_pos) = input_state.touch_start {
            let current_pos = touch.position();
            let delta = current_pos - start_pos;
            let distance = delta.length();

            // Dead zone - игнорируем очень маленькие движения
            if distance > DEAD_ZONE {
                // Нормализуем направление для движения
                let direction = delta.normalize();

                // Конвертируем screen coordinates → world movement
                // X остается X, Y на экране = -Z в мире (инвертируем)
                input_state.movement = Vec3::new(
                    direction.x,
                    0.0,
                    -direction.y  // Инвертируем Y → Z
                );

                // Определяем скорость по расстоянию от стартовой точки
                if distance > RUN_THRESHOLD {
                    // Дальше 80px - БЕГ
                    input_state.is_running = true;

                    // Опционально: haptic feedback при переходе в бег
                    #[cfg(any(target_os = "ios", target_os = "android"))]
                    {
                        // TODO: Add haptic feedback через bevy_haptic если нужно
                        // haptic_feedback(HapticType::Light);
                    }
                } else if distance > WALK_THRESHOLD {
                    // 30-80px - обычная ходьба
                    input_state.is_running = false;
                } else {
                    // Меньше 30px - медленная ходьба
                    input_state.is_running = false;
                }
            }
        }
    }

    // Проверяем отпускание касания
    if !touches.iter_just_released().next().is_none() || !touches.iter_just_canceled().next().is_none() {
        // Отпустили палец - остановка
        input_state.movement = Vec3::ZERO;
        input_state.is_running = false;
        input_state.is_touch_active = false;
        input_state.touch_start = None;
        debug!("🛑 Touch ended - stopping movement");
    }

    // Если нет активных касаний - убеждаемся что состояние сброшено
    if touches.iter().count() == 0 && input_state.is_touch_active {
        input_state.movement = Vec3::ZERO;
        // Не сбрасываем is_running - пусть клавиатура управляет им
        input_state.is_touch_active = false;
        input_state.touch_start = None;
    }
}
