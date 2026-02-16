use bevy::prelude::*;
use crate::modules::progression::components::*;
use crate::modules::player::components::{Player, PlayerStats};
use crate::modules::combat::components::{Weapon, PlayerHealth};
use crate::modules::menu::KillFeedMessage;
use crate::toolkit::asset_paths;
use super::upgrades;

/// Маркер для всех элементов level-up UI (для bulk despawn)
#[derive(Component)]
pub struct LevelUpUI;

/// Маркер карточки апгрейда
#[derive(Component)]
pub struct UpgradeCard {
    pub index: usize,
    pub upgrade_id: UpgradeId,
}

/// Спавнит UI level-up экрана когда LevelUpState становится активным
pub fn spawn_level_up_ui(
    level_up_state: Res<LevelUpState>,
    existing_ui: Query<Entity, With<LevelUpUI>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    inventory: Res<UpgradeInventory>,
) {
    // Только если активен и UI ещё не создан
    if !level_up_state.is_active || !existing_ui.is_empty() {
        return;
    }

    let font_bold = asset_server.load(asset_paths::FONT_UI_BOLD);
    let font_ui = asset_server.load(asset_paths::FONT_UI);

    // Root overlay
    commands.spawn((
        LevelUpUI,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        GlobalZIndex(50),
    )).with_children(|parent| {
        // Заголовок "НОВЫЙ УРОВЕНЬ!"
        parent.spawn((
            Text::new("НОВЫЙ УРОВЕНЬ!"),
            TextFont {
                font: font_bold.clone(),
                font_size: 36.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.85, 0.2)),
            TextShadow {
                offset: Vec2::new(2.0, 2.0),
                color: Color::srgba(0.0, 0.0, 0.0, 0.9),
            },
        ));

        // Контейнер карточек
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(16.0),
                ..default()
            },
        )).with_children(|row| {
            for (i, &upgrade_id) in level_up_state.offered_upgrades.iter().enumerate() {
                let Some(def) = upgrades::get_upgrade_def(&upgrade_id) else { continue };
                let current_level = inventory.get_level(&upgrade_id);

                let category_color = match def.category {
                    UpgradeCategory::Attack => Color::srgb(0.9, 0.3, 0.2),
                    UpgradeCategory::Defense => Color::srgb(0.3, 0.6, 0.9),
                    UpgradeCategory::Path => Color::srgb(0.3, 0.9, 0.4),
                };

                let category_text = match def.category {
                    UpgradeCategory::Attack => "Атака",
                    UpgradeCategory::Defense => "Оберег",
                    UpgradeCategory::Path => "Путь",
                };

                // Карточка
                row.spawn((
                    UpgradeCard { index: i, upgrade_id },
                    Button,
                    Node {
                        width: Val::Px(180.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        row_gap: Val::Px(6.0),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.8, 0.7, 0.3)),
                    BackgroundColor(Color::srgba(0.1, 0.08, 0.15, 0.9)),
                )).with_children(|card| {
                    // Категория
                    card.spawn((
                        Text::new(category_text),
                        TextFont {
                            font: font_ui.clone(),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(category_color),
                    ));

                    // Название
                    card.spawn((
                        Text::new(def.name),
                        TextFont {
                            font: font_bold.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.95, 0.9, 0.8)),
                    ));

                    // Описание
                    card.spawn((
                        Text::new(def.description),
                        TextFont {
                            font: font_ui.clone(),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.75)),
                    ));

                    // Уровень
                    card.spawn((
                        Text::new(format!("Ур. {}/{}", current_level + 1, def.max_level)),
                        TextFont {
                            font: font_ui.clone(),
                            font_size: 13.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.55, 0.5)),
                    ));

                    // Кнопка-подсказка [N]
                    card.spawn((
                        Text::new(format!("[{}]", i + 1)),
                        TextFont {
                            font: font_bold.clone(),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.7, 0.3)),
                    ));
                });
            }
        });
    });
}

/// Обработка выбора апгрейда (клик или клавиши 1/2/3)
pub fn level_up_interaction_system(
    mut level_up_state: ResMut<LevelUpState>,
    mut time: ResMut<Time<Virtual>>,
    mut inventory: ResMut<UpgradeInventory>,
    mut player_query: Query<(&mut Weapon, &mut PlayerHealth, &mut PlayerStats), With<Player>>,
    cards: Query<(&Interaction, &UpgradeCard), Changed<Interaction>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    ui_entities: Query<Entity, (With<LevelUpUI>, Without<ChildOf>)>,
    mut commands: Commands,
    mut feed: MessageWriter<KillFeedMessage>,
) {
    if !level_up_state.is_active {
        return;
    }

    let mut selected: Option<UpgradeId> = None;

    // Клик на карточку
    for (interaction, card) in &cards {
        if *interaction == Interaction::Pressed {
            if let Some(&id) = level_up_state.offered_upgrades.get(card.index) {
                selected = Some(id);
                break;
            }
        }
    }

    // Клавиши 1/2/3
    if selected.is_none() {
        let key_map = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3];
        for (i, key) in key_map.iter().enumerate() {
            if keyboard.just_pressed(*key) {
                if let Some(&id) = level_up_state.offered_upgrades.get(i) {
                    selected = Some(id);
                    break;
                }
            }
        }
    }

    let Some(upgrade_id) = selected else { return };

    // Применяем апгрейд
    if let Ok((mut weapon, mut health, mut stats)) = player_query.single_mut() {
        upgrades::apply_upgrade(
            upgrade_id,
            &mut inventory,
            &mut weapon,
            &mut health,
            &mut stats,
        );

        // Бонус level-up: +5 HP
        health.current = (health.current + 5.0).min(health.max);
    }

    // Kill feed уведомление об апгрейде
    if let Some(def) = upgrades::get_upgrade_def(&upgrade_id) {
        let color = match def.category {
            UpgradeCategory::Attack => Color::srgb(0.9, 0.3, 0.2),
            UpgradeCategory::Defense => Color::srgb(0.3, 0.6, 0.9),
            UpgradeCategory::Path => Color::srgb(0.3, 0.9, 0.4),
        };
        feed.write(KillFeedMessage {
            text: format!("{}: {}", def.name, def.description),
            color,
            group_key: None,
        });
    }

    // Закрываем UI
    level_up_state.is_active = false;
    level_up_state.offered_upgrades.clear();

    // Despawn всех UI элементов
    for entity in &ui_entities {
        commands.entity(entity).despawn();
    }

    // Unpause
    time.unpause();

    info!("✅ Upgrade selected: {:?}", upgrade_id);
}

/// Hover-эффект на карточках
pub fn card_hover_system(
    mut cards: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor), (With<UpgradeCard>, Changed<Interaction>)>,
) {
    for (interaction, mut bg, mut border) in &mut cards {
        match interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgba(0.15, 0.12, 0.25, 0.95));
                *border = BorderColor::all(Color::srgb(1.0, 0.85, 0.3));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgba(0.1, 0.08, 0.15, 0.9));
                *border = BorderColor::all(Color::srgb(0.8, 0.7, 0.3));
            }
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgba(0.2, 0.15, 0.3, 0.95));
            }
        }
    }
}

/// Удаляет level-up UI при новом ране
pub fn cleanup_level_up_ui(
    mut commands: Commands,
    ui_entities: Query<Entity, (With<LevelUpUI>, Without<ChildOf>)>,
    mut level_up_state: ResMut<LevelUpState>,
) {
    for entity in &ui_entities {
        commands.entity(entity).despawn();
    }
    level_up_state.is_active = false;
    level_up_state.offered_upgrades.clear();
}
