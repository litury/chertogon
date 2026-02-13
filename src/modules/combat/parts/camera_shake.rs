use bevy::prelude::*;

/// Ресурс направленной тряски камеры при ударе.
/// Толчок в направлении удара → упругое затухание с осцилляцией.
#[derive(Resource)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub elapsed: f32,
    /// Направление удара (XZ плоскость, нормализовано)
    pub direction: Vec3,
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            intensity: 0.0,
            duration: 0.0,
            elapsed: 0.0,
            direction: Vec3::ZERO,
        }
    }
}

impl CameraShake {
    pub fn trigger(&mut self, intensity: f32, duration: f32, direction: Vec3) {
        self.intensity = intensity;
        self.duration = duration;
        self.elapsed = 0.0;
        self.direction = Vec3::new(direction.x, 0.0, direction.z).normalize_or_zero();
    }

    /// Направленное смещение камеры: импульс в сторону удара → упругий возврат
    pub fn offset(&self) -> Vec3 {
        if self.intensity <= 0.0 || self.elapsed >= self.duration {
            return Vec3::ZERO;
        }
        let progress = self.elapsed / self.duration;

        // Экспоненциальное затухание × косинусная осцилляция (≈3 колебания)
        let decay = (-4.0 * progress).exp();
        let oscillation = (progress * std::f32::consts::TAU * 2.5).cos();
        let strength = self.intensity * decay * oscillation;

        Vec3::new(
            self.direction.x * strength,
            0.0,
            self.direction.z * strength * 0.5, // меньше по глубине
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
