use bevy::prelude::*;
use crate::shared::GameState;
use crate::modules::menu::components::*;
use crate::modules::menu::parts::fade_transition::FadeState;
use crate::modules::combat::parts::game_over::KillCount;
use crate::modules::combat::parts::game_timer::GameTimer;
use crate::modules::enemies::components::WaveState;
use crate::modules::progression::components::{PlayerXp, UpgradeInventory, UpgradeCategory};
use crate::modules::progression::parts::upgrades;
use crate::toolkit::asset_paths;

/// Создаёт Game Over оверлей — кровавая виньетка поверх замёрзшей сцены
pub fn setup_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    kill_count: Res<KillCount>,
    wave_state: Res<WaveState>,
    game_timer: Res<GameTimer>,
    player_xp: Res<PlayerXp>,
    inventory: Res<UpgradeInventory>,
) {
    let font_title = asset_server.load(asset_paths::FONT_TITLE);
    let font_ui = asset_server.load(asset_paths::FONT_UI);
    let font_ui_bold: Handle<Font> = asset_server.load(asset_paths::FONT_UI_BOLD);

    let bg_image: Handle<Image> = asset_server.load(asset_paths::GAMEOVER_BG);

    let wave = wave_state.current_wave;
    let kills = kill_count.total;
    let time_str = game_timer.formatted();

    // Root — контейнер для Game Over контента
    commands.spawn((
        GameOverUI,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(12.0),
            ..default()
        },
    )).with_children(|parent| {
        // Фоновое изображение (absolute, fullscreen)
        parent.spawn((
            GameOverUI,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ImageNode::new(bg_image),
        ));

        // Кровавая виньетка поверх фона
        parent.spawn((
            GameOverUI,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundGradient::from(RadialGradient {
                stops: vec![
                    ColorStop::new(Color::srgba(0.0, 0.0, 0.0, 0.6), Val::Percent(0.0)),
                    ColorStop::new(Color::srgba(0.12, 0.02, 0.02, 0.4), Val::Percent(50.0)),
                    ColorStop::new(Color::srgba(0.25, 0.03, 0.02, 0.7), Val::Percent(100.0)),
                ],
                ..default()
            }),
        ));

        // "ВЫ ПАЛИ В БОЮ"
        parent.spawn((
            GameOverUI,
            Text::new("ВЫ ПАЛИ В БОЮ"),
            TextFont {
                font: font_title,
                font_size: 64.0,
                ..default()
            },
            TextColor(Color::srgb(0.85, 0.12, 0.08)),
            TextShadow {
                offset: Vec2::new(4.0, 4.0),
                color: Color::srgba(0.0, 0.0, 0.0, 0.8),
            },
        ));

        // Золотой разделитель
        spawn_separator(parent);

        // Волна + Время — одна строка
        parent.spawn((
            GameOverUI,
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(32.0),
                ..default()
            },
        )).with_children(|row| {
            row.spawn((
                GameOverUI,
                Text::new(format!("Волна: {}", wave)),
                TextFont { font: font_ui.clone(), font_size: 26.0, ..default() },
                TextColor(Color::srgb(0.95, 0.7, 0.2)),
                TextShadow { offset: Vec2::new(1.0, 1.0), color: Color::srgba(0.0, 0.0, 0.0, 0.7) },
            ));
            row.spawn((
                GameOverUI,
                Text::new(format!("Время: {}", time_str)),
                TextFont { font: font_ui.clone(), font_size: 26.0, ..default() },
                TextColor(Color::srgb(0.8, 0.75, 0.65)),
                TextShadow { offset: Vec2::new(1.0, 1.0), color: Color::srgba(0.0, 0.0, 0.0, 0.7) },
            ));
        });

        // Разделитель
        spawn_separator(parent);

        // Убито врагов: N
        parent.spawn((
            GameOverUI,
            Text::new(format!("Убито врагов: {}", kills)),
            TextFont { font: font_ui.clone(), font_size: 26.0, ..default() },
            TextColor(Color::srgb(0.8, 0.75, 0.65)),
            TextShadow { offset: Vec2::new(1.0, 1.0), color: Color::srgba(0.0, 0.0, 0.0, 0.7) },
        ));

        // Per-type breakdown (только если были убийства)
        if kills > 0 {
            parent.spawn((
                GameOverUI,
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    ..default()
                },
            )).with_children(|row| {
                let stats = [
                    ("Упыри", kill_count.upyr),
                    ("Лешие", kill_count.leshiy),
                    ("Волколаки", kill_count.volkolak),
                ];
                for (name, count) in stats {
                    if count > 0 {
                        row.spawn((
                            GameOverUI,
                            Text::new(format!("{}: {}", name, count)),
                            TextFont { font: font_ui.clone(), font_size: 20.0, ..default() },
                            TextColor(Color::srgb(0.6, 0.55, 0.5)),
                            TextShadow { offset: Vec2::new(1.0, 1.0), color: Color::srgba(0.0, 0.0, 0.0, 0.7) },
                        ));
                    }
                }
            });
        }

        // Разделитель
        spawn_separator(parent);

        // Уровень
        parent.spawn((
            GameOverUI,
            Text::new(format!("Уровень: {}", player_xp.level)),
            TextFont { font: font_ui, font_size: 26.0, ..default() },
            TextColor(Color::srgb(0.95, 0.7, 0.2)),
            TextShadow { offset: Vec2::new(1.0, 1.0), color: Color::srgba(0.0, 0.0, 0.0, 0.7) },
        ));

        // Иконки апгрейдов (если есть)
        if !inventory.upgrades.is_empty() {
            parent.spawn((
                GameOverUI,
                Node {
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: Val::Px(4.0),
                    row_gap: Val::Px(4.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            )).with_children(|row| {
                for &(upgrade_id, level) in &inventory.upgrades {
                    let Some(def) = upgrades::get_upgrade_def(&upgrade_id) else { continue };
                    let category_color = match def.category {
                        UpgradeCategory::Attack => Color::srgb(0.9, 0.3, 0.2),
                        UpgradeCategory::Defense => Color::srgb(0.3, 0.6, 0.9),
                        UpgradeCategory::Path => Color::srgb(0.3, 0.9, 0.4),
                    };
                    row.spawn((
                        GameOverUI,
                        Node {
                            width: Val::Px(28.0),
                            height: Val::Px(28.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.0)),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            ..default()
                        },
                        BackgroundColor(category_color.with_alpha(0.7)),
                        BorderColor::all(category_color),
                    )).with_children(|icon| {
                        icon.spawn((
                            GameOverUI,
                            Text::new(format!("{}", level)),
                            TextFont { font: font_ui_bold.clone(), font_size: 14.0, ..default() },
                            TextColor(Color::WHITE),
                            TextShadow { offset: Vec2::new(1.0, 1.0), color: Color::srgba(0.0, 0.0, 0.0, 0.9) },
                        ));
                    });
                }
            });
        }

        // Кнопки
        let btn_font = font_ui_bold.clone();
        parent.spawn((
            GameOverUI,
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(24.0),
                margin: UiRect::top(Val::Px(24.0)),
                ..default()
            },
        )).with_children(|row| {
            spawn_button(row, "ЗАНОВО", RestartButton, btn_font.clone(),
                Color::srgb(0.95, 0.75, 0.3),
                Color::srgba(0.95, 0.7, 0.2, 0.8),
            );
            spawn_button(row, "В МЕНЮ", MenuButton, btn_font,
                Color::srgb(0.7, 0.65, 0.55),
                Color::srgba(0.6, 0.55, 0.45, 0.5),
            );
        });
    });

    info!("Game Over screen: wave {}, kills {} (upyr {}, leshiy {}, volkolak {}), level {}, time {}",
        wave, kills, kill_count.upyr, kill_count.leshiy, kill_count.volkolak, player_xp.level, time_str);
}

/// Золотой разделитель (transparent → gold → transparent)
fn spawn_separator(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        GameOverUI,
        Node {
            width: Val::Px(220.0),
            height: Val::Px(2.0),
            margin: UiRect::vertical(Val::Px(6.0)),
            ..default()
        },
        BackgroundGradient::from(LinearGradient {
            angle: 90_f32.to_radians(),
            stops: vec![
                Color::srgba(0.95, 0.7, 0.2, 0.0).into(),
                Color::srgba(0.95, 0.7, 0.2, 0.7).into(),
                Color::srgba(0.95, 0.7, 0.2, 0.0).into(),
            ],
            ..default()
        }),
    ));
}

fn spawn_button(
    parent: &mut ChildSpawnerCommands,
    text: &str,
    marker: impl Component,
    font: Handle<Font>,
    text_color: Color,
    border_color: Color,
) {
    parent.spawn((
        GameOverUI,
        marker,
        Node {
            padding: UiRect::axes(Val::Px(36.0), Val::Px(14.0)),
            border: UiRect::all(Val::Px(1.5)),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.95, 0.7, 0.2, 0.06)),
        BorderGradient::from(LinearGradient {
            angle: 90_f32.to_radians(),
            stops: vec![
                border_color.into(),
                border_color.with_alpha(border_color.alpha() * 0.3).into(),
            ],
            ..default()
        }),
        BoxShadow(vec![ShadowStyle {
            color: Color::srgba(0.95, 0.7, 0.2, 0.15),
            x_offset: Val::Px(0.0),
            y_offset: Val::Px(0.0),
            spread_radius: Val::Px(2.0),
            blur_radius: Val::Px(15.0),
        }]),
        Button,
    )).with_children(|btn: &mut ChildSpawnerCommands| {
        btn.spawn((
            GameOverUI,
            Text::new(text),
            TextFont {
                font,
                font_size: 22.0,
                ..default()
            },
            TextColor(text_color),
            TextShadow {
                offset: Vec2::new(1.0, 1.0),
                color: Color::srgba(0.0, 0.0, 0.0, 0.6),
            },
        ));
    });
}

/// Обработка кнопок Game Over (через fade)
pub fn game_over_interaction(
    restart_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    menu_query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
    mut fade: ResMut<FadeState>,
) {
    if fade.is_active() {
        return;
    }

    for interaction in &restart_query {
        if *interaction == Interaction::Pressed {
            fade.start_fade(GameState::Loading, true);
        }
    }
    for interaction in &menu_query {
        if *interaction == Interaction::Pressed {
            fade.start_fade(GameState::TitleScreen, true);
        }
    }
}

/// Удаляет Game Over UI
pub fn cleanup_game_over(
    mut commands: Commands,
    query: Query<Entity, (With<GameOverUI>, Without<ChildOf>)>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
