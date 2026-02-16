// Пути к ассетам игры

// Модель богатыря (merged GLB — все анимации в одном файле)
pub const BOGATYR_MODEL: &str = "models/characters/bogatyr/bogatyr_merged.glb#Scene0";

// Анимации богатыря (из единого merged GLB)
pub const ANIM_IDLE: &str = "models/characters/bogatyr/bogatyr_merged.glb#Animation1";    // Idle (4s)
pub const ANIM_WALK: &str = "models/characters/bogatyr/bogatyr_merged.glb#Animation2";    // walking_man
pub const ANIM_RUN: &str = "models/characters/bogatyr/bogatyr_merged.glb#Animation3";     // running
pub const ANIM_ATTACK: &str = "models/characters/bogatyr/bogatyr_merged.glb#Animation4";  // Attack
pub const ANIM_HIT: &str = "models/characters/bogatyr/bogatyr_merged.glb#Animation5";     // Hit Reaction

// Модель Упыря (merged GLB — все анимации в одном файле)
pub const UPYR_MODEL: &str = "models/enemies/upyr_merged.glb#Scene0";
pub const UPYR_ANIM_IDLE: &str = "models/enemies/upyr_merged.glb#Animation1";         // Idle
pub const UPYR_ANIM_WALK: &str = "models/enemies/upyr_merged.glb#Animation2";         // Monster_Walk
pub const UPYR_ANIM_ATTACK: &str = "models/enemies/upyr_merged.glb#Animation3";       // Attack
pub const UPYR_ANIM_DEATH: &str = "models/enemies/upyr_merged.glb#Animation4";        // Dead
pub const UPYR_ANIM_HIT: &str = "models/enemies/upyr_merged.glb#Animation5";          // Hit_Reaction
pub const UPYR_ANIM_RUN: &str = "models/enemies/upyr_merged.glb#Animation6";          // Running
pub const UPYR_ANIM_SCREAM: &str = "models/enemies/upyr_merged.glb#Animation7";       // Zombie_Scream

// Модель Лешего (merged GLB — все анимации в одном файле)
pub const LESHIY_MODEL: &str = "models/enemies/leshiy_merged.glb#Scene0";
pub const LESHIY_ANIM_IDLE: &str = "models/enemies/leshiy_merged.glb#Animation0";      // Idle
pub const LESHIY_ANIM_WALK: &str = "models/enemies/leshiy_merged.glb#Animation1";      // Walking
pub const LESHIY_ANIM_RUN: &str = "models/enemies/leshiy_merged.glb#Animation2";       // Running
pub const LESHIY_ANIM_ATTACK: &str = "models/enemies/leshiy_merged.glb#Animation3";    // Slash Attack
pub const LESHIY_ANIM_DEATH: &str = "models/enemies/leshiy_merged.glb#Animation4";     // Death
pub const LESHIY_ANIM_HIT: &str = "models/enemies/leshiy_merged.glb#Animation5";       // Hit Reaction

// Модель Волколака (Tripo GLB — 6 анимаций квадрупеда)
pub const VOLKOLAK_MODEL: &str = "models/enemies/volkolak_merged.glb#Scene0";
pub const VOLKOLAK_ANIM_IDLE: &str = "models/enemies/volkolak_merged.glb#Animation0";    // idle
pub const VOLKOLAK_ANIM_WALK: &str = "models/enemies/volkolak_merged.glb#Animation1";    // walk
pub const VOLKOLAK_ANIM_RUN: &str = "models/enemies/volkolak_merged.glb#Animation2";     // run
pub const VOLKOLAK_ANIM_ATTACK: &str = "models/enemies/volkolak_merged.glb#Animation3";  // attack (slash)
pub const VOLKOLAK_ANIM_HIT: &str = "models/enemies/volkolak_merged.glb#Animation4";     // hit (hurt)
pub const VOLKOLAK_ANIM_DEATH: &str = "models/enemies/volkolak_merged.glb#Animation5";   // death (fall)

// Окружение

// Скальные стены арены (Meshy GLB — 3 варианта для разнообразия)
pub const CLIFF_WALL_A: &str = "models/environment/cliff_wall_a.glb#Scene0";
pub const CLIFF_WALL_B: &str = "models/environment/cliff_wall_b.glb#Scene0";
pub const CLIFF_WALL_C: &str = "models/environment/cliff_wall_c.glb#Scene0";

// Порталы (Meshy GLB — Разломы Нави)
pub const PORTAL_FIRE: &str = "models/environment/portal_fire.glb#Scene0";
pub const PORTAL_DARK: &str = "models/environment/portal_dark.glb#Scene0";

// Пол арены — seamless PBR текстуры с Polyhaven (forrest_ground_01, 1K)
pub const FLOOR_DIFF: &str = "textures/grass_diff.jpg";
pub const FLOOR_NORMAL: &str = "textures/grass_nor.jpg";

// Декор арены (Meshy GLB)
pub const RUNE_STONE: &str = "models/environment/rune_stone.glb#Scene0";
pub const ROCK_LARGE: &str = "models/environment/rock_large.glb#Scene0";
pub const DEAD_TREE: &str = "models/environment/dead_tree.glb#Scene0";
pub const BONE_PILE: &str = "models/environment/bone_pile.glb#Scene0";

// Оружие
pub const SWORD_MODEL: &str = "models/weapons/runic_sword.glb#Scene0";

// Портреты персонажей (AI-generated, nano-banana)
pub const PORTRAIT_BOGATYR: &str = "textures/ui/portraits/bogatyr_portrait.jpg";
pub const PORTRAIT_UPYR: &str = "textures/ui/portraits/upyr_portrait.jpg";
pub const PORTRAIT_LESHIY: &str = "textures/ui/portraits/leshiy_portrait.jpg";
pub const PORTRAIT_VOLKOLAK: &str = "textures/ui/portraits/volkolak_portrait.jpg";

// UI фоны
pub const TITLE_BG: &str = "textures/ui/title_bg.jpg";
pub const GAMEOVER_BG: &str = "textures/ui/gameover_bg.jpg";

// UI текстуры (AI-generated, nano-banana + remove.bg)
pub const UI_HP_BAR_FRAME: &str = "ui/hp_bar_frame.png";
pub const UI_PANEL_BG: &str = "ui/panel_bg.png";
pub const UI_MINIMAP_FRAME: &str = "ui/minimap_frame.png";
pub const UI_XP_BAR_FRAME: &str = "ui/xp_bar_frame.png";
pub const UI_BUTTON_FRAME: &str = "ui/button_frame.png";

// Иконки статов (AI-generated, nano-banana)
pub const ICON_DAMAGE: &str = "ui/icons/icon_damage.png";
pub const ICON_SPEED: &str = "ui/icons/icon_speed.png";
pub const ICON_RANGE: &str = "ui/icons/icon_range.png";

// Шрифты (Google Fonts, OFL)
pub const FONT_TITLE: &str = "fonts/RuslanDisplay-Regular.ttf";      // Ruslan Display — заголовки (полуустав XVI век)
pub const FONT_UI: &str = "fonts/CormorantGaramond-Regular.otf";     // Cormorant Garamond — HUD/текст
pub const FONT_UI_BOLD: &str = "fonts/CormorantGaramond-Bold.otf";   // Cormorant Garamond Bold — кнопки
