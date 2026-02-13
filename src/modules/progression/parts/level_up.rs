use bevy::prelude::*;
use crate::modules::progression::components::{PlayerXp, LevelUpState};
use super::upgrades::pick_random_upgrades;
use crate::modules::progression::components::UpgradeInventory;

/// Детектирует pending level-up → ставит игру на паузу, генерирует апгрейды
pub fn check_level_up_system(
    mut player_xp: ResMut<PlayerXp>,
    mut level_up_state: ResMut<LevelUpState>,
    mut time: ResMut<Time<Virtual>>,
    inventory: Res<UpgradeInventory>,
) {
    if !player_xp.pending_level_up || level_up_state.is_active {
        return;
    }

    player_xp.pending_level_up = false;

    // Генерируем 3 случайных апгрейда
    let offered = pick_random_upgrades(&inventory, 3);
    if offered.is_empty() {
        // Все апгрейды на максе — просто пропускаем
        return;
    }

    level_up_state.is_active = true;
    level_up_state.offered_upgrades = offered;

    // Пауза игры
    time.pause();

    info!("⬆️ LEVEL UP! Level {}, offered {} upgrades", player_xp.level, level_up_state.offered_upgrades.len());
}
