use bevy::prelude::*;
use crate::modules::input::data::input_state::InputState;
use crate::modules::player::Player;
use crate::modules::enemies::components::{Enemy, EnemyDying};
use crate::modules::combat::components::Weapon;
use crate::toolkit::asset_paths;

const AUTO_PLAY_SEEK_RADIUS: f32 = 20.0;

/// Состояние автоплея
#[derive(Resource, Default)]
pub struct AutoPlayState {
    pub enabled: bool,
}

/// Маркер UI кнопки автоплея
#[derive(Component)]
pub struct AutoPlayButton;

/// AI движение: WC3/Dota паттерн — бежать к врагу, стоп на дистанции атаки.
/// Ручной ввод (WASD / touch) имеет приоритет.
pub fn auto_play_movement(
    auto_play: Res<AutoPlayState>,
    mut input_state: ResMut<InputState>,
    player_query: Query<(&Transform, &Weapon), With<Player>>,
    enemies: Query<&Transform, (With<Enemy>, Without<EnemyDying>)>,
) {
    // WC3/Dota override: ручной ввод > автопилот
    if !auto_play.enabled || input_state.is_touch_active || input_state.has_keyboard_input {
        return;
    }

    let Ok((player_tf, weapon)) = player_query.single() else { return };
    let player_pos = player_tf.translation;
    let stop_distance = weapon.range * 0.9;

    // Найти ближайшего живого врага
    let mut closest_dist = AUTO_PLAY_SEEK_RADIUS;
    let mut closest_dir = None;

    for enemy_tf in &enemies {
        let diff = enemy_tf.translation - player_pos;
        let dist = diff.length();
        if dist < closest_dist {
            closest_dist = dist;
            closest_dir = Some(diff.normalize());
        }
    }

    if let Some(dir) = closest_dir {
        if closest_dist <= stop_distance {
            // В зоне атаки — стоим, auto_attack сделает остальное
            input_state.movement = Vec3::ZERO;
            input_state.is_running = false;
        } else {
            input_state.movement = Vec3::new(dir.x, 0.0, dir.z);
            // Бег если далеко, шаг если почти дошёл
            input_state.is_running = closest_dist > stop_distance * 2.0;
        }
    } else {
        // Нет врагов — стоим
        input_state.movement = Vec3::ZERO;
        input_state.is_running = false;
    }
}

/// Обработка клика по кнопке автоплея
pub fn toggle_auto_play(
    mut auto_play: ResMut<AutoPlayState>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<AutoPlayButton>),
    >,
) {
    for (interaction, mut bg) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            auto_play.enabled = !auto_play.enabled;
            // Подсветка: золотой когда включён, тёмный когда выключен
            bg.0 = if auto_play.enabled {
                Color::srgba(0.6, 0.45, 0.1, 0.9)
            } else {
                Color::srgba(0.15, 0.12, 0.2, 0.9)
            };
        }
    }
}

/// Спавн UI кнопки автоплея
pub fn spawn_auto_play_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(asset_paths::FONT_UI_BOLD);

    commands.spawn((
        AutoPlayButton,
        Button,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(80.0),
            right: Val::Px(16.0),
            width: Val::Px(48.0),
            height: Val::Px(48.0),
            border_radius: BorderRadius::all(Val::Percent(50.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.15, 0.12, 0.2, 0.9)),
        GlobalZIndex(200),
    )).with_children(|parent| {
        // Frame overlay — текстура кнопки
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border_radius: BorderRadius::all(Val::Percent(50.0)),
                ..default()
            },
            ImageNode::new(asset_server.load(asset_paths::UI_BUTTON_FRAME)),
        ));
        // Текст "A"
        parent.spawn((
            Text::new("A"),
            TextFont { font, font_size: 22.0, ..default() },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
        ));
    });
}

/// Despawn кнопки при выходе из Playing
pub fn cleanup_auto_play_button(
    mut commands: Commands,
    query: Query<Entity, With<AutoPlayButton>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
