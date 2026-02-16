use bevy::prelude::*;
use crate::modules::menu::components::*;
use crate::toolkit::asset_paths;

const FEED_DURATION: f32 = 2.5;
const MAX_FEED_ENTRIES: usize = 6;
const SLIDE_DURATION: f32 = 0.2;
const SLIDE_OFFSET: f32 = -80.0;

/// Спавнит контейнер для kill feed (правый край экрана)
pub fn setup_kill_feed(mut commands: Commands) {
    commands.spawn((
        HudUI,
        KillFeedContainer,
        Node {
            position_type: PositionType::Absolute,
            right: Val::Percent(2.5),
            top: Val::Percent(30.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            row_gap: Val::Px(4.0),
            ..default()
        },
    ));
}

/// Читает KillFeedMessage: группирует одинаковых врагов, создаёт новые записи
pub fn consume_kill_feed_messages(
    mut messages: MessageReader<KillFeedMessage>,
    container_query: Query<Entity, With<KillFeedContainer>>,
    mut entries: Query<(Entity, &mut KillFeedEntry, &Children)>,
    mut texts: Query<&mut Text>,
    all_entries: Query<Entity, With<KillFeedEntry>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let Ok(container) = container_query.single() else { return };
    let font = asset_server.load(asset_paths::FONT_UI_BOLD);

    for msg in messages.read() {
        // Группировка: ищем entry с таким же group_key
        if let Some(ref key) = msg.group_key {
            let mut grouped = false;
            for (_, mut entry, children) in &mut entries {
                if entry.group_key.as_deref() == Some(key) {
                    entry.count += 1;
                    entry.timer.reset();
                    // Обновить текст: "Упырь убит! x3"
                    for child in children.iter() {
                        if let Ok(mut text) = texts.get_mut(child) {
                            **text = format!("{} x{}", msg.text, entry.count);
                        }
                    }
                    grouped = true;
                    break;
                }
            }
            if grouped {
                continue;
            }
        }

        // Лимитируем количество записей — удаляем самую старую
        let entry_count = all_entries.iter().count();
        if entry_count >= MAX_FEED_ENTRIES {
            if let Some(oldest) = all_entries.iter().next() {
                commands.entity(oldest).despawn();
            }
        }

        // Spawn entry: Node wrapper + child Text (для slide-in анимации через margin)
        let entry = commands.spawn((
            HudUI,
            KillFeedEntry {
                timer: Timer::from_seconds(FEED_DURATION, TimerMode::Once),
                group_key: msg.group_key.clone(),
                count: 1,
                base_color: msg.color,
                slide_timer: 0.0,
            },
            Node {
                margin: UiRect::right(Val::Px(SLIDE_OFFSET)),
                ..default()
            },
        )).with_children(|row| {
            row.spawn((
                HudUI,
                Text::new(&msg.text),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(msg.color),
                TextShadow {
                    offset: Vec2::new(1.0, 1.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.85),
                },
            ));
        }).id();

        commands.entity(container).add_child(entry);
    }
}

/// Тикает таймеры, slide-in, fade-out в последние 30%, despawn по завершении
pub fn update_kill_feed(
    time: Res<Time>,
    mut entries: Query<(Entity, &mut KillFeedEntry, &mut Node, &Children)>,
    mut text_colors: Query<(&mut TextColor, &mut TextShadow)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();

    for (entity, mut entry, mut node, children) in &mut entries {
        entry.timer.tick(time.delta());

        // Slide-in анимация
        if entry.slide_timer < 1.0 {
            entry.slide_timer = (entry.slide_timer + dt / SLIDE_DURATION).min(1.0);
            let ease = ease_cubic_out(entry.slide_timer);
            node.margin.right = Val::Px(SLIDE_OFFSET * (1.0 - ease));
        }

        // Fade out в последние 30%
        let progress = entry.timer.fraction();
        if progress > 0.7 {
            let alpha = 1.0 - (progress - 0.7) / 0.3;
            for child in children.iter() {
                if let Ok((mut color, mut shadow)) = text_colors.get_mut(child) {
                    color.0 = entry.base_color.with_alpha(alpha);
                    shadow.color = Color::srgba(0.0, 0.0, 0.0, 0.85 * alpha);
                }
            }
        }

        if entry.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn ease_cubic_out(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}
