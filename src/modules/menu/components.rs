use bevy::prelude::*;

/// Маркер для всех entity Title Screen (для bulk despawn)
#[derive(Component)]
pub struct TitleScreenUI;

/// Маркер для всех HUD entity
#[derive(Component)]
pub struct HudUI;

/// Маркер для текста счётчика убийств
#[derive(Component)]
pub struct KillCounterText;

/// Маркер для текста индикатора волны
#[derive(Component)]
pub struct WaveIndicatorText;

/// Маркер для всех entity Game Over экрана
#[derive(Component)]
pub struct GameOverUI;

/// Маркер кнопки "Заново"
#[derive(Component)]
pub struct RestartButton;

/// Маркер кнопки "В меню"
#[derive(Component)]
pub struct MenuButton;

/// Маркер FPS-счётчика
#[derive(Component)]
pub struct FpsText;

/// Маркер для текста с пульсирующей прозрачностью
#[derive(Component)]
pub struct PulsingText;

/// Маркер для текста таймера в HUD
#[derive(Component)]
pub struct TimerText;

/// Маркер загрузочного overlay (скрывает 3D-сцену до готовности UI)
#[derive(Component)]
pub struct LoadingOverlay;
