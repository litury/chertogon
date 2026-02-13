use bevy::prelude::*;
use crate::toolkit::asset_paths;
use crate::modules::selection::components::PortraitCamera;

/// Всплывающее число урона (UI-based, проецируется из 3D в экранные координаты)
#[derive(Component)]
pub struct DamageNumber {
    pub timer: Timer,
    pub world_position: Vec3,
    pub velocity: Vec3,
    pub base_font_size: f32,
}

/// Спавнит число урона как UI-элемент с абсолютной позицией
pub fn spawn_damage_number(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec3,
    damage: f32,
) {
    let font = asset_server.load(asset_paths::FONT_UI_BOLD);
    let text = format!("-{}", damage as i32);

    // Детерминистичный X-разброс из позиции врага
    let seed = (position.x * 73.7 + position.z * 31.3).sin();
    let x_spread = seed * 1.5;

    let base_font_size = 28.0;

    commands.spawn((
        // UI Node с абсолютной позицией
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            ..default()
        },
        Text::new(text),
        TextFont {
            font,
            font_size: base_font_size * 1.5, // scale pop: начинаем на 1.5×
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.3, 0.1)),
        TextShadow {
            offset: Vec2::new(1.5, 1.5),
            color: Color::srgba(0.0, 0.0, 0.0, 0.9),
        },
        // Начинаем невидимым — позиция обновится в первом кадре системы
        Visibility::Hidden,
        DamageNumber {
            timer: Timer::from_seconds(0.8, TimerMode::Once),
            world_position: position + Vec3::new(x_spread * 0.3, 2.0, 0.0),
            velocity: Vec3::new(x_spread, 4.0, 0.0),
            base_font_size,
        },
    ));
}

/// Проецирует 3D позицию в экранные координаты, анимирует и despawn'ит
pub fn damage_number_system(
    time: Res<Time>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera3d>, Without<PortraitCamera>)>,
    mut query: Query<(Entity, &mut DamageNumber, &mut Node, &mut TextFont, &mut TextColor, &mut Visibility)>,
    mut commands: Commands,
) {
    let Ok((camera, cam_transform)) = camera_query.single() else { return };
    let dt = time.delta_secs();

    for (entity, mut dmg, mut node, mut text_font, mut color, mut visibility) in &mut query {
        dmg.timer.tick(time.delta());
        let progress = dmg.timer.fraction();

        // Физика: гравитационная дуга
        dmg.velocity.y -= 8.0 * dt;
        let vel = dmg.velocity;
        dmg.world_position += vel * dt;

        // Проецируем 3D → экран
        if let Ok(screen_pos) = camera.world_to_viewport(cam_transform, dmg.world_position) {
            *visibility = Visibility::Inherited;
            node.left = Val::Px(screen_pos.x);
            node.top = Val::Px(screen_pos.y);
        } else {
            // За пределами экрана — убираем
            commands.entity(entity).despawn();
            continue;
        }

        // Scale pop: font_size 1.5× → 1.0× за первые 20%
        let scale_mult = if progress < 0.2 {
            1.5 - 0.5 * (progress / 0.2)
        } else {
            1.0
        };
        text_font.font_size = dmg.base_font_size * scale_mult;

        // Fade out в последние 40%
        let alpha = if progress > 0.6 {
            1.0 - (progress - 0.6) / 0.4
        } else {
            1.0
        };
        color.0 = color.0.with_alpha(alpha);

        if dmg.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
