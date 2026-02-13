pub mod plugin;
pub mod components;
pub(crate) mod parts;

pub use plugin::PlayerPlugin;
pub use components::{Player, AnimatedCharacter, AnimationState, PlayerAnimations, PlayerModel, AnimationSetupComplete, PlayerHitStagger, StaggerCooldown, WeaponModel, WeaponAttachmentComplete, PlayerStats};
