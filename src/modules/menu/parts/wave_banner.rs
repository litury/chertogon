use bevy::prelude::*;
use crate::modules::menu::components::*;
use crate::modules::enemies::components::{WaveState, WavePhase};
use crate::toolkit::asset_paths;

/// Спавнит баннер "ВОЛНА N" по центру экрана при старте новой волны
pub fn spawn_wave_banner(
    wave: Res<WaveState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing: Query<Entity, With<WaveBanner>>,
) {
    if !wave.is_changed() {
        return;
    }
    if wave.phase != WavePhase::Spawning {
        return;
    }

    // Убираем предыдущий баннер если есть
    for entity in &existing {
        commands.entity(entity).despawn();
    }

    let font = asset_server.load(asset_paths::FONT_TITLE);

    commands.spawn((
        HudUI,
        WaveBanner {
            timer: Timer::from_seconds(2.5, TimerMode::Once),
        },
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            HudUI,
            Text::new(format!("ВОЛНА {}", wave.current_wave)),
            TextFont {
                font,
                font_size: 48.0,
                ..default()
            },
            TextColor(Color::srgba(0.95, 0.7, 0.2, 0.0)),
            TextShadow {
                offset: Vec2::new(3.0, 3.0),
                color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            },
        ));
    });
}

/// Анимирует баннер: fade-in → hold → fade-out, затем despawn
pub fn animate_wave_banner(
    time: Res<Time>,
    mut banners: Query<(Entity, &mut WaveBanner, &Children)>,
    mut text_colors: Query<(&mut TextColor, &mut TextShadow)>,
    mut commands: Commands,
) {
    for (entity, mut banner, children) in &mut banners {
        banner.timer.tick(time.delta());
        let t = banner.timer.fraction();

        let alpha = if t < 0.12 {
            // FadeIn: 0→1
            ease_cubic_out(t / 0.12)
        } else if t < 0.72 {
            // Hold
            1.0
        } else {
            // FadeOut: 1→0
            1.0 - ease_quadratic_in((t - 0.72) / 0.28)
        };

        // Обновить alpha у дочернего текста
        for child in children.iter() {
            if let Ok((mut color, mut shadow)) = text_colors.get_mut(child) {
                color.0 = color.0.with_alpha(alpha);
                shadow.color = shadow.color.with_alpha(alpha * 0.8);
            }
        }

        if banner.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn ease_cubic_out(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

fn ease_quadratic_in(t: f32) -> f32 {
    t * t
}
