use bevy::prelude::*;

/// Маркер оглушения от удара — пока на враге, AI не управляет движением
#[derive(Component)]
pub struct Staggered {
    pub timer: Timer,
}

impl Staggered {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

/// Система: снимает Staggered когда таймер истёк
pub fn stagger_decay_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Staggered)>,
    mut commands: Commands,
) {
    for (entity, mut stagger) in &mut query {
        stagger.timer.tick(time.delta());
        if stagger.timer.is_finished() {
            commands.entity(entity).remove::<Staggered>();
        }
    }
}
