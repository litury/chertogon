use bevy::prelude::*;
use crate::modules::enemies::components::{Enemy, EnemyType, Health, ChasePlayer};
use crate::modules::combat::components::EnemyAttackCooldown;
use crate::modules::selection::components::*;
use crate::toolkit::asset_paths;

/// Пересоздаёт панель выделения при изменении SelectionState.
pub fn manage_selection_panel(
    selection: Res<SelectionState>,
    enemies: Query<
        (&EnemyType, &Health, &ChasePlayer, &EnemyAttackCooldown),
        With<Enemy>,
    >,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing_panels: Query<Entity, With<SelectionPanelUI>>,
    portrait_target: Option<Res<PortraitRenderTarget>>,
) {
    if !selection.is_changed() {
        return;
    }

    // Despawn старой панели
    for entity in &existing_panels {
        commands.entity(entity).despawn();
    }

    let Some(selected) = selection.selected_entity else { return };
    let Ok((enemy_type, health, chase, attack_cd)) = enemies.get(selected) else { return };

    let font_title = asset_server.load(asset_paths::FONT_TITLE);
    let font_ui = asset_server.load(asset_paths::FONT_UI);
    let font_ui_bold = asset_server.load(asset_paths::FONT_UI_BOLD);

    let name = match enemy_type {
        EnemyType::Upyr => "Упырь",
        EnemyType::Leshiy => "Леший",
        EnemyType::Volkolak => "Волколак",
    };

    let hp_color = match enemy_type {
        EnemyType::Upyr => Color::srgb(0.8, 0.15, 0.1),
        EnemyType::Leshiy => Color::srgb(0.15, 0.7, 0.2),
        EnemyType::Volkolak => Color::srgb(0.5, 0.5, 0.65),
    };

    let hp_fraction = (health.current / health.max).clamp(0.0, 1.0);

    // Root: absolute bottom-center
    commands.spawn((
        SelectionPanelUI,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(16.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-170.0)), // center: half of width
            width: Val::Px(340.0),
            padding: UiRect::all(Val::Px(10.0)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            border: UiRect::all(Val::Px(1.5)),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.06, 0.04, 0.1, 0.92)),
        BorderColor::all(Color::srgba(0.95, 0.7, 0.2, 0.5)),
        BoxShadow(vec![ShadowStyle {
            color: Color::srgba(0.95, 0.7, 0.2, 0.12),
            x_offset: Val::Px(0.0),
            y_offset: Val::Px(0.0),
            spread_radius: Val::Px(2.0),
            blur_radius: Val::Px(16.0),
        }]),
        GlobalZIndex(100),
    )).with_children(|parent| {
        // Портрет (render-to-texture или placeholder)
        let portrait_node = (
            SelectionPanelUI,
            SelectionPortrait,
            Node {
                width: Val::Px(80.0),
                height: Val::Px(80.0),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            BorderColor::all(Color::srgba(0.95, 0.7, 0.2, 0.4)),
        );

        if let Some(ref target) = portrait_target {
            parent.spawn((
                portrait_node,
                ImageNode::new(target.0.clone()),
            ));
        } else {
            parent.spawn((
                portrait_node,
                BackgroundColor(Color::srgba(0.15, 0.1, 0.2, 0.8)),
            ));
        }

        // Info column
        parent.spawn((
            SelectionPanelUI,
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                flex_grow: 1.0,
                ..default()
            },
        )).with_children(|info| {
            // Имя врага
            info.spawn((
                SelectionPanelUI,
                Text::new(name),
                TextFont {
                    font: font_title,
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.7, 0.2)),
                TextShadow {
                    offset: Vec2::new(1.5, 1.5),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.85),
                },
            ));

            // HP бар контейнер
            info.spawn((
                SelectionPanelUI,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(14.0),
                    border_radius: BorderRadius::all(Val::Px(3.0)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.08, 0.15, 0.8)),
            )).with_children(|bar| {
                // HP заполнение
                bar.spawn((
                    SelectionPanelUI,
                    SelectionHpFill,
                    Node {
                        width: Val::Percent(hp_fraction * 100.0),
                        height: Val::Percent(100.0),
                        border_radius: BorderRadius::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(hp_color),
                ));
            });

            // HP текст
            info.spawn((
                SelectionPanelUI,
                SelectionHpText,
                Text::new(format!("{} / {}", health.current as i32, health.max as i32)),
                TextFont {
                    font: font_ui_bold,
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.8, 0.7)),
                TextShadow {
                    offset: Vec2::new(1.0, 1.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.7),
                },
            ));

            // Статы: Урон | Скорость | Дальность
            info.spawn((
                SelectionPanelUI,
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    margin: UiRect::top(Val::Px(2.0)),
                    ..default()
                },
            )).with_children(|stats| {
                let stat_style = TextFont {
                    font: font_ui.clone(),
                    font_size: 13.0,
                    ..default()
                };
                let stat_color = TextColor(Color::srgb(0.65, 0.6, 0.55));
                let stat_shadow = TextShadow {
                    offset: Vec2::new(1.0, 1.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.6),
                };

                // Урон
                stats.spawn((
                    SelectionPanelUI,
                    Text::new(format!("Урон: {:.0}", attack_cd.damage)),
                    stat_style.clone(),
                    stat_color,
                    stat_shadow,
                ));

                // Скорость
                stats.spawn((
                    SelectionPanelUI,
                    Text::new(format!("Скор: {:.0}", chase.speed)),
                    stat_style.clone(),
                    stat_color,
                    stat_shadow,
                ));

                // Дальность атаки
                stats.spawn((
                    SelectionPanelUI,
                    Text::new(format!("Рад: {:.1}м", chase.attack_range)),
                    stat_style,
                    stat_color,
                    stat_shadow,
                ));
            });
        });
    });
}

/// Обновляет HP-бар и HP-текст в реальном времени.
pub fn update_selection_panel(
    selection: Res<SelectionState>,
    enemies: Query<(&Health, &EnemyType), With<Enemy>>,
    mut hp_text: Query<&mut Text, With<SelectionHpText>>,
    mut hp_fill: Query<&mut Node, With<SelectionHpFill>>,
    mut hp_fill_color: Query<&mut BackgroundColor, With<SelectionHpFill>>,
) {
    let Some(selected) = selection.selected_entity else { return };
    let Ok((health, enemy_type)) = enemies.get(selected) else { return };

    let hp_fraction = (health.current / health.max).clamp(0.0, 1.0);

    for mut text in &mut hp_text {
        **text = format!("{} / {}", health.current as i32, health.max as i32);
    }
    for mut node in &mut hp_fill {
        node.width = Val::Percent(hp_fraction * 100.0);
    }

    // Мигание красным при низком HP (< 30%)
    if hp_fraction < 0.3 {
        let warning_color = Color::srgb(0.9, 0.15, 0.1);
        for mut bg in &mut hp_fill_color {
            bg.0 = warning_color;
        }
    } else {
        let normal_color = match enemy_type {
            EnemyType::Upyr => Color::srgb(0.8, 0.15, 0.1),
            EnemyType::Leshiy => Color::srgb(0.15, 0.7, 0.2),
            EnemyType::Volkolak => Color::srgb(0.5, 0.5, 0.65),
        };
        for mut bg in &mut hp_fill_color {
            bg.0 = normal_color;
        }
    }
}

/// Удаляет UI панели при выходе из Playing.
pub fn cleanup_selection_panel(
    mut commands: Commands,
    query: Query<Entity, With<SelectionPanelUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
