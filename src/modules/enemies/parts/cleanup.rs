use bevy::prelude::*;
use crate::modules::enemies::components::*;
use crate::modules::combat::parts::game_over::KillCount;

/// Удаляет всех врагов, трупы и умирающих
pub fn despawn_enemies(
    mut commands: Commands,
    enemies: Query<Entity, Or<(With<Enemy>, With<EnemyCorpse>, With<EnemyDying>)>>,
) {
    for entity in &enemies {
        commands.entity(entity).despawn();
    }
}

/// Сбрасывает волновую систему на начальное состояние
pub fn reset_wave_state(mut wave: ResMut<WaveState>) {
    *wave = WaveState::default();
}

/// Сбрасывает счётчик убийств
pub fn reset_kill_count(mut kills: ResMut<KillCount>) {
    kills.total = 0;
}
