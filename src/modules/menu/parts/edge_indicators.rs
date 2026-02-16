use bevy::prelude::*;
use bevy::ui::UiScale;
use crate::modules::menu::components::*;
use crate::modules::enemies::components::{Enemy, EnemyDying};
use crate::modules::player::components::Player;

const MAX_INDICATORS: usize = 8;
const INDICATOR_SIZE: f32 = 16.0;
const SCREEN_MARGIN: f32 = 8.0;

/// Спавнит пул из MAX_INDICATORS скрытых индикаторов на краях экрана
pub fn setup_edge_indicators(mut commands: Commands) {
    for i in 0..MAX_INDICATORS {
        commands.spawn((
            HudUI,
            EdgeIndicator { index: i as u8 },
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(INDICATOR_SIZE),
                height: Val::Px(INDICATOR_SIZE),
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                border_radius: BorderRadius::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.9, 0.15, 0.1, 0.85)),
            Visibility::Hidden,
            GlobalZIndex(10),
        ));
    }
}

/// Каждый кадр: находит off-screen врагов, позиционирует индикаторы на краях экрана
pub fn update_edge_indicators(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    player_query: Query<&Transform, With<Player>>,
    enemies: Query<&Transform, (With<Enemy>, Without<EnemyDying>, Without<Player>)>,
    mut indicators: Query<(&EdgeIndicator, &mut Node, &mut Visibility)>,
    ui_scale: Res<UiScale>,
    windows: Query<&Window>,
    mut offscreen: Local<Vec<(f32, Vec3)>>,
) {
    let Ok((camera, cam_transform)) = camera_query.single() else { return };
    let Ok(player_tf) = player_query.single() else { return };
    let Ok(window) = windows.single() else { return };
    let scale = ui_scale.0 as f32;
    let scale = if scale < 0.01 { 1.0 } else { scale };

    let screen_w = window.width() / scale;
    let screen_h = window.height() / scale;
    let player_pos = player_tf.translation;

    // Собираем off-screen врагов с дистанциями (Local — 0 аллокаций в steady state)
    offscreen.clear();

    for enemy_tf in &enemies {
        let world_pos = enemy_tf.translation;
        let dist_sq = (world_pos - player_pos).length_squared();

        match camera.world_to_viewport(cam_transform, world_pos) {
            Ok(screen_pos) => {
                let sp = screen_pos / scale;
                // За пределами видимой зоны?
                if sp.x < 0.0 || sp.x > screen_w || sp.y < 0.0 || sp.y > screen_h {
                    offscreen.push((dist_sq, world_pos));
                }
            }
            Err(_) => {
                // За камерой
                offscreen.push((dist_sq, world_pos));
            }
        }
    }

    // Partial sort: только top-8 ближайших, O(n) вместо O(n log n)
    if offscreen.len() > MAX_INDICATORS {
        offscreen.select_nth_unstable_by(MAX_INDICATORS, |a, b|
            a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
        );
        offscreen.truncate(MAX_INDICATORS);
    }
    offscreen.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // Обновляем индикаторы
    for (indicator, mut node, mut visibility) in &mut indicators {
        let idx = indicator.index as usize;

        if idx < offscreen.len() {
            let (_, world_pos) = offscreen[idx];

            // Промежуточная точка ближе к игроку (для стабильной проекции)
            let dir = (world_pos - player_pos).normalize_or_zero();
            let near_pos = player_pos + dir * 2.0;
            let center = Vec2::new(screen_w / 2.0, screen_h / 2.0);

            if let Ok(screen_pos) = camera.world_to_viewport(cam_transform, near_pos) {
                let sp = screen_pos / scale;
                let dir_2d = (sp - center).normalize_or_zero();

                if dir_2d.length() < 0.001 {
                    *visibility = Visibility::Hidden;
                    continue;
                }

                let (clamped_x, clamped_y) = clamp_to_screen_edge(
                    center, dir_2d, screen_w, screen_h, SCREEN_MARGIN,
                );

                node.left = Val::Px(clamped_x - INDICATOR_SIZE / 2.0);
                node.top = Val::Px(clamped_y - INDICATOR_SIZE / 2.0);
                *visibility = Visibility::Inherited;
            } else {
                *visibility = Visibility::Hidden;
            }
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

/// Clamp точки в направлении dir от center на край экрана с margin
fn clamp_to_screen_edge(
    center: Vec2,
    dir: Vec2,
    screen_w: f32,
    screen_h: f32,
    margin: f32,
) -> (f32, f32) {
    let half_w = screen_w / 2.0 - margin;
    let half_h = screen_h / 2.0 - margin;

    // Как далеко можно уйти по dir пока не упрёмся в край
    let t_x = if dir.x.abs() > 0.001 { half_w / dir.x.abs() } else { f32::MAX };
    let t_y = if dir.y.abs() > 0.001 { half_h / dir.y.abs() } else { f32::MAX };
    let t = t_x.min(t_y);

    let point = center + dir * t;
    (
        point.x.clamp(margin, screen_w - margin),
        point.y.clamp(margin, screen_h - margin),
    )
}
