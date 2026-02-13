use bevy::prelude::*;
use crate::shared::GameState;
use super::parts::{orb_assets, orbs, level_up, level_up_ui, hp_regen};
use super::components::{PlayerXp, UpgradeInventory, LevelUpState};

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerXp>()
            .init_resource::<UpgradeInventory>()
            .init_resource::<LevelUpState>()
            // Инициализация при входе в Playing
            .add_systems(OnEnter(GameState::Playing), (
                orb_assets::init_orb_assets,
                orbs::reset_player_xp,
                orbs::cleanup_orbs,
                level_up_ui::cleanup_level_up_ui,
            ))
            // Основные системы (работают в Playing)
            .add_systems(Update, (
                orbs::spawn_orbs_on_enemy_death,
                orbs::xp_orb_physics_system,
                orbs::hp_orb_physics_system,
                level_up::check_level_up_system,
                hp_regen::hp_regen_system,
            ).chain().run_if(in_state(GameState::Playing)))
            // Level-up UI (работает даже на паузе — виртуальное время на паузе, но Update крутится)
            .add_systems(Update, (
                level_up_ui::spawn_level_up_ui,
                level_up_ui::level_up_interaction_system,
                level_up_ui::card_hover_system,
            ).run_if(in_state(GameState::Playing)));

        info!("📊 ProgressionPlugin loaded (XP orbs, level-up, upgrades)");
    }
}
