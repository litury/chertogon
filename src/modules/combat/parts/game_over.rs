use bevy::prelude::*;
use crate::shared::GameState;
use crate::modules::player::components::Player;
use crate::modules::combat::components::PlayerHealth;
use crate::modules::menu::parts::fade_transition::FadeState;
use super::camera_shake::CameraShake;

/// Счётчик убийств за текущий раунд
#[derive(Resource, Default)]
pub struct KillCount {
    pub total: u32,
}

/// Проверяет смерть игрока → fade-переход в GameOver
pub fn check_game_over_system(
    player: Query<&PlayerHealth, With<Player>>,
    mut fade: ResMut<FadeState>,
    mut time: ResMut<Time<Virtual>>,
) {
    if fade.is_active() {
        return;
    }

    if let Ok(health) = player.single() {
        if health.is_dead() {
            info!("Game Over: player has died");
            time.pause();
            fade.start_fade(GameState::GameOver, false);
        }
    }
}

/// Сброс при входе в Playing: unpause время + reset CameraShake
pub fn reset_on_enter(
    mut time: ResMut<Time<Virtual>>,
    mut shake: ResMut<CameraShake>,
) {
    time.unpause();
    *shake = CameraShake::default();
}
