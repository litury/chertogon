use bevy::prelude::*;

/// Маркер для факелов с мерцающим огнём
#[derive(Component)]
pub struct TorchFlicker {
    pub base_intensity: f32,
    pub flicker_amount: f32,
    pub speed: f32,
    pub phase: f32,
}

/// Система мерцания: комбинация синусоид разной частоты = живой огонь
pub fn torch_flicker_system(
    time: Res<Time>,
    mut torches: Query<(&TorchFlicker, &mut PointLight)>,
) {
    let t = time.elapsed_secs();
    for (flicker, mut light) in &mut torches {
        let wave1 = (t * flicker.speed + flicker.phase).sin();
        let wave2 = (t * flicker.speed * 2.3 + flicker.phase * 1.7).sin();
        let combined = wave1 * 0.6 + wave2 * 0.4;
        light.intensity = flicker.base_intensity + combined * flicker.flicker_amount;
    }
}
