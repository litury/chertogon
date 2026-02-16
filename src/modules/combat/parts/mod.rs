//! Реализация боевой системы:
//! - auto_attack: автоатака ближайшего врага, PendingAttack с задержкой на windup
//! - enemy_damage: контактный урон врагов по игроку
//! - camera_shake: тряска камеры при ударе (ресурс CameraShake)
//! - slash_vfx: огненная дуга (6-кадровая анимация billboard)
//! - hit_particles: искры при попадании (эмиссивные сферы с физикой)
//! - damage_numbers: всплывающие числа урона (Text2d)
//! - blood_decals: пятна крови на полу (текстурированные quad'ы)
//! - knockback: отбрасывание врага (компонент Staggered)
//! - hit_flash: импульс масштаба + эмиссивная вспышка модели при попадании
//! - damage_vignette: красный сдвиг экрана при уроне игроку (ColorGrading)
//! - game_over: проверка смерти игрока, fade → GameOver
//! - game_timer: таймер раунда (MM:SS), ресурс GameTimer

pub mod auto_attack;
pub mod enemy_damage;
pub mod camera_shake;
pub mod slash_vfx;
pub mod hit_particles;
pub mod game_over;
pub mod game_timer;
pub mod knockback;
pub mod hit_flash;
pub mod damage_numbers;
pub mod blood_decals;
pub mod impact_flash;
pub mod damage_vignette;
pub mod vfx_assets;
pub mod bloody_footprints;
pub mod haptic;
