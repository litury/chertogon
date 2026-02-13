use bevy::prelude::*;
use bevy::input::touch::Touches;
use crate::modules::selection::components::SelectionTapEvent;

const TAP_MAX_DURATION_SECS: f32 = 0.25;
const TAP_MAX_DISTANCE_PX: f32 = 15.0;

/// Трекер касания для определения тапа vs драга
#[derive(Resource, Default)]
pub struct TapTracker {
    pub pending_touch: Option<PendingTap>,
}

pub struct PendingTap {
    pub touch_id: u64,
    pub start_pos: Vec2,
    pub start_time: f32,
}

/// Определяет тапы (короткие нажатия) и эмитит SelectionTapEvent.
/// Мышь: любой клик левой кнопкой = тап.
/// Тач: press→release < 250мс и < 15px = тап.
pub fn detect_selection_tap(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    touches: Res<Touches>,
    mut tap_tracker: ResMut<TapTracker>,
    time: Res<Time>,
    mut tap_events: MessageWriter<SelectionTapEvent>,
) {
    // --- Мышь: клик = тап ---
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                tap_events.write(SelectionTapEvent { screen_pos: cursor_pos });
            }
        }
    }

    // --- Тач: отслеживаем press → release ---
    for touch in touches.iter_just_pressed() {
        tap_tracker.pending_touch = Some(PendingTap {
            touch_id: touch.id(),
            start_pos: touch.position(),
            start_time: time.elapsed_secs(),
        });
    }

    // Отменяем тап если палец сдвинулся слишком далеко или таймаут
    if let Some(ref pending) = tap_tracker.pending_touch {
        let mut cancel = false;

        if let Some(touch) = touches.iter().find(|t| t.id() == pending.touch_id) {
            if (touch.position() - pending.start_pos).length() > TAP_MAX_DISTANCE_PX {
                cancel = true;
            }
        }

        if time.elapsed_secs() - pending.start_time > TAP_MAX_DURATION_SECS {
            cancel = true;
        }

        if cancel {
            tap_tracker.pending_touch = None;
        }
    }

    // Проверяем отпускание — если тап ещё валиден, эмитим событие
    for touch in touches.iter_just_released() {
        if let Some(ref pending) = tap_tracker.pending_touch {
            if touch.id() == pending.touch_id {
                let distance = (touch.position() - pending.start_pos).length();
                let elapsed = time.elapsed_secs() - pending.start_time;

                if distance <= TAP_MAX_DISTANCE_PX && elapsed <= TAP_MAX_DURATION_SECS {
                    tap_events.write(SelectionTapEvent {
                        screen_pos: touch.position(),
                    });
                }

                tap_tracker.pending_touch = None;
            }
        }
    }
}
