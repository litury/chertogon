use bevy::prelude::*;
use crate::toolkit::asset_paths;

/// Всплывающее число урона
#[derive(Component)]
pub struct DamageNumber {
    pub timer: Timer,
    pub velocity: Vec3,
}

/// Спавнит всплывающее число урона над врагом
pub fn spawn_damage_number(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec3,
    damage: f32,
) {
    let font = asset_server.load(asset_paths::FONT_UI_BOLD);
    let text = format!("-{}", damage as i32);

    commands.spawn((
        // Bevy 0.18: Text2d для мирового текста
        Text2d::new(text),
        TextFont {
            font,
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.3, 0.1)),
        Transform::from_translation(position + Vec3::Y * 2.0)
            .with_scale(Vec3::splat(0.02)), // Масштаб для мирового пространства
        DamageNumber {
            timer: Timer::from_seconds(0.8, TimerMode::Once),
            velocity: Vec3::new(0.0, 3.0, 0.0),
        },
    ));
}

/// Система: числа урона поднимаются вверх и исчезают
pub fn damage_number_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut DamageNumber, &mut Transform, &mut TextColor)>,
    mut commands: Commands,
) {
    for (entity, mut dmg, mut transform, mut color) in &mut query {
        dmg.timer.tick(time.delta());
        let dt = time.delta_secs();

        // Поднимается вверх с замедлением
        transform.translation += dmg.velocity * dt;
        dmg.velocity.y *= 0.96; // Затухание

        // Fade out
        let progress = dmg.timer.fraction();
        let alpha = if progress > 0.5 {
            1.0 - (progress - 0.5) / 0.5
        } else {
            1.0
        };
        color.0 = color.0.with_alpha(alpha);

        if dmg.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
