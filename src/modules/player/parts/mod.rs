//! Реализация игрока:
//! - spawner: спавн с AnimationGraph и Avian3D физикой, привязка AnimationPlayer
//! - movement: перемещение через LinearVelocity по InputState, поворот модели
//! - animation: переключение idle/walk/run с гистерезисом и плавным блендингом (200мс)
//! - weapon_attachment: поиск кости "RightHand" в скелете, спавн меча как дочерней сущности
//! - cleanup: деспавн игрока, сброс InputState

pub(super) mod spawner;
pub(crate) mod movement;
pub(super) mod animation;
pub(super) mod cleanup;
pub(super) mod weapon_attachment;
