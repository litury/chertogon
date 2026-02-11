pub mod components;
pub mod plugin;
mod parts;

// Публичный API
pub use plugin::EnemiesPlugin;
pub use components::{Enemy, Health, EnemyType, ChasePlayer, EnemyModel, EnemyAnimations, EnemyAnimationSetupComplete, EnemyAnimState, EnemyAnim, EnemyDying};
