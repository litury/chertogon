use bevy::prelude::*;
use crate::shared::GameState;
use super::components::*;
use super::parts::{tap_detection, picking, panel};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectionState>()
            .init_resource::<tap_detection::TapTracker>()
            .add_message::<SelectionTapEvent>()
            // Основной цикл: детекция → пикинг → панель
            .add_systems(Update, (
                tap_detection::detect_selection_tap,
                picking::pick_character_at_screen_pos,
                panel::manage_selection_panel,
                panel::update_selection_panel,
                picking::clear_dead_selection,
            ).chain().run_if(in_state(GameState::Playing)))
            // Чистка при выходе
            .add_systems(OnExit(GameState::Playing), (
                panel::cleanup_selection_panel,
                reset_selection,
            ));

        info!("🎯 SelectionPlugin loaded");
    }
}

fn reset_selection(mut selection: ResMut<SelectionState>) {
    selection.selected_entity = None;
}
