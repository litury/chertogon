pub mod components;
pub mod plugin;
pub(crate) mod parts;

pub use plugin::CombatPlugin;
pub use components::{Weapon, AttackCooldown, PlayerHealth, EnemyAttackCooldown};
pub use parts::camera_shake::CameraShake;
