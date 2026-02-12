// Константы скоростей движения

pub const WALK_SPEED: f32 = 5.0;
pub const RUN_SPEED: f32 = 10.0;

// Параметры камеры (оптимизировано для arena shooter)
pub const CAMERA_FOLLOW_SPEED: f32 = 15.0;  // Exponential decay rate (higher = snappier)
pub const CAMERA_OFFSET_Y: f32 = 16.0;  // Высота камеры (было 8.0, удвоено для лучшего обзора)
pub const CAMERA_OFFSET_Z: f32 = 14.0;  // Расстояние от игрока (было 10.0, увеличено)

// Параметры зума камеры (mouse wheel)
pub const CAMERA_ZOOM_MIN: f32 = 10.0;      // Минимальное расстояние (близко)
pub const CAMERA_ZOOM_MAX: f32 = 22.0;      // Максимальное расстояние (далеко)
pub const CAMERA_ZOOM_DEFAULT: f32 = 14.0;  // По умолчанию (текущее расстояние)
pub const CAMERA_ZOOM_SPEED: f32 = 1.5;     // Чувствительность зума
pub const CAMERA_ZOOM_SMOOTHNESS: f32 = 8.0; // Exponential decay rate for zoom

// Collision Layers для разделения физических объектов
use avian3d::prelude::*;

#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
pub enum GameLayer {
    #[default]
    Static,    // Стены, пол, статические объекты
    Player,    // Игрок
    Enemy,     // Враги
    Projectile // Пули/снаряды (для будущего расширения)
}

impl GameLayer {
    /// Возвращает CollisionLayers для статических объектов (стены, пол)
    /// Коллайдируют с: Player, Enemy, Projectile
    pub fn static_layers() -> CollisionLayers {
        CollisionLayers::new(
            [GameLayer::Static],
            [GameLayer::Player, GameLayer::Enemy, GameLayer::Projectile]
        )
    }

    /// Возвращает CollisionLayers для игрока
    /// Коллайдирует с: Static, Enemy
    pub fn player_layers() -> CollisionLayers {
        CollisionLayers::new(
            [GameLayer::Player],
            [GameLayer::Static, GameLayer::Enemy]
        )
    }

    /// Возвращает CollisionLayers для врагов
    /// Коллайдируют с: Static, Player, Enemy (враги не проходят друг через друга)
    pub fn enemy_layers() -> CollisionLayers {
        CollisionLayers::new(
            [GameLayer::Enemy],
            [GameLayer::Static, GameLayer::Player, GameLayer::Enemy]
        )
    }
}
