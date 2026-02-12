use bevy::prelude::*;
use crate::shared::GameState;

/// Маркер для fullscreen fade overlay
#[derive(Component)]
pub struct FadeOverlay;

/// Направление fade-анимации
#[derive(Debug, Clone, PartialEq)]
pub enum FadeDirection {
    /// Затемнение (alpha 0→1)
    Out,
    /// Проявление (alpha 1→0)
    In,
    /// Не активен
    Idle,
}

/// Состояние fade-перехода между экранами
#[derive(Resource)]
pub struct FadeState {
    pub timer: Timer,
    pub direction: FadeDirection,
    pub target_state: Option<GameState>,
    /// Нужно ли разпаузить время при переходе
    pub unpause_on_transition: bool,
}

impl Default for FadeState {
    fn default() -> Self {
        Self {
            // Начинаем с fade-in (экран проявляется из тёмного)
            timer: Timer::from_seconds(0.2, TimerMode::Once),
            direction: FadeDirection::In,
            target_state: None,
            unpause_on_transition: false,
        }
    }
}

impl FadeState {
    /// Запустить fade-out → смена стейта → fade-in
    pub fn start_fade(&mut self, target: GameState, unpause: bool) {
        self.direction = FadeDirection::Out;
        self.target_state = Some(target);
        self.unpause_on_transition = unpause;
        self.timer = Timer::from_seconds(0.1, TimerMode::Once);
    }

    /// Активен ли fade (блокировать ввод)
    pub fn is_active(&self) -> bool {
        self.direction != FadeDirection::Idle
    }
}

/// Спавнит fullscreen overlay при старте (начинает с alpha=1, тёмный экран)
pub fn spawn_fade_overlay(mut commands: Commands) {
    commands.spawn((
        FadeOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        // Тёмно-кровавый цвет fade (не чисто чёрный)
        BackgroundColor(Color::srgba(0.05, 0.02, 0.02, 1.0)),
        GlobalZIndex(999),
    ));
}

/// Анимирует fade overlay каждый кадр
pub fn animate_fade(
    real_time: Res<Time<Real>>,
    mut fade: ResMut<FadeState>,
    mut overlay: Query<&mut BackgroundColor, With<FadeOverlay>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    if fade.direction == FadeDirection::Idle {
        return;
    }

    // Используем Real time чтобы fade работал даже когда Virtual time на паузе
    fade.timer.tick(real_time.delta());
    let progress = fade.timer.fraction();

    let alpha = match fade.direction {
        FadeDirection::Out => progress,       // 0→1 (затемнение)
        FadeDirection::In => 1.0 - progress,  // 1→0 (проявление)
        FadeDirection::Idle => 0.0,
    };

    for mut bg in &mut overlay {
        bg.0 = Color::srgba(0.05, 0.02, 0.02, alpha);
    }

    if fade.timer.is_finished() {
        match fade.direction {
            FadeDirection::Out => {
                // Fade-out завершён → сменить стейт, начать fade-in
                if let Some(target) = fade.target_state.take() {
                    if fade.unpause_on_transition {
                        virtual_time.unpause();
                        fade.unpause_on_transition = false;
                    }
                    next_state.set(target);
                }
                fade.direction = FadeDirection::In;
                fade.timer = Timer::from_seconds(0.1, TimerMode::Once);
            }
            FadeDirection::In => {
                // Fade-in завершён → сделать overlay невидимым
                fade.direction = FadeDirection::Idle;
                for mut bg in &mut overlay {
                    bg.0 = Color::srgba(0.05, 0.02, 0.02, 0.0);
                }
            }
            FadeDirection::Idle => {}
        }
    }
}
