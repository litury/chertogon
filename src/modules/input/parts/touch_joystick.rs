use bevy::prelude::*;
use crate::modules::input::data::input_state::InputState;

const JOYSTICK_BASE_SIZE: f32 = 120.0;
const JOYSTICK_THUMB_SIZE: f32 = 40.0;
const JOYSTICK_THUMB_MAX_OFFSET: f32 = 50.0;
const DEAD_ZONE: f32 = 5.0;

/// Внешний круг joystick (появляется в точке касания)
#[derive(Component)]
pub struct JoystickBase;

/// Внутренний кружок (следует за пальцем)
#[derive(Component)]
pub struct JoystickThumb;

/// Показывает/скрывает floating joystick при touch drag
pub fn update_touch_joystick(
    input_state: Res<InputState>,
    mut commands: Commands,
    base_query: Query<Entity, With<JoystickBase>>,
    mut thumb_query: Query<(Entity, &mut Node), With<JoystickThumb>>,
) {
    let should_show = input_state.is_touch_active
        && input_state.touch_start.is_some()
        && input_state.touch_current.is_some();

    if should_show {
        let start = input_state.touch_start.unwrap();
        let current = input_state.touch_current.unwrap();
        let delta = current - start;
        let distance = delta.length();

        // Не показывать в dead zone (это может быть tap)
        if distance < DEAD_ZONE {
            return;
        }

        // Clamp thumb offset к максимальному радиусу
        let thumb_offset = if distance > JOYSTICK_THUMB_MAX_OFFSET {
            delta.normalize() * JOYSTICK_THUMB_MAX_OFFSET
        } else {
            delta
        };

        if base_query.is_empty() {
            // Spawn base + thumb
            let base_x = start.x - JOYSTICK_BASE_SIZE / 2.0;
            let base_y = start.y - JOYSTICK_BASE_SIZE / 2.0;

            commands.spawn((
                JoystickBase,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(base_x),
                    top: Val::Px(base_y),
                    width: Val::Px(JOYSTICK_BASE_SIZE),
                    height: Val::Px(JOYSTICK_BASE_SIZE),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Percent(50.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.1)),
                BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.25)),
                GlobalZIndex(150),
            ));

            let thumb_x = start.x + thumb_offset.x - JOYSTICK_THUMB_SIZE / 2.0;
            let thumb_y = start.y + thumb_offset.y - JOYSTICK_THUMB_SIZE / 2.0;

            commands.spawn((
                JoystickThumb,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(thumb_x),
                    top: Val::Px(thumb_y),
                    width: Val::Px(JOYSTICK_THUMB_SIZE),
                    height: Val::Px(JOYSTICK_THUMB_SIZE),
                    border_radius: BorderRadius::all(Val::Percent(50.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
                GlobalZIndex(151),
            ));
        } else {
            // Update thumb position
            for (_entity, mut node) in &mut thumb_query {
                node.left = Val::Px(start.x + thumb_offset.x - JOYSTICK_THUMB_SIZE / 2.0);
                node.top = Val::Px(start.y + thumb_offset.y - JOYSTICK_THUMB_SIZE / 2.0);
            }
        }
    } else {
        // Despawn joystick
        for entity in &base_query {
            commands.entity(entity).despawn();
        }
        for (entity, _node) in &thumb_query {
            commands.entity(entity).despawn();
        }
    }
}
