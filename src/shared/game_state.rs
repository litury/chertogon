use bevy::prelude::*;

/// Глобальное состояние игры (аналог роутера)
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    TitleScreen,
    Playing,
    GameOver,
}
