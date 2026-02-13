use bevy::prelude::*;
use bevy::render::view::ColorGrading;
use crate::modules::selection::components::PortraitCamera;

/// Ресурс: красный сдвиг экрана при получении урона игроком
/// Паттерн аналогичен CameraShake: trigger → decay
#[derive(Resource)]
pub struct DamageVignette {
    pub intensity: f32,
    pub duration: f32,
    pub elapsed: f32,
}

impl Default for DamageVignette {
    fn default() -> Self {
        Self {
            intensity: 0.0,
            duration: 0.0,
            elapsed: 0.0,
        }
    }
}

impl DamageVignette {
    pub fn trigger(&mut self, intensity: f32, duration: f32) {
        self.intensity = intensity;
        self.duration = duration;
        self.elapsed = 0.0;
    }
}

/// Затухание виньетки
pub fn damage_vignette_decay_system(
    time: Res<Time>,
    mut vignette: ResMut<DamageVignette>,
) {
    if vignette.intensity > 0.0 {
        vignette.elapsed += time.delta_secs();
        if vignette.elapsed >= vignette.duration {
            vignette.intensity = 0.0;
        }
    }
}

/// Применение эффекта: красный сдвиг ColorGrading камеры
/// Exposure темнее, температура теплее (красный), тени насыщеннее
pub fn damage_vignette_apply_system(
    vignette: Res<DamageVignette>,
    mut camera_query: Query<&mut ColorGrading, (With<Camera3d>, Without<PortraitCamera>)>,
) {
    let Ok(mut grading) = camera_query.single_mut() else { return };

    if vignette.intensity <= 0.0 || vignette.elapsed >= vignette.duration {
        // Базовые значения (из Phase 1 setup_camera)
        grading.global.exposure = 0.0;
        grading.global.temperature = -0.05;
        grading.shadows.saturation = 0.9;
        return;
    }

    let progress = vignette.elapsed / vignette.duration;
    let strength = vignette.intensity * (1.0 - progress);

    // Красный сдвиг: теплее, темнее, насыщенные тени
    grading.global.exposure = -0.3 * strength;
    grading.global.temperature = -0.05 + 0.15 * strength;
    grading.shadows.saturation = 0.9 + 0.5 * strength;
}

/// Сброс ColorGrading при смерти — чтобы не застревал красный/тёмный экран
pub fn reset_color_grading(
    mut camera_query: Query<&mut ColorGrading, (With<Camera3d>, Without<PortraitCamera>)>,
) {
    let Ok(mut grading) = camera_query.single_mut() else { return };
    grading.global.exposure = -0.15;
    grading.global.temperature = -0.05;
    grading.shadows.saturation = 0.9;
}
