//! Реализация игрового мира:
//! - setup_scene: арена 50x50м (пол с PBR, 40 стен с коллайдерами, 4 факела с огнём)
//! - torch_flicker: анимация мерцания факелов (наложенные синусоиды)
//! - ground_circle: кольца HP под сущностями (динамический меш-арка, поворот, пульсация)

pub(super) mod setup_scene;
pub(super) mod torch_flicker;
pub(super) mod portal_setup;
pub mod stylized_material;
pub mod ground_circle;
