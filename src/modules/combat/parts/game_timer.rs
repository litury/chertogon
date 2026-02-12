use bevy::prelude::*;

/// Отслеживает время текущего забега (в секундах)
#[derive(Resource, Default)]
pub struct GameTimer {
    pub elapsed: f32,
}

impl GameTimer {
    /// Форматирует время как MM:SS
    pub fn formatted(&self) -> String {
        let total_secs = self.elapsed as u32;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }
}

pub fn tick_game_timer(time: Res<Time>, mut timer: ResMut<GameTimer>) {
    timer.elapsed += time.delta_secs();
}

pub fn reset_game_timer(mut timer: ResMut<GameTimer>) {
    timer.elapsed = 0.0;
}
