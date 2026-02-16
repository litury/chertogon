use bevy::prelude::*;
use crate::shared::GameState;
use super::data::input_state::InputState;
use super::handlers::{keyboard, touch};
use super::parts::{touch_joystick, tap_ripple, auto_play};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(InputState::default())
            .insert_resource(auto_play::AutoPlayState::default())
            .add_systems(Update, (
                keyboard::handle_keyboard_input,
                touch::handle_touch_input,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(Update, (
                touch_joystick::update_touch_joystick,
                tap_ripple::spawn_tap_ripple,
                tap_ripple::animate_tap_ripple,
                auto_play::auto_play_movement
                    .after(keyboard::handle_keyboard_input)
                    .after(touch::handle_touch_input),
                auto_play::toggle_auto_play,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Playing), auto_play::spawn_auto_play_button)
            .add_systems(OnExit(GameState::Playing), auto_play::cleanup_auto_play_button);
    }
}
