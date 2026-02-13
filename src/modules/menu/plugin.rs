use bevy::prelude::*;
use crate::shared::GameState;
use super::parts::{title_screen, game_over_screen, hud, fps_counter, button_hover, fade_transition, font_diagnostics};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Диагностика загрузки шрифтов
            .add_systems(Startup, font_diagnostics::load_fonts)
            .add_systems(Update, font_diagnostics::check_font_loading)
            // Fade transition system (глобальный, работает во всех стейтах)
            .init_resource::<fade_transition::FadeState>()
            .add_systems(Startup, fade_transition::spawn_fade_overlay)
            .add_systems(Update, fade_transition::animate_fade)
            // Title Screen
            .add_systems(OnEnter(GameState::TitleScreen), title_screen::setup_title_screen)
            .add_systems(Update, (
                title_screen::title_screen_interaction,
                title_screen::pulsing_text_system,
                title_screen::remove_loading_overlay,
            ).run_if(in_state(GameState::TitleScreen)))
            .add_systems(OnExit(GameState::TitleScreen), title_screen::cleanup_title_screen)
            // HUD
            .add_systems(OnEnter(GameState::Playing), (hud::setup_hud, fps_counter::setup_fps))
            .add_systems(Update, (hud::update_hud, hud::update_timer_text, fps_counter::update_fps).run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), (hud::cleanup_hud, fps_counter::cleanup_fps))
            // Game Over
            .add_systems(OnEnter(GameState::GameOver), game_over_screen::setup_game_over)
            .add_systems(Update, game_over_screen::game_over_interaction
                .run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), game_over_screen::cleanup_game_over)
            // Button hover — работает на всех экранах с кнопками
            .add_systems(Update, button_hover::button_hover_system);

        info!("MenuPlugin loaded (title + HUD + game over)");
    }
}
