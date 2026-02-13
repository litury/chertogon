use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy::asset::RenderAssetUsages;
use bevy::camera::{ImageRenderTarget, RenderTarget};
use bevy::camera::visibility::RenderLayers;
use std::time::Duration;
use crate::modules::enemies::components::EnemyType;
use crate::modules::selection::components::*;
use crate::toolkit::asset_paths;

const PORTRAIT_POS: Vec3 = Vec3::new(0.0, -500.0, 0.0);
const PORTRAIT_SIZE: u32 = 128;

/// Маркер: портрет ждёт настройки анимации
#[derive(Component)]
pub struct PortraitAnimPending;

/// Создаёт render target (Image 128×128) + камеру портрета + свет.
/// Камера выключена по умолчанию — включается при выделении.
pub fn setup_portrait_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // Render target — прозрачная текстура
    let size = Extent3d {
        width: PORTRAIT_SIZE,
        height: PORTRAIT_SIZE,
        depth_or_array_layers: 1,
    };
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING
        | TextureUsages::COPY_DST
        | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);
    commands.insert_resource(PortraitRenderTarget(image_handle.clone()));

    // Камера портрета
    commands.spawn((
        PortraitCamera,
        Camera3d::default(),
        Camera {
            is_active: false,
            order: -1,
            clear_color: ClearColorConfig::Custom(Color::srgba(0.06, 0.04, 0.1, 1.0)),
            ..default()
        },
        RenderTarget::Image(ImageRenderTarget {
            handle: image_handle,
            scale_factor: 1.0,
        }),
        Transform::from_translation(PORTRAIT_POS + Vec3::new(0.0, 1.0, 2.5))
            .looking_at(PORTRAIT_POS + Vec3::Y * 0.6, Vec3::Y),
        RenderLayers::layer(1),
    ));

    // Свет для портретной сцены
    commands.spawn((
        PortraitScene,
        PointLight {
            color: Color::srgb(1.0, 0.85, 0.7),
            intensity: 3000.0,
            range: 10.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_translation(PORTRAIT_POS + Vec3::new(1.0, 2.0, 2.0)),
        RenderLayers::layer(1),
    ));

    // Заполняющий свет (мягче, с другой стороны)
    commands.spawn((
        PortraitScene,
        PointLight {
            color: Color::srgb(0.4, 0.5, 0.8),
            intensity: 1000.0,
            range: 10.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_translation(PORTRAIT_POS + Vec3::new(-1.5, 1.0, 1.0)),
        RenderLayers::layer(1),
    ));
}

/// Обновляет 3D модель портрета при смене выделения.
pub fn update_portrait_model(
    selection: Res<SelectionState>,
    enemies: Query<&EnemyType>,
    portrait_models: Query<Entity, With<PortraitScene>>,
    mut portrait_camera: Query<&mut Camera, With<PortraitCamera>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    if !selection.is_changed() {
        return;
    }

    // Удаляем старую модель портрета (но не свет — он постоянный, отличаем по наличию PointLight)
    for entity in &portrait_models {
        // Не удаляем сущности со светом — удаляем только модели
        commands.entity(entity).despawn();
    }

    let Some(selected) = selection.selected_entity else {
        // Нет выделения — выключаем камеру
        if let Ok(mut cam) = portrait_camera.single_mut() {
            cam.is_active = false;
        }
        return;
    };

    let Ok(enemy_type) = enemies.get(selected) else {
        if let Ok(mut cam) = portrait_camera.single_mut() {
            cam.is_active = false;
        }
        return;
    };

    // Включаем камеру
    if let Ok(mut cam) = portrait_camera.single_mut() {
        cam.is_active = true;
    }

    // Определяем модель и idle-анимацию по типу врага
    let (model_path, idle_anim_path, y_offset) = match enemy_type {
        EnemyType::Upyr => (asset_paths::UPYR_MODEL, asset_paths::UPYR_ANIM_IDLE, -0.9),
        EnemyType::Leshiy => (asset_paths::LESHIY_MODEL, asset_paths::LESHIY_ANIM_IDLE, -0.8),
        EnemyType::Volkolak => (asset_paths::VOLKOLAK_MODEL, asset_paths::VOLKOLAK_ANIM_IDLE, -0.3),
    };

    // Создаём AnimationGraph с idle-клипом
    let mut animation_graph = AnimationGraph::new();
    let idle_handle = asset_server.load(idle_anim_path);
    let idle_index = animation_graph.add_clip(idle_handle, 1.0, animation_graph.root);
    let graph_handle = graphs.add(animation_graph);

    // Спавним модель портрета
    commands.spawn((
        PortraitScene,
        PortraitAnimPending,
        SceneRoot(asset_server.load(model_path)),
        Transform::from_translation(PORTRAIT_POS + Vec3::new(0.0, y_offset, 0.0))
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_6)),
        AnimationGraphHandle(graph_handle),
        RenderLayers::layer(1),
        PortraitIdleIndex(idle_index),
    ));
}

/// Хранит индекс idle-анимации для портрета
#[derive(Component)]
pub struct PortraitIdleIndex(pub AnimationNodeIndex);

/// Настраивает анимацию портрета после загрузки сцены.
pub fn setup_portrait_animation(
    portrait_models: Query<(Entity, &Children, &PortraitIdleIndex), With<PortraitAnimPending>>,
    children_query: Query<&Children>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut commands: Commands,
) {
    for (model_entity, model_children, idle_index) in &portrait_models {
        // Ищем AnimationPlayer в потомках (может быть на 1-2 уровня глубже)
        for child in model_children.iter() {
            if try_start_portrait_anim(child, idle_index.0, &mut animation_players, &mut commands) {
                commands.entity(model_entity).remove::<PortraitAnimPending>();
                return;
            }
            if let Ok(grandchildren) = children_query.get(child) {
                for gc in grandchildren.iter() {
                    if try_start_portrait_anim(gc, idle_index.0, &mut animation_players, &mut commands) {
                        commands.entity(model_entity).remove::<PortraitAnimPending>();
                        return;
                    }
                }
            }
        }
    }
}

fn try_start_portrait_anim(
    entity: Entity,
    idle_index: AnimationNodeIndex,
    animation_players: &mut Query<&mut AnimationPlayer>,
    commands: &mut Commands,
) -> bool {
    let Ok(mut player) = animation_players.get_mut(entity) else {
        return false;
    };

    let mut transitions = AnimationTransitions::new();
    transitions
        .play(&mut player, idle_index, Duration::ZERO)
        .repeat();
    commands.entity(entity).insert(transitions);
    true
}

/// Распространяет RenderLayers::layer(1) на всех потомков портретной сцены.
/// Без этого модели из SceneRoot рендерятся на layer 0 (видимы основной камере).
pub fn propagate_portrait_layers(
    portrait_roots: Query<Entity, With<PortraitScene>>,
    children_query: Query<&Children>,
    has_layers: Query<(), With<RenderLayers>>,
    mut commands: Commands,
) {
    for root in &portrait_roots {
        propagate_recursive(root, &children_query, &has_layers, &mut commands);
    }
}

fn propagate_recursive(
    entity: Entity,
    children_query: &Query<&Children>,
    has_layers: &Query<(), With<RenderLayers>>,
    commands: &mut Commands,
) {
    let Ok(children) = children_query.get(entity) else { return };

    for child in children.iter() {
        if has_layers.get(child).is_err() {
            commands.entity(child).insert(RenderLayers::layer(1));
        }
        propagate_recursive(child, children_query, has_layers, commands);
    }
}

/// Чистит всю портретную систему при выходе из Playing.
pub fn cleanup_portrait(
    mut commands: Commands,
    portrait_entities: Query<Entity, Or<(With<PortraitScene>, With<PortraitCamera>)>>,
) {
    for entity in &portrait_entities {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<PortraitRenderTarget>();
}
