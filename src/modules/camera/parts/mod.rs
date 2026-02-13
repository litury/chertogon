//! Реализация камеры:
//! - setup: спавн 3D камеры с FXAA, Bloom и distance fog
//! - follow_system: следование за игроком с зумом (mouse wheel) и camera shake
//! - menu_camera: орбитальное вращение камеры на title screen

pub(super) mod setup;
pub(super) mod follow_system;
pub(super) mod menu_camera;
