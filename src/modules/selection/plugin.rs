use bevy::prelude::*;
use crate::shared::GameState;
use super::components::*;
use super::parts::{tap_detection, picking, panel, portrait};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectionState>()
            .init_resource::<tap_detection::TapTracker>()
            .add_message::<SelectionTapEvent>()
            // Основной цикл: детекция → пикинг → панель → портрет
            .add_systems(Update, (
                tap_detection::detect_selection_tap,
                picking::pick_enemy_at_screen_pos,
                panel::manage_selection_panel,
                panel::update_selection_panel,
                portrait::update_portrait_model,
                portrait::setup_portrait_animation,
                portrait::propagate_portrait_layers,
                picking::clear_dead_selection,
            ).chain().run_if(in_state(GameState::Playing)))
            // Инициализация портретной камеры
            .add_systems(OnEnter(GameState::Playing), portrait::setup_portrait_camera)
            // Чистка при выходе
            .add_systems(OnExit(GameState::Playing), (
                panel::cleanup_selection_panel,
                portrait::cleanup_portrait,
                reset_selection,
            ));

        info!("🎯 SelectionPlugin loaded");
    }
}

fn reset_selection(mut selection: ResMut<SelectionState>) {
    selection.selected_entity = None;
}
