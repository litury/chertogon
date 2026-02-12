use bevy::prelude::*;
use crate::modules::menu::components::*;
use crate::modules::combat::parts::game_over::KillCount;
use crate::modules::combat::parts::game_timer::GameTimer;
use crate::modules::enemies::components::WaveState;
use crate::toolkit::asset_paths;

/// Создаёт минималистичный HUD: волна (лево) + таймер/убийства (право)
/// Адаптивный padding (Val::Percent) для мобильных экранов
pub fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_ui_bold = asset_server.load(asset_paths::FONT_UI_BOLD);
    let font_ui = asset_server.load(asset_paths::FONT_UI);

    // Root — fullscreen, safe-area padding
    commands.spawn((
        HudUI,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexStart,
            padding: UiRect::all(Val::Percent(2.5)),
            ..default()
        },
    )).with_children(|parent| {
        // Left: Волна
        parent.spawn((
            HudUI,
            WaveIndicatorText,
            Text::new("Волна: 1"),
            TextFont {
                font: font_ui_bold,
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.95, 0.7, 0.2)),
            TextShadow {
                offset: Vec2::new(2.0, 2.0),
                color: Color::srgba(0.0, 0.0, 0.0, 0.85),
            },
        ));

        // Right column: Таймер + Убийства
        parent.spawn((
            HudUI,
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                row_gap: Val::Px(4.0),
                ..default()
            },
        )).with_children(|right| {
            // Таймер
            right.spawn((
                HudUI,
                TimerText,
                Text::new("00:00"),
                TextFont {
                    font: font_ui.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.75, 0.65)),
                TextShadow {
                    offset: Vec2::new(2.0, 2.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.85),
                },
            ));

            // Убийства
            right.spawn((
                HudUI,
                KillCounterText,
                Text::new("Убито: 0"),
                TextFont {
                    font: font_ui,
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.75, 0.65)),
                TextShadow {
                    offset: Vec2::new(2.0, 2.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.85),
                },
            ));
        });
    });
}

/// Обновляет тексты HUD при изменении ресурсов
pub fn update_hud(
    kill_count: Res<KillCount>,
    wave_state: Res<WaveState>,
    mut kill_text: Query<&mut Text, (With<KillCounterText>, Without<WaveIndicatorText>)>,
    mut wave_text: Query<&mut Text, (With<WaveIndicatorText>, Without<KillCounterText>)>,
) {
    if kill_count.is_changed() {
        for mut text in &mut kill_text {
            **text = format!("Убито: {}", kill_count.total);
        }
    }
    if wave_state.is_changed() {
        for mut text in &mut wave_text {
            **text = format!("Волна: {}", wave_state.current_wave);
        }
    }
}

/// Обновляет текст таймера каждый кадр
pub fn update_timer_text(
    game_timer: Res<GameTimer>,
    mut query: Query<&mut Text, With<TimerText>>,
) {
    for mut text in &mut query {
        **text = game_timer.formatted();
    }
}

/// Удаляет HUD
pub fn cleanup_hud(
    mut commands: Commands,
    query: Query<Entity, With<HudUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
