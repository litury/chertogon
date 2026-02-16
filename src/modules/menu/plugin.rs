use bevy::prelude::*;
use crate::shared::GameState;
use crate::modules::enemies::EnemyCoreSet;
use super::parts::{title_screen, game_over_screen, hud, fps_counter, button_hover, fade_transition, font_diagnostics, adaptive_scale, loading_screen, upgrade_bar, kill_feed, wave_banner, minimap};
use super::components;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Kill feed message type
            .add_message::<components::KillFeedMessage>()
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
            // Loading Screen
            .add_systems(OnEnter(GameState::Loading), loading_screen::setup_loading_screen)
            .add_systems(Update, loading_screen::update_loading_progress
                .run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), loading_screen::cleanup_loading_screen)
            // HUD
            .add_systems(OnEnter(GameState::Playing), (
                hud::setup_hud,
                fps_counter::setup_fps,
                minimap::setup_minimap,
                kill_feed::setup_kill_feed,
            ))
            .add_systems(Update, (
                hud::update_hud,
                hud::update_timer_text,
                hud::update_hp_bar,
                hud::update_xp_bar,
                fps_counter::update_fps,
                upgrade_bar::update_upgrade_bar,
                minimap::update_minimap,
                kill_feed::consume_kill_feed_messages,
                kill_feed::update_kill_feed,
                wave_banner::spawn_wave_banner,
                wave_banner::animate_wave_banner,
            ).after(EnemyCoreSet).run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), (hud::cleanup_hud, fps_counter::cleanup_fps, minimap::cleanup_minimap))
            // Game Over
            .add_systems(OnEnter(GameState::GameOver), game_over_screen::setup_game_over)
            .add_systems(Update, game_over_screen::game_over_interaction
                .run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), game_over_screen::cleanup_game_over)
            // Button hover — работает на всех экранах с кнопками
            .add_systems(Update, button_hover::button_hover_system)
            // Адаптивный UI масштаб (mobile/desktop)
            .add_systems(Update, adaptive_scale::adaptive_ui_scale_system);

        info!("MenuPlugin loaded (title + HUD + game over)");
    }
}
