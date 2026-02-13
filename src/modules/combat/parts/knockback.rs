use bevy::prelude::*;
use avian3d::prelude::*;
use crate::modules::enemies::components::EnemyAnimState;

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

/// Фаза восстановления после stagger — задержка перед возвратом AI контроля.
/// Пока активна, враг стоит на месте (velocity = 0) и плавно переходит в idle.
#[derive(Component)]
pub struct StaggerRecovery {
    pub timer: Timer,
}

impl StaggerRecovery {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

/// Система: снимает Staggered когда таймер истёк, обнуляет velocity,
/// переводит в фазу Recovery
pub fn stagger_decay_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Staggered, &mut LinearVelocity, &mut EnemyAnimState)>,
    mut commands: Commands,
) {
    for (entity, mut stagger, mut velocity, mut anim_state) in &mut query {
        stagger.timer.tick(time.delta());
        if stagger.timer.is_finished() {
            // Обнуляем остаточный knockback velocity
            velocity.0 = Vec3::ZERO;

            // Переводим анимацию из HitReaction → Idle
            if anim_state.current == crate::modules::enemies::components::EnemyAnim::HitReaction {
                anim_state.current = crate::modules::enemies::components::EnemyAnim::Idle;
            }

            // Заменяем Stagger на Recovery (задержка 0.2с перед возвратом AI)
            commands.entity(entity)
                .remove::<Staggered>()
                .insert(StaggerRecovery::new(0.2));
        }
    }
}

/// Система: тикает Recovery таймер, по завершении — возвращает AI контроль
pub fn recovery_decay_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut StaggerRecovery)>,
    mut commands: Commands,
) {
    for (entity, mut recovery) in &mut query {
        recovery.timer.tick(time.delta());
        if recovery.timer.is_finished() {
            commands.entity(entity).remove::<StaggerRecovery>();
        }
    }
}
