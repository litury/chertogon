use bevy::prelude::*;
use bevy::time::Real;

/// Ресурс микро-заморозки при попадании (Hades-style hitstop)
#[derive(Resource, Default)]
pub struct Hitstop {
    pub timer: Timer,
    pub active: bool,
}

impl Hitstop {
    pub fn trigger(&mut self, duration_secs: f32) {
        self.timer = Timer::from_seconds(duration_secs, TimerMode::Once);
        self.active = true;
    }
}

/// Система hitstop: замедляет виртуальное время при ударе.
/// Тикает по Real-time, чтобы не зависеть от собственного замедления.
pub fn hitstop_system(
    real_time: Res<Time<Real>>,
    mut hitstop: ResMut<Hitstop>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    if !hitstop.active {
        return;
    }

    virtual_time.set_relative_speed(0.01);
    hitstop.timer.tick(real_time.delta());

    if hitstop.timer.is_finished() {
        hitstop.active = false;
        virtual_time.set_relative_speed(1.0);
    }
}
