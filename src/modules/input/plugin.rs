use bevy::prelude::*;
use crate::shared::GameState;
use super::data::input_state::InputState;
use super::handlers::{keyboard, touch};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(InputState::default())
            .add_systems(Update, (
                keyboard::handle_keyboard_input,
                touch::handle_touch_input,
            ).run_if(in_state(GameState::Playing)));
    }
}
