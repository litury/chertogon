use bevy::prelude::*;
use crate::modules::enemies::components::EnemyDying;
use super::camera_shake::CameraShake;
use super::damage_vignette::DamageVignette;

/// Трекер предыдущих значений для детекции спайков
#[derive(Resource, Default)]
pub struct HapticState {
    pub last_shake: f32,
    pub last_vignette: f32,
}

/// Haptic feedback: вибрация при ударах (только WASM/Android)
pub fn haptic_feedback_system(
    mut state: ResMut<HapticState>,
    shake: Res<CameraShake>,
    vignette: Res<DamageVignette>,
    dying: Query<Entity, Added<EnemyDying>>,
) {
    // Удар по врагу (спайк camera shake)
    if shake.intensity > state.last_shake + 0.1 {
        vibrate(30);
    }
    state.last_shake = shake.intensity;

    // Получение урона (спайк vignette)
    if vignette.intensity > state.last_vignette + 0.1 {
        vibrate(100);
    }
    state.last_vignette = vignette.intensity;

    // Смерть врага
    if !dying.is_empty() {
        vibrate(50);
    }
}

/// Вызов navigator.vibrate() через web_sys (только wasm32)
#[cfg(target_arch = "wasm32")]
fn vibrate(ms: u32) {
    use wasm_bindgen::JsCast;
    let Some(window) = web_sys::window() else { return };
    let navigator = window.navigator();
    let _ = navigator.vibrate_with_duration(ms);
}

/// No-op на native платформах
#[cfg(not(target_arch = "wasm32"))]
fn vibrate(_ms: u32) {}
