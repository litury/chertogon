use bevy::prelude::*;

/// Маркер выделенного врага
#[derive(Component)]
pub struct Selected;

/// Текущее состояние выделения
#[derive(Resource, Default)]
pub struct SelectionState {
    pub selected_entity: Option<Entity>,
}

/// Сообщение тапа/клика по экрану (для выделения)
#[derive(Message)]
pub struct SelectionTapEvent {
    pub screen_pos: Vec2,
}

// === UI маркеры ===

/// Корень панели выделения (для bulk despawn)
#[derive(Component)]
pub struct SelectionPanelUI;

/// Текст HP в панели (живое обновление)
#[derive(Component)]
pub struct SelectionHpText;

/// Нода заполнения HP-бара (ширина меняется)
#[derive(Component)]
pub struct SelectionHpFill;

/// Портретное изображение в панели
#[derive(Component)]
pub struct SelectionPortrait;

// === Портретная система ===

/// Маркер сущностей портретной сцены (модель + свет)
#[derive(Component)]
pub struct PortraitScene;

/// Маркер камеры портрета
#[derive(Component)]
pub struct PortraitCamera;

/// Render target портрета (создаётся при входе в Playing)
#[derive(Resource)]
pub struct PortraitRenderTarget(pub Handle<Image>);
