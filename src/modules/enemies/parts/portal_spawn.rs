use bevy::prelude::*;
use crate::modules::enemies::components::PortalSpawnAnim;

/// Система анимации появления врага из портала (масштаб 0.01→1.0 за 0.5с)
pub fn portal_spawn_anim_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut PortalSpawnAnim, &mut Transform)>,
) {
    for (entity, mut anim, mut transform) in &mut query {
        anim.timer.tick(time.delta());
        let progress = anim.timer.fraction();

        // Ease-out cubic: быстро вырастает, плавно завершается
        let t = 1.0 - (1.0 - progress).powi(3);
        let scale = 0.01_f32.lerp(1.0, t);
        transform.scale = Vec3::splat(scale);

        if anim.timer.is_finished() {
            transform.scale = Vec3::ONE;
            commands.entity(entity).remove::<PortalSpawnAnim>();
        }
    }
}
