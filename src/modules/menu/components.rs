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

/// Маркер для заполненной части HP бара
#[derive(Component)]
pub struct HpBarFill;

/// Маркер для текста HP значения
#[derive(Component)]
pub struct HpBarText;

/// Маркер для заполненной части XP бара
#[derive(Component)]
pub struct XpBarFill;

/// Маркер для текста уровня
#[derive(Component)]
pub struct LevelText;

/// Контейнер для иконок апгрейдов (flex row под HP баром)
#[derive(Component)]
pub struct UpgradeBarContainer;

/// Одна иконка апгрейда в баре
#[derive(Component)]
pub struct UpgradeIcon;

/// Маркер индикатора на краю экрана (пул из 8 штук)
#[derive(Component)]
pub struct EdgeIndicator {
    pub index: u8,
}

/// Контейнер для kill feed записей (правый край экрана)
#[derive(Component)]
pub struct KillFeedContainer;

/// Одна запись kill feed с таймером fade-out, группировкой и slide-in
#[derive(Component)]
pub struct KillFeedEntry {
    pub timer: Timer,
    pub group_key: Option<String>,
    pub count: u32,
    pub base_color: Color,
    pub slide_timer: f32,
}

/// Центральный баннер волны со fade-анимацией
#[derive(Component)]
pub struct WaveBanner {
    pub timer: Timer,
}

/// Сообщение для kill feed (от разных систем: убийства, волны, апгрейды)
#[derive(Message)]
pub struct KillFeedMessage {
    pub text: String,
    pub color: Color,
    pub group_key: Option<String>,
}
