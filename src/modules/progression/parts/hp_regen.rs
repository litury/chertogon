use bevy::prelude::*;
use crate::modules::player::components::Player;
use crate::modules::combat::components::PlayerHealth;
use crate::modules::progression::components::{UpgradeInventory, UpgradeId};

/// Регенерация HP от Слезы Лады (+1 HP/сек за уровень апгрейда)
pub fn hp_regen_system(
    time: Res<Time>,
    inventory: Res<UpgradeInventory>,
    mut player: Query<&mut PlayerHealth, With<Player>>,
) {
    let regen_level = inventory.get_level(&UpgradeId::SlezaLady);
    if regen_level == 0 { return; }

    let dt = time.delta_secs();
    if dt == 0.0 { return; }

    let regen_per_sec = regen_level as f32;
    if let Ok(mut health) = player.single_mut() {
        if health.current < health.max {
            health.current = (health.current + regen_per_sec * dt).min(health.max);
        }
    }
}
