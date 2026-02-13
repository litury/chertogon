pub mod world;
pub mod input;
pub mod player;
pub mod camera;
pub mod enemies;
pub mod combat;
pub mod menu;
pub mod selection;
pub mod progression;

// Реэкспорт публичных API
pub use world::WorldPlugin;
pub use input::{InputPlugin, InputState};
pub use player::{PlayerPlugin, Player, AnimatedCharacter, AnimationState, PlayerModel, AnimationSetupComplete};
pub use camera::{CameraPlugin, CameraTarget};
pub use enemies::{EnemiesPlugin, Enemy, Health, EnemyType};
pub use combat::{CombatPlugin, PlayerHealth};
pub use menu::MenuPlugin;
pub use selection::SelectionPlugin;
pub use progression::ProgressionPlugin;
