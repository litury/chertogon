//! Реализация игрового мира:
//! - setup_scene: лесная арена 50x50м (трава, скалы, рунные камни, декор)
//! - ground_circle: кольца HP под сущностями (динамический меш-арка, поворот, пульсация)

pub(super) mod setup_scene;
pub(super) mod portal_setup;
pub(super) mod portal_fill;
pub mod stylized_material;
pub mod ground_circle;
pub mod range_indicator;
