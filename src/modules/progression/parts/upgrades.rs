use bevy::prelude::*;
use crate::modules::player::components::PlayerStats;
use crate::modules::combat::components::{Weapon, PlayerHealth};
use crate::modules::progression::components::*;
use crate::shared::rand_01;

/// Все определения апгрейдов
pub fn all_upgrades() -> Vec<UpgradeDef> {
    vec![
        UpgradeDef {
            id: UpgradeId::RunePeruna,
            name: "Руна Перуна",
            description: "+5% урон",
            category: UpgradeCategory::Attack,
            max_level: 5,
        },
        UpgradeDef {
            id: UpgradeId::RuneVetra,
            name: "Руна Ветра",
            description: "+6% скорость атаки",
            category: UpgradeCategory::Attack,
            max_level: 5,
        },
        UpgradeDef {
            id: UpgradeId::OberegSvaroga,
            name: "Оберег Сварога",
            description: "+10 HP + лечение",
            category: UpgradeCategory::Defense,
            max_level: 3,
        },
        UpgradeDef {
            id: UpgradeId::SlezaLady,
            name: "Слеза Лады",
            description: "+1 HP/сек",
            category: UpgradeCategory::Defense,
            max_level: 3,
        },
        UpgradeDef {
            id: UpgradeId::ZnakVolka,
            name: "Знак Волка",
            description: "+5% скорость",
            category: UpgradeCategory::Path,
            max_level: 5,
        },
    ]
}

/// Выбирает N случайных апгрейдов из доступных (не на максе)
pub fn pick_random_upgrades(inventory: &UpgradeInventory, count: usize) -> Vec<UpgradeId> {
    let all = all_upgrades();
    let available: Vec<&UpgradeDef> = all.iter()
        .filter(|def| inventory.get_level(&def.id) < def.max_level)
        .collect();

    if available.is_empty() {
        return vec![];
    }

    // Fisher-Yates shuffle с rand_01()
    let mut indices: Vec<usize> = (0..available.len()).collect();
    for i in (1..indices.len()).rev() {
        let j = (rand_01() * (i + 1) as f32) as usize;
        indices.swap(i, j.min(i));
    }

    indices.into_iter()
        .take(count.min(available.len()))
        .map(|i| available[i].id)
        .collect()
}

/// Применяет апгрейд к игроку
pub fn apply_upgrade(
    id: UpgradeId,
    inventory: &mut UpgradeInventory,
    weapon: &mut Weapon,
    player_health: &mut PlayerHealth,
    player_stats: &mut PlayerStats,
) {
    inventory.increment(id);

    match id {
        UpgradeId::RunePeruna => {
            // +5% base damage
            weapon.damage *= 1.05;
        }
        UpgradeId::RuneVetra => {
            // +6% attack speed (reduce cooldown)
            weapon.cooldown *= 0.94;
        }
        UpgradeId::OberegSvaroga => {
            // +10 max HP + instant heal 10
            player_health.max += 10.0;
            player_health.current = (player_health.current + 10.0).min(player_health.max);
        }
        UpgradeId::SlezaLady => {
            // HP regen обрабатывается отдельной системой hp_regen_system
        }
        UpgradeId::ZnakVolka => {
            // +5% movement speed
            player_stats.move_speed_multiplier *= 1.05;
        }
    }
}

/// Получает описание апгрейда по ID
pub fn get_upgrade_def(id: &UpgradeId) -> Option<UpgradeDef> {
    all_upgrades().into_iter().find(|def| def.id == *id)
}
