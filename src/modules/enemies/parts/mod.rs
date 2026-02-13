//! Реализация врагов:
//! - spawner: волновой спавнер (Cooldown → Spawning → Fighting), масштабирование по волнам
//! - ai: дистанционный AI (Idle/Chase/Attack), запуск смерти, конвертация в труп
//! - animation: привязка AnimationPlayer из GLB, переключение анимаций по состоянию
//! - cleanup: деспавн врагов и трупов, сброс волн и kill count

pub mod spawner;
pub mod ai;
pub mod animation;
pub mod cleanup;
pub mod preload;
