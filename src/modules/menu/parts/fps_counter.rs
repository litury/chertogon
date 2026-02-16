use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use crate::modules::menu::components::FpsText;

#[derive(Resource)]
pub(crate) struct FpsUpdateTimer {
    timer: Timer,
    last_value: u32,
}

pub fn setup_fps(mut commands: Commands) {
    commands.insert_resource(FpsUpdateTimer {
        timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        last_value: 0,
    });
    commands.spawn((
        FpsText,
        Text::new("FPS: ..."),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.2, 0.9, 0.2)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(4.0),
            right: Val::Px(8.0),
            ..default()
        },
    ));
}

pub fn update_fps(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut state: ResMut<FpsUpdateTimer>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    state.timer.tick(time.delta());
    if !state.timer.just_finished() {
        return;
    }

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            let rounded = value as u32;
            if rounded != state.last_value {
                state.last_value = rounded;
                for mut text in &mut query {
                    **text = format!("FPS: {rounded}");
                }
            }
        }
    }
}

pub fn cleanup_fps(mut commands: Commands, query: Query<Entity, With<FpsText>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<FpsUpdateTimer>();
}
