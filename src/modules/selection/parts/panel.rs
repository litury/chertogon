use bevy::prelude::*;
use crate::modules::enemies::components::{Enemy, EnemyType, Health, ChasePlayer};
use crate::modules::combat::components::{EnemyAttackCooldown, PlayerHealth, Weapon};
use crate::modules::player::Player;
use crate::modules::selection::components::*;
use crate::modules::selection::parts::portrait;
use crate::toolkit::asset_paths;

/// Пересоздаёт панель выделения при изменении SelectionState.
pub fn manage_selection_panel(
    selection: Res<SelectionState>,
    enemies: Query<
        (&EnemyType, &Health, &ChasePlayer, &EnemyAttackCooldown),
        With<Enemy>,
    >,
    players: Query<(&PlayerHealth, &Weapon), With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing_panels: Query<Entity, (With<SelectionPanelUI>, Without<ChildOf>)>,
) {
    if !selection.is_changed() {
        return;
    }

    // Despawn старой панели
    for entity in &existing_panels {
        commands.entity(entity).despawn();
    }

    let Some(selected) = selection.selected_entity else { return };

    if let Ok((enemy_type, health, chase, attack_cd)) = enemies.get(selected) {
        let portrait_path = portrait::portrait_for_enemy(enemy_type);
        build_enemy_panel(
            &mut commands, &asset_server, portrait_path,
            enemy_type, health, chase, attack_cd,
        );
    } else if let Ok((player_health, weapon)) = players.get(selected) {
        let portrait_path = portrait::portrait_for_player();
        build_player_panel(
            &mut commands, &asset_server, portrait_path,
            player_health, weapon,
        );
    }
}

/// Общий каркас панели (root + portrait + info column)
fn spawn_panel_root(
    commands: &mut Commands,
    asset_server: &AssetServer,
    portrait_path: &str,
) -> Entity {
    let portrait_handle: Handle<Image> = asset_server.load(portrait_path.to_string());

    commands.spawn((
        SelectionPanelUI,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(60.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-170.0)),
            width: Val::Px(340.0),
            padding: UiRect {
                left: Val::Px(32.0),
                right: Val::Px(32.0),
                top: Val::Px(20.0),
                bottom: Val::Px(8.0),
            },
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            border: UiRect::all(Val::Px(1.5)),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            overflow: Overflow::clip(),
            ..default()
        },
        ImageNode::new(asset_server.load(asset_paths::UI_PANEL_BG)),
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
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            BorderColor::all(Color::srgba(0.95, 0.7, 0.2, 0.4)),
        );

        parent.spawn((
            portrait_node,
            ImageNode::new(portrait_handle),
        ));
    }).id()
}

/// Спавнит info-колонку внутри root
fn spawn_info_column(
    commands: &mut Commands,
    root: Entity,
    name: &str,
    hp_current: f32,
    hp_max: f32,
    hp_color: Color,
    stats: &[(&str, String, &str)],
    asset_server: &AssetServer,
) {
    let font_title = asset_server.load(asset_paths::FONT_TITLE);
    let font_ui = asset_server.load(asset_paths::FONT_UI);
    let font_ui_bold = asset_server.load(asset_paths::FONT_UI_BOLD);
    let hp_fraction = (hp_current / hp_max).clamp(0.0, 1.0);

    let info_column = commands.spawn((
        SelectionPanelUI,
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(3.0),
            flex_grow: 1.0,
            ..default()
        },
    )).with_children(|info| {
        // Имя
        info.spawn((
            SelectionPanelUI,
            Text::new(name),
            TextFont { font: font_title, font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.95, 0.7, 0.2)),
            TextShadow {
                offset: Vec2::new(1.5, 1.5),
                color: Color::srgba(0.0, 0.0, 0.0, 0.85),
            },
        ));

        // HP бар
        info.spawn((
            SelectionPanelUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(12.0),
                border_radius: BorderRadius::all(Val::Px(3.0)),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.08, 0.15, 0.8)),
        )).with_children(|bar| {
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

        // Ряд: HP текст + статы
        info.spawn((
            SelectionPanelUI,
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                ..default()
            },
        )).with_children(|row| {
            // HP текст (лево)
            row.spawn((
                SelectionPanelUI,
                SelectionHpText,
                Text::new(format!("{}/{}", hp_current as i32, hp_max as i32)),
                TextFont { font: font_ui_bold, font_size: 14.0, ..default() },
                TextColor(Color::srgb(0.95, 0.9, 0.8)),
                TextShadow {
                    offset: Vec2::new(1.0, 1.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.9),
                },
            ));
            // Статы с иконками (право)
            for (_label, value, icon_path) in stats {
                row.spawn((
                    SelectionPanelUI,
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(2.0),
                        ..default()
                    },
                )).with_children(|stat| {
                    // Иконка
                    stat.spawn((
                        SelectionPanelUI,
                        Node {
                            width: Val::Px(14.0),
                            height: Val::Px(14.0),
                            ..default()
                        },
                        ImageNode::new(asset_server.load(icon_path.to_string())),
                    ));
                    // Значение
                    stat.spawn((
                        SelectionPanelUI,
                        Text::new(value.clone()),
                        TextFont { font: font_ui.clone(), font_size: 14.0, ..default() },
                        TextColor(Color::srgb(0.85, 0.8, 0.7)),
                        TextShadow {
                            offset: Vec2::new(1.0, 1.0),
                            color: Color::srgba(0.0, 0.0, 0.0, 0.85),
                        },
                    ));
                });
            }
        });
    }).id();

    commands.entity(root).add_child(info_column);
}

fn build_enemy_panel(
    commands: &mut Commands,
    asset_server: &AssetServer,
    portrait_path: &str,
    enemy_type: &EnemyType,
    health: &Health,
    chase: &ChasePlayer,
    attack_cd: &EnemyAttackCooldown,
) {
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
    let stats = vec![
        ("damage", format!("{:.0}", attack_cd.damage), asset_paths::ICON_DAMAGE),
        ("speed", format!("{:.0}", chase.speed), asset_paths::ICON_SPEED),
        ("range", format!("{:.1}м", chase.attack_range), asset_paths::ICON_RANGE),
    ];

    let root = spawn_panel_root(commands, asset_server, portrait_path);
    spawn_info_column(commands, root, name, health.current, health.max, hp_color, &stats, asset_server);
}

fn build_player_panel(
    commands: &mut Commands,
    asset_server: &AssetServer,
    portrait_path: &str,
    player_health: &PlayerHealth,
    weapon: &Weapon,
) {
    let hp_color = Color::srgb(0.95, 0.7, 0.2);
    let stats = vec![
        ("damage", format!("{:.0}", weapon.damage), asset_paths::ICON_DAMAGE),
        ("range", format!("{:.1}м", weapon.range), asset_paths::ICON_RANGE),
        ("cooldown", format!("{:.1}с", weapon.cooldown), asset_paths::ICON_SPEED),
    ];

    let root = spawn_panel_root(commands, asset_server, portrait_path);
    spawn_info_column(
        commands, root, "Богатырь",
        player_health.current, player_health.max,
        hp_color, &stats, asset_server,
    );
}

/// Обновляет HP-бар и HP-текст в реальном времени.
pub fn update_selection_panel(
    selection: Res<SelectionState>,
    enemies: Query<(&Health, &EnemyType), With<Enemy>>,
    players: Query<&PlayerHealth, With<Player>>,
    mut hp_text: Query<&mut Text, With<SelectionHpText>>,
    mut hp_fill: Query<&mut Node, With<SelectionHpFill>>,
    mut hp_fill_color: Query<&mut BackgroundColor, With<SelectionHpFill>>,
) {
    let Some(selected) = selection.selected_entity else { return };

    let (hp_current, hp_max, normal_color) =
        if let Ok((health, enemy_type)) = enemies.get(selected) {
            let color = match enemy_type {
                EnemyType::Upyr => Color::srgb(0.8, 0.15, 0.1),
                EnemyType::Leshiy => Color::srgb(0.15, 0.7, 0.2),
                EnemyType::Volkolak => Color::srgb(0.5, 0.5, 0.65),
            };
            (health.current, health.max, color)
        } else if let Ok(health) = players.get(selected) {
            (health.current, health.max, Color::srgb(0.95, 0.7, 0.2))
        } else {
            return;
        };

    let hp_fraction = (hp_current / hp_max).clamp(0.0, 1.0);

    for mut text in &mut hp_text {
        **text = format!("{} / {}", hp_current as i32, hp_max as i32);
    }
    for mut node in &mut hp_fill {
        node.width = Val::Percent(hp_fraction * 100.0);
    }

    let fill_color = if hp_fraction < 0.3 {
        Color::srgb(0.9, 0.15, 0.1)
    } else {
        normal_color
    };
    for mut bg in &mut hp_fill_color {
        bg.0 = fill_color;
    }
}

/// Удаляет UI панели при выходе из Playing.
pub fn cleanup_selection_panel(
    mut commands: Commands,
    query: Query<Entity, (With<SelectionPanelUI>, Without<ChildOf>)>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
