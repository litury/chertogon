use bevy::prelude::*;
use crate::modules::menu::components::*;
use crate::modules::combat::parts::game_over::KillCount;
use crate::modules::combat::parts::game_timer::GameTimer;
use crate::modules::combat::components::PlayerHealth;
use crate::modules::enemies::components::WaveState;
use crate::modules::player::components::Player;
use crate::modules::progression::components::PlayerXp;
use crate::toolkit::asset_paths;

/// Создаёт HUD: волна (лево) + таймер/убийства (право) + HP bar + XP bar
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
        // Left column: Волна + HP bar
        parent.spawn((
            HudUI,
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                row_gap: Val::Px(8.0),
                ..default()
            },
        )).with_children(|left| {
            // Волна
            left.spawn((
                HudUI,
                WaveIndicatorText,
                Text::new("Волна: 1"),
                TextFont {
                    font: font_ui_bold.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.7, 0.2)),
                TextShadow {
                    offset: Vec2::new(2.0, 2.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.85),
                },
            ));

            // HP bar container
            left.spawn((
                HudUI,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(18.0),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(3.0)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.7, 0.5, 0.2)),
                BackgroundColor(Color::srgba(0.1, 0.05, 0.05, 0.7)),
            )).with_children(|bar| {
                // HP fill
                bar.spawn((
                    HudUI,
                    HpBarFill,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border_radius: BorderRadius::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.15, 0.1)),
                ));
            });

            // HP text
            left.spawn((
                HudUI,
                HpBarText,
                Text::new("100/100"),
                TextFont {
                    font: font_ui.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.85, 0.8)),
                TextShadow {
                    offset: Vec2::new(1.0, 1.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.9),
                },
            ));
        });

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

            right.spawn((
                HudUI,
                KillCounterText,
                Text::new("Убито: 0"),
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
        });
    });

    // XP bar — нижний центр (отдельный absolute root)
    commands.spawn((
        HudUI,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(3.0),
            left: Val::Percent(25.0),
            width: Val::Percent(50.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.0),
            ..default()
        },
    )).with_children(|xp_root| {
        // Текст уровня
        xp_root.spawn((
            HudUI,
            LevelText,
            Text::new("Уровень 1"),
            TextFont {
                font: font_ui_bold,
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.4, 0.9, 0.5)),
            TextShadow {
                offset: Vec2::new(1.0, 1.0),
                color: Color::srgba(0.0, 0.0, 0.0, 0.9),
            },
        ));

        // XP bar container
        xp_root.spawn((
            HudUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(12.0),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(3.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.3, 0.6, 0.3)),
            BackgroundColor(Color::srgba(0.05, 0.1, 0.05, 0.7)),
        )).with_children(|bar| {
            // XP fill
            bar.spawn((
                HudUI,
                XpBarFill,
                Node {
                    width: Val::Percent(0.0),
                    height: Val::Percent(100.0),
                    border_radius: BorderRadius::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.8, 0.3)),
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

/// Обновляет текст таймера раз в секунду (не каждый кадр)
pub fn update_timer_text(
    game_timer: Res<GameTimer>,
    mut query: Query<&mut Text, With<TimerText>>,
    mut last_secs: Local<u32>,
) {
    let current_secs = game_timer.elapsed as u32;
    if current_secs != *last_secs {
        *last_secs = current_secs;
        for mut text in &mut query {
            **text = game_timer.formatted();
        }
    }
}

/// Обновляет HP bar (ширина fill + текст)
pub fn update_hp_bar(
    player_query: Query<&PlayerHealth, With<Player>>,
    mut hp_fill: Query<&mut Node, (With<HpBarFill>, Without<HpBarText>)>,
    mut hp_text: Query<&mut Text, (With<HpBarText>, Without<HpBarFill>)>,
) {
    let Ok(health) = player_query.single() else {
        warn!("update_hp_bar: Player entity not found!");
        return;
    };
    let fraction = (health.current / health.max).clamp(0.0, 1.0);

    for mut node in &mut hp_fill {
        node.width = Val::Percent(fraction * 100.0);
    }
    for mut text in &mut hp_text {
        **text = format!("{}/{}", health.current as i32, health.max as i32);
    }
}

/// Обновляет XP bar (ширина fill + текст уровня)
pub fn update_xp_bar(
    player_xp: Res<PlayerXp>,
    mut xp_fill: Query<&mut Node, (With<XpBarFill>, Without<LevelText>)>,
    mut level_text: Query<&mut Text, (With<LevelText>, Without<XpBarFill>)>,
) {
    let fraction = (player_xp.current_xp / player_xp.xp_to_next).clamp(0.0, 1.0);

    for mut node in &mut xp_fill {
        node.width = Val::Percent(fraction * 100.0);
    }
    for mut text in &mut level_text {
        **text = format!("Уровень {}", player_xp.level);
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
