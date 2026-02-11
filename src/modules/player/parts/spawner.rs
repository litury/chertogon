use bevy::prelude::*;
use std::time::Duration;
use avian3d::prelude::*;  // ✅ Добавляем импорт физики (RigidBody, Collider)
use crate::modules::player::components::{Player, AnimatedCharacter, AnimationState, PlayerAnimations, PlayerModel, AnimationSetupComplete};
use crate::modules::combat::components::{Weapon, AttackCooldown, PlayerHealth};
use crate::toolkit::asset_paths;

/// Временный компонент для передачи индексов анимаций от spawn к setup
/// Удаляется после настройки AnimationPlayer
#[derive(Component, Clone, Copy)]
pub struct AnimationIndices {
    idle: AnimationNodeIndex,
    walk: AnimationNodeIndex,
    run: AnimationNodeIndex,
    attack: AnimationNodeIndex,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    info!("🎮 Spawning player with animations from {}", asset_paths::BOGATYR_MODEL);

    // Загружаем модель богатыря
    let scene = asset_server.load(asset_paths::BOGATYR_MODEL);

    // Создаем AnimationGraph
    let mut animation_graph = AnimationGraph::new();

    // Загружаем анимации (каждая из отдельного GLB)
    let idle_handle = asset_server.load(asset_paths::ANIM_IDLE);
    let walk_handle = asset_server.load(asset_paths::ANIM_WALK);
    let run_handle = asset_server.load(asset_paths::ANIM_RUN);
    let attack_handle = asset_server.load(asset_paths::ANIM_ATTACK);

    // Добавляем каждую анимацию в граф и получаем индексы
    let idle_index = animation_graph.add_clip(idle_handle, 1.0, animation_graph.root);
    let walk_index = animation_graph.add_clip(walk_handle, 1.0, animation_graph.root);
    let run_index = animation_graph.add_clip(run_handle, 1.0, animation_graph.root);
    let attack_index = animation_graph.add_clip(attack_handle, 1.0, animation_graph.root);

    // Сохраняем граф
    let graph_handle = graphs.add(animation_graph);

    info!("📊 AnimationGraph created with 4 animation nodes");
    info!("  - Idle: {}", asset_paths::ANIM_IDLE);
    info!("  - Walk: {}", asset_paths::ANIM_WALK);
    info!("  - Run: {}", asset_paths::ANIM_RUN);
    info!("  - Attack: {}", asset_paths::ANIM_ATTACK);

    // Создаем ЛОГИЧЕСКИЙ Player entity (без mesh) + ФИЗИКА
    let player_entity = commands.spawn((
        Transform::from_xyz(0.0, 0.9, 0.0),  // ✅ Y = 0.9 (половина высоты 1.8м) - стоит на полу
        Player,
        AnimatedCharacter {
            current_animation: AnimationState::Idle,
        },
        RigidBody::Dynamic,  // ✅ Dynamic = сталкивается со Static и другими Dynamic
        Collider::cylinder(0.5, 1.8),  // ✅ Цилиндр: радиус 0.5м, высота 1.8м (человек)
        LinearVelocity::default(),  // ✅ Для плавного движения через физику
        LinearDamping(10.0),  // ✅ Моментальная остановка без инерции
        AngularDamping(10.0),  // ✅ Предотвращает вращение от столкновений
        crate::shared::GameLayer::player_layers(), // ✅ Collision layers для игрока
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()   // ✅ Коллизии не вращают тело — визуальный поворот через PlayerModel child
            .lock_rotation_z(),
        // Combat
        Weapon::default(),
        AttackCooldown::new(1.0),
        PlayerHealth::new(100.0),
    )).id();

    // Создаем ВИЗУАЛЬНЫЙ child с SceneRoot
    let model_child = commands.spawn((
        SceneRoot(scene),
        Transform::from_xyz(0.0, -0.9, 0.0),  // ✅ Опускаем модель вниз - ноги на полу
        PlayerModel,
        // Временный компонент для передачи индексов в setup_scene_animation
        AnimationIndices {
            idle: idle_index,
            walk: walk_index,
            run: run_index,
            attack: attack_index,
        },
        AnimationGraphHandle(graph_handle),
    )).id();

    // Связываем parent-child
    commands.entity(player_entity).add_child(model_child);

    info!("✅ Created Player entity with PlayerModel child");
}

/// Система настройки AnimationPlayer после загрузки GLB
/// Выполняется КАЖДЫЙ КАДР пока AnimationPlayer не будет найден и настроен
pub fn setup_scene_animation(
    player: Query<&Children, With<Player>>,
    model_query: Query<(&Children, &AnimationIndices, &AnimationGraphHandle), With<PlayerModel>>,
    mut animation_players: Query<
        (Entity, &mut AnimationPlayer),
        (Without<AnimationSetupComplete>, Without<PlayerModel>)
    >,
    children: Query<&Children>,
    mut commands: Commands,
) {
    // Обходим Player -> PlayerModel -> AnimationPlayer
    for player_children in &player {
        for &model_child in player_children {
            if let Ok((model_children, anim_indices, graph_handle)) = model_query.get(model_child) {

                // Helper для настройки entity
                let setup_entity = |entity: Entity,
                                   player: &mut AnimationPlayer,
                                   commands: &mut Commands| {
                    info!("✅ Found AnimationPlayer in GLB hierarchy!");

                    // Конвертируем временные индексы в PlayerAnimations
                    let animations = PlayerAnimations {
                        idle: anim_indices.idle,
                        walk: anim_indices.walk,
                        run: anim_indices.run,
                        attack: anim_indices.attack,
                    };

                    // Добавляем компоненты ТОЛЬКО к AnimationPlayer entity
                    commands.entity(entity).insert(animations);
                    commands.entity(entity).insert(graph_handle.clone());

                    // Создаем transitions и запускаем Idle
                    let mut transitions = AnimationTransitions::new();
                    transitions
                        .play(player, animations.idle, Duration::ZERO)
                        .repeat();

                    commands.entity(entity).insert(transitions);

                    // Маркер завершения (предотвращает повторное выполнение)
                    commands.entity(entity).insert(AnimationSetupComplete);

                    // Удаляем временный компонент
                    commands.entity(model_child).remove::<AnimationIndices>();

                    info!("🎬 Animation system initialized successfully!");
                };

                // Ищем в direct children
                for &child in model_children {
                    if let Ok((entity, mut player)) = animation_players.get_mut(child) {
                        setup_entity(entity, &mut player, &mut commands);
                        return;
                    }

                    // Ищем в grandchildren
                    if let Ok(grandchildren) = children.get(child) {
                        for &grandchild in grandchildren {
                            if let Ok((entity, mut player)) = animation_players.get_mut(grandchild) {
                                setup_entity(entity, &mut player, &mut commands);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}
