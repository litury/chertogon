use bevy::prelude::*;
use avian3d::prelude::*;
use crate::modules::enemies::components::*;
use crate::modules::combat::components::EnemyAttackCooldown;
use crate::toolkit::asset_paths;

/// Временный компонент для передачи индексов анимаций от spawn к setup
#[derive(Component, Clone, Copy)]
pub struct EnemyAnimationIndices {
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub attack: AnimationNodeIndex,
    pub death: AnimationNodeIndex,
}

/// Спавнит тестового Упыря (merged GLB с анимациями из Meshy)
pub fn spawn_test_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    info!("👾 Spawning Упырь (Upyr) with animations");

    let spawn_pos = Vec3::new(0.0, 0.0, 8.0);

    // Создаём AnimationGraph с 4 анимациями
    let mut animation_graph = AnimationGraph::new();

    let idle_handle = asset_server.load(asset_paths::UPYR_ANIM_IDLE);
    let walk_handle = asset_server.load(asset_paths::UPYR_ANIM_WALK);
    let attack_handle = asset_server.load(asset_paths::UPYR_ANIM_ATTACK);
    let death_handle = asset_server.load(asset_paths::UPYR_ANIM_DEATH);

    let idle_index = animation_graph.add_clip(idle_handle, 1.0, animation_graph.root);
    let walk_index = animation_graph.add_clip(walk_handle, 1.0, animation_graph.root);
    let attack_index = animation_graph.add_clip(attack_handle, 1.0, animation_graph.root);
    let death_index = animation_graph.add_clip(death_handle, 1.0, animation_graph.root);

    let graph_handle = graphs.add(animation_graph);

    // Parent entity: логика + физика
    let enemy_entity = commands.spawn((
        Enemy,
        EnemyType::Upyr,
        Health::new(20.0),
        ChasePlayer {
            speed: 3.0,
            aggro_range: 12.0,
            attack_range: 2.0,
        },
        EnemyAnimState { current: EnemyAnim::Idle },
        Transform::from_translation(spawn_pos),
        RigidBody::Dynamic,
        Collider::cylinder(0.5, 1.8),
        LinearVelocity::default(),
        LinearDamping(8.0),
        AngularDamping(8.0),
        crate::shared::GameLayer::enemy_layers(),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z(),
        // Combat: контактный урон 5.0 каждую 1.0с
        EnemyAttackCooldown::new(5.0, 1.0),
    )).id();

    // Child entity: визуальная модель + AnimationGraph
    let scene = asset_server.load(asset_paths::UPYR_MODEL);
    let model_child = commands.spawn((
        SceneRoot(scene),
        Transform::from_xyz(0.0, -0.9, 0.0),
        EnemyModel,
        EnemyAnimationIndices {
            idle: idle_index,
            walk: walk_index,
            attack: attack_index,
            death: death_index,
        },
        AnimationGraphHandle(graph_handle),
    )).id();

    commands.entity(enemy_entity).add_child(model_child);

    info!("✅ Упырь spawned at {:?} with 4 animations (idle, walk, attack, death)", spawn_pos);
}
