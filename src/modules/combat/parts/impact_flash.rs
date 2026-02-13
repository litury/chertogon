use bevy::prelude::*;

/// Кратковременная вспышка света в точке удара
#[derive(Component)]
pub struct ImpactFlash {
    pub timer: Timer,
    pub initial_intensity: f32,
}

/// Спавнит яркий point light на 0.1с в точке удара
pub fn spawn_impact_flash(
    commands: &mut Commands,
    hit_pos: Vec3,
) {
    let intensity = 5000.0;

    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.7, 0.3),
            intensity,
            range: 4.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_translation(hit_pos + Vec3::Y * 1.0),
        ImpactFlash {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
            initial_intensity: intensity,
        },
    ));
}

/// Затухание и despawn вспышки света
pub fn impact_flash_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut ImpactFlash, &mut PointLight)>,
) {
    for (entity, mut flash, mut light) in &mut query {
        flash.timer.tick(time.delta());

        let progress = flash.timer.fraction();
        // Быстрое квадратичное затухание
        let fade = (1.0 - progress) * (1.0 - progress);
        light.intensity = flash.initial_intensity * fade;

        if flash.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
