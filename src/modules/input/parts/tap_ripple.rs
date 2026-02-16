use bevy::prelude::*;
use crate::modules::selection::components::SelectionTapEvent;

/// Анимированное ripple-кольцо в месте тапа
#[derive(Component)]
pub struct TapRipple {
    pub timer: Timer,
    pub center: Vec2, // Screen-space центр ripple
}

/// Спавнит ripple-кольцо при получении SelectionTapEvent
pub fn spawn_tap_ripple(
    mut tap_events: MessageReader<SelectionTapEvent>,
    mut commands: Commands,
) {
    for event in tap_events.read() {
        let pos = event.screen_pos;
        let half = 15.0;

        commands.spawn((
            TapRipple {
                timer: Timer::from_seconds(0.4, TimerMode::Once),
                center: pos,
            },
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(pos.x - half),
                top: Val::Px(pos.y - half),
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Percent(50.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.8)),
            GlobalZIndex(160),
        ));
    }
}

/// Анимирует ripple: scale up + fade out, despawn по таймеру
pub fn animate_tap_ripple(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TapRipple, &mut Node, &mut BorderColor)>,
) {
    for (entity, mut ripple, mut node, mut border_color) in &mut query {
        ripple.timer.tick(time.delta());
        let t = ripple.timer.fraction(); // 0.0 → 1.0

        // Scale: 30px → 90px (3×), keep centered
        let size = 30.0 + 60.0 * t;
        let half = size / 2.0;
        node.left = Val::Px(ripple.center.x - half);
        node.top = Val::Px(ripple.center.y - half);
        node.width = Val::Px(size);
        node.height = Val::Px(size);

        // Fade out
        let alpha = 0.8 * (1.0 - t);
        *border_color = BorderColor::all(Color::srgba(1.0, 1.0, 1.0, alpha));

        if ripple.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
