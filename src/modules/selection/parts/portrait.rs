use crate::modules::enemies::components::EnemyType;
use crate::toolkit::asset_paths;

/// Возвращает путь к портрету по типу врага
pub fn portrait_for_enemy(enemy_type: &EnemyType) -> &'static str {
    match enemy_type {
        EnemyType::Upyr => asset_paths::PORTRAIT_UPYR,
        EnemyType::Leshiy => asset_paths::PORTRAIT_LESHIY,
        EnemyType::Volkolak => asset_paths::PORTRAIT_VOLKOLAK,
    }
}

/// Возвращает путь к портрету игрока
pub fn portrait_for_player() -> &'static str {
    asset_paths::PORTRAIT_BOGATYR
}
