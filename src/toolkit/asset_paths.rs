// Пути к ассетам игры

// Модель богатыря (merged GLB — все анимации в одном файле)
pub const BOGATYR_MODEL: &str = "models/characters/bogatyr/bogatyr_merged.glb#Scene0";

// Анимации богатыря (из единого merged GLB)
pub const ANIM_IDLE: &str = "models/characters/bogatyr/bogatyr_merged.glb#Animation1";    // Idle (4s)
pub const ANIM_WALK: &str = "models/characters/bogatyr/bogatyr_merged.glb#Animation2";    // walking_man
pub const ANIM_RUN: &str = "models/characters/bogatyr/bogatyr_merged.glb#Animation3";     // running
pub const ANIM_ATTACK: &str = "models/characters/bogatyr/bogatyr_merged.glb#Animation4";  // Attack

// Модель Упыря (merged GLB — все анимации в одном файле)
pub const UPYR_MODEL: &str = "models/enemies/upyr_merged.glb#Scene0";
pub const UPYR_ANIM_IDLE: &str = "models/enemies/upyr_merged.glb#Animation1";         // Idle
pub const UPYR_ANIM_WALK: &str = "models/enemies/upyr_merged.glb#Animation2";         // Monster_Walk
pub const UPYR_ANIM_ATTACK: &str = "models/enemies/upyr_merged.glb#Animation3";       // Attack
pub const UPYR_ANIM_DEATH: &str = "models/enemies/upyr_merged.glb#Animation4";        // Dead

// Окружение

// Стены арены (Meshy GLB — тёмный готический камень с рунами)
pub const WALL_PANEL: &str = "models/environment/wall_panel.glb#Scene0";

// Факел (Meshy GLB — кованый кронштейн с пламенем)
pub const TORCH: &str = "models/environment/torch.glb#Scene0";

// Пол арены — seamless PBR текстуры с Polyhaven (stone_tiles, 1K)
pub const FLOOR_DIFF: &str = "textures/stone_floor_diff.jpg";
pub const FLOOR_NORMAL: &str = "textures/stone_floor_nor.jpg";
pub const FLOOR_ROUGH: &str = "textures/stone_floor_rough.jpg";
