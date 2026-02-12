use bevy::prelude::*;

/// Маркер «враг получил удар» — scale-pop эффект (раздуваем и обратно)
#[derive(Component)]
pub struct HitFlash {
    pub timer: Timer,
}

impl HitFlash {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

/// Система: scale-pop — враг кратковременно раздувается при ударе, потом возвращается
pub fn hit_flash_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut HitFlash, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, mut flash, mut transform) in &mut query {
        flash.timer.tick(time.delta());

        let progress = flash.timer.fraction();
        // Быстро раздувается до 1.2× в первой половине, потом обратно
        let scale_factor = if progress < 0.5 {
            1.0 + 0.2 * (progress / 0.5)
        } else {
            1.0 + 0.2 * (1.0 - (progress - 0.5) / 0.5)
        };
        transform.scale = Vec3::splat(scale_factor);

        if flash.timer.is_finished() {
            transform.scale = Vec3::ONE;
            commands.entity(entity).remove::<HitFlash>();
        }
    }
}
