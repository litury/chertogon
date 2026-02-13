use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::ui::UiScale;

/// Масштабирует UI пропорционально меньшей стороне окна (базис 720px = 1.0)
pub fn adaptive_ui_scale_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut ui_scale: ResMut<UiScale>,
) {
    let Ok(window) = windows.single() else { return };
    let min_dim = window.width().min(window.height());
    let scale = (min_dim / 720.0).clamp(0.5, 1.5);
    if (ui_scale.0 - scale).abs() > 0.01 {
        ui_scale.0 = scale;
    }
}
