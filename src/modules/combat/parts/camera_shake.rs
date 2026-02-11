use bevy::prelude::*;

/// Ресурс тряски камеры при ударе
#[derive(Resource)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub elapsed: f32,
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            intensity: 0.0,
            duration: 0.0,
            elapsed: 0.0,
        }
    }
}

impl CameraShake {
    pub fn trigger(&mut self, intensity: f32, duration: f32) {
        self.intensity = intensity;
        self.duration = duration;
        self.elapsed = 0.0;
    }

    /// Текущее смещение камеры (затухающий шум)
    pub fn offset(&self, time_secs: f32) -> Vec3 {
        if self.intensity <= 0.0 || self.elapsed >= self.duration {
            return Vec3::ZERO;
        }
        let progress = self.elapsed / self.duration;
        let decay = 1.0 - progress; // линейное затухание
        let strength = self.intensity * decay;

        Vec3::new(
            (time_secs * 53.0).sin() * strength,
            (time_secs * 71.0).cos() * strength * 0.5,
            0.0,
        )
    }
}

/// Затухание тряски камеры
pub fn camera_shake_decay_system(
    time: Res<Time>,
    mut shake: ResMut<CameraShake>,
) {
    if shake.intensity > 0.0 {
        shake.elapsed += time.delta_secs();
        if shake.elapsed >= shake.duration {
            shake.intensity = 0.0;
        }
    }
}
