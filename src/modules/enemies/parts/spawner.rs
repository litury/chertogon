use bevy::prelude::*;
use avian3d::prelude::*;
use crate::modules::enemies::components::*;
use crate::modules::combat::components::EnemyAttackCooldown;
use crate::modules::world::GroundCircle;
use crate::modules::enemies::components::PortalSpawnAnim;
use crate::shared::constants::{PORTAL_1_SPAWN, PORTAL_2_SPAWN};
use crate::toolkit::asset_paths;
use crate::shared::rand_01;

/// Временный компонент для передачи индексов анимаций от spawn к setup
#[derive(Component, Clone, Copy)]
pub struct EnemyAnimationIndices {
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub run: AnimationNodeIndex,
    pub attack: AnimationNodeIndex,
    pub death: AnimationNodeIndex,
    pub hit: AnimationNodeIndex,
    pub scream: AnimationNodeIndex,
}

/// Спавнит одного Упыря в указанной позиции
fn spawn_upyr_at(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_pos: Vec3,
) {
    // AnimationGraph с 5 анимациями
    let mut animation_graph = AnimationGraph::new();

    let idle_handle = asset_server.load(asset_paths::UPYR_ANIM_IDLE);
    let walk_handle = asset_server.load(asset_paths::UPYR_ANIM_WALK);
    let attack_handle = asset_server.load(asset_paths::UPYR_ANIM_ATTACK);
    let death_handle = asset_server.load(asset_paths::UPYR_ANIM_DEATH);
    let hit_handle = asset_server.load(asset_paths::UPYR_ANIM_HIT);
    let run_handle = asset_server.load(asset_paths::UPYR_ANIM_RUN);
    let scream_handle = asset_server.load(asset_paths::UPYR_ANIM_SCREAM);

    let idle_index = animation_graph.add_clip(idle_handle, 1.0, animation_graph.root);
    let walk_index = animation_graph.add_clip(walk_handle, 1.0, animation_graph.root);
    let run_index = animation_graph.add_clip(run_handle, 1.0, animation_graph.root);
    let attack_index = animation_graph.add_clip(attack_handle, 1.0, animation_graph.root);
    let death_index = animation_graph.add_clip(death_handle, 1.0, animation_graph.root);
    let hit_index = animation_graph.add_clip(hit_handle, 1.0, animation_graph.root);
    let scream_index = animation_graph.add_clip(scream_handle, 1.0, animation_graph.root);

    let graph_handle = graphs.add(animation_graph);

    // Parent entity: логика + физика (split spawn + insert из-за ограничения Bundle на 15 элементов)
    let enemy_entity = commands.spawn((
        Enemy,
        EnemyType::Upyr,
        Health::new(20.0),
        ChasePlayer {
            speed: 3.0,
            aggro_range: 12.0,
            attack_range: 1.5,
        },
        OrbitDirection {
            clockwise: rand_01() > 0.5,
            change_timer: Timer::from_seconds(3.0 + rand_01() * 3.0, TimerMode::Repeating),
        },
        EnemyAnimState { current: EnemyAnim::Screaming },
        SpawnScream { timer: Timer::from_seconds(1.5, TimerMode::Once) },
        PortalSpawnAnim::new(),
        Transform::from_translation(spawn_pos).with_scale(Vec3::splat(0.01)),
        RigidBody::Dynamic,
        Collider::cylinder(0.5, 1.8),
        LinearVelocity::default(),
        LinearDamping(12.0),
        AngularDamping(8.0),
    )).insert((
        crate::shared::GameLayer::enemy_layers(),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z(),
        EnemyAttackCooldown::new(5.0, 1.0, 1.5),
    )).id();

    // Child: визуальная модель + AnimationGraph
    let scene = asset_server.load(asset_paths::UPYR_MODEL);
    let model_child = commands.spawn((
        SceneRoot(scene),
        Transform::from_xyz(0.0, -0.9, 0.0),
        EnemyModel,
        EnemyAnimationIndices {
            idle: idle_index,
            walk: walk_index,
            run: run_index,
            attack: attack_index,
            death: death_index,
            hit: hit_index,
            scream: scream_index,
        },
        AnimationGraphHandle(graph_handle),
    )).id();

    // Ground ring — багровая HP-дуга
    let ring_mesh = meshes.add(Annulus::new(0.45, 0.6)); // Будет заменён на arc в первом кадре
    let ring_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.8, 0.1, 0.05, 0.45),
        emissive: LinearRgba::new(0.8, 0.1, 0.0, 0.0) * 1.5,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let ground_circle = commands.spawn((
        Mesh3d(ring_mesh),
        MeshMaterial3d(ring_material.clone()),
        Transform::from_xyz(0.0, -0.89, 0.0)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        GroundCircle {
            inner_radius: 0.45,
            outer_radius: 0.6,
            base_alpha: 0.45,
            pulse_speed: 3.0,
            material_handle: ring_material,
            last_hp_fraction: -1.0,
            last_facing: 0.0,
            last_alpha: 0.0,
        },
    )).id();

    commands.entity(enemy_entity).add_child(model_child);
    commands.entity(enemy_entity).add_child(ground_circle);
}

/// Спавнит одного Лешего в указанной позиции
fn spawn_leshiy_at(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_pos: Vec3,
) {
    let mut animation_graph = AnimationGraph::new();

    let idle_handle = asset_server.load(asset_paths::LESHIY_ANIM_IDLE);
    let walk_handle = asset_server.load(asset_paths::LESHIY_ANIM_WALK);
    let run_handle = asset_server.load(asset_paths::LESHIY_ANIM_RUN);
    let attack_handle = asset_server.load(asset_paths::LESHIY_ANIM_ATTACK);
    let death_handle = asset_server.load(asset_paths::LESHIY_ANIM_DEATH);
    let hit_handle = asset_server.load(asset_paths::LESHIY_ANIM_HIT);

    let idle_index = animation_graph.add_clip(idle_handle, 1.0, animation_graph.root);
    let walk_index = animation_graph.add_clip(walk_handle, 1.0, animation_graph.root);
    let run_index = animation_graph.add_clip(run_handle, 1.0, animation_graph.root);
    let attack_index = animation_graph.add_clip(attack_handle, 1.0, animation_graph.root);
    let death_index = animation_graph.add_clip(death_handle, 1.0, animation_graph.root);
    let hit_index = animation_graph.add_clip(hit_handle, 1.0, animation_graph.root);

    let graph_handle = graphs.add(animation_graph);

    // Леший: HP 15, speed 6.0, damage 8, aggro 15м, attack 2.5м
    // split spawn + insert из-за ограничения Bundle на 15 элементов
    let enemy_entity = commands.spawn((
        Enemy,
        EnemyType::Leshiy,
        Health::new(15.0),
        ChasePlayer {
            speed: 6.0,
            aggro_range: 15.0,
            attack_range: 1.8,
        },
        OrbitDirection {
            clockwise: rand_01() > 0.5,
            change_timer: Timer::from_seconds(3.0 + rand_01() * 3.0, TimerMode::Repeating),
        },
        EnemyAnimState { current: EnemyAnim::Idle },
        PortalSpawnAnim::new(),
        Transform::from_translation(spawn_pos).with_scale(Vec3::splat(0.01)),
        RigidBody::Dynamic,
        Collider::cylinder(0.4, 1.6),
        LinearVelocity::default(),
        LinearDamping(12.0),
        AngularDamping(8.0),
    )).insert((
        crate::shared::GameLayer::enemy_layers(),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z(),
        EnemyAttackCooldown::new(8.0, 0.8, 1.8),
    )).id();

    let scene = asset_server.load(asset_paths::LESHIY_MODEL);
    let model_child = commands.spawn((
        SceneRoot(scene),
        Transform::from_xyz(0.0, -0.8, 0.0),
        EnemyModel,
        EnemyAnimationIndices {
            idle: idle_index,
            walk: walk_index,
            run: run_index,
            attack: attack_index,
            death: death_index,
            hit: hit_index,
            scream: idle_index, // Леший не кричит при спавне
        },
        AnimationGraphHandle(graph_handle),
    )).id();

    // Ground ring — зелёная HP-дуга (отличается от красной у Упыря)
    let ring_mesh = meshes.add(Annulus::new(0.35, 0.5));
    let ring_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.1, 0.7, 0.15, 0.45),
        emissive: LinearRgba::new(0.1, 0.7, 0.0, 0.0) * 1.5,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let ground_circle = commands.spawn((
        Mesh3d(ring_mesh),
        MeshMaterial3d(ring_material.clone()),
        Transform::from_xyz(0.0, -0.79, 0.0)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        GroundCircle {
            inner_radius: 0.35,
            outer_radius: 0.5,
            base_alpha: 0.45,
            pulse_speed: 4.0,
            material_handle: ring_material,
            last_hp_fraction: -1.0,
            last_facing: 0.0,
            last_alpha: 0.0,
        },
    )).id();

    commands.entity(enemy_entity).add_child(model_child);
    commands.entity(enemy_entity).add_child(ground_circle);
}

/// Спавнит одного Волколака в указанной позиции
fn spawn_volkolak_at(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_pos: Vec3,
) {
    let mut animation_graph = AnimationGraph::new();

    let idle_handle = asset_server.load(asset_paths::VOLKOLAK_ANIM_IDLE);
    let walk_handle = asset_server.load(asset_paths::VOLKOLAK_ANIM_WALK);
    let run_handle = asset_server.load(asset_paths::VOLKOLAK_ANIM_RUN);
    let attack_handle = asset_server.load(asset_paths::VOLKOLAK_ANIM_ATTACK);
    let hit_handle = asset_server.load(asset_paths::VOLKOLAK_ANIM_HIT);
    let death_handle = asset_server.load(asset_paths::VOLKOLAK_ANIM_DEATH);

    let idle_index = animation_graph.add_clip(idle_handle.clone(), 1.0, animation_graph.root);
    let walk_index = animation_graph.add_clip(walk_handle, 1.0, animation_graph.root);
    let run_index = animation_graph.add_clip(run_handle, 1.0, animation_graph.root);
    let attack_index = animation_graph.add_clip(attack_handle, 1.0, animation_graph.root);
    let death_index = animation_graph.add_clip(death_handle, 1.0, animation_graph.root);
    let hit_index = animation_graph.add_clip(hit_handle, 1.0, animation_graph.root);
    let scream_index = animation_graph.add_clip(idle_handle, 0.5, animation_graph.root); // нет отдельного крика

    let graph_handle = graphs.add(animation_graph);

    // Волколак: HP 12, speed 7.0, damage 12, aggro 20м, attack 1.8м
    // split spawn + insert из-за ограничения Bundle на 15 элементов
    let enemy_entity = commands.spawn((
        Enemy,
        EnemyType::Volkolak,
        Health::new(12.0),
        ChasePlayer {
            speed: 7.0,
            aggro_range: 20.0,
            attack_range: 1.3,
        },
        OrbitDirection {
            clockwise: rand_01() > 0.5,
            change_timer: Timer::from_seconds(3.0 + rand_01() * 3.0, TimerMode::Repeating),
        },
        EnemyAnimState { current: EnemyAnim::Idle },
        PortalSpawnAnim::new(),
        Transform::from_translation(spawn_pos).with_scale(Vec3::splat(0.01)),
        RigidBody::Dynamic,
        Collider::cylinder(0.8, 1.0),
        LinearVelocity::default(),
        LinearDamping(12.0),
        AngularDamping(8.0),
    )).insert((
        crate::shared::GameLayer::enemy_layers(),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z(),
        EnemyAttackCooldown::new(6.0, 0.8, 1.3),  // 7.5 DPS (между Упырём 5 и Лешим 10)
    )).id();

    let scene = asset_server.load(asset_paths::VOLKOLAK_MODEL);
    let model_child = commands.spawn((
        SceneRoot(scene),
        Transform::from_xyz(0.0, -0.9, 0.0)
            .with_scale(Vec3::splat(1.0)),  // Нормализованная модель: ~1.4 единицы высоты
        EnemyModel,
        EnemyAnimationIndices {
            idle: idle_index,
            walk: walk_index,
            run: run_index,
            attack: attack_index,
            death: death_index,
            hit: hit_index,
            scream: scream_index,
        },
        AnimationGraphHandle(graph_handle),
    )).id();

    // Ground ring — серебристо-серая HP-дуга
    let ring_mesh = meshes.add(Annulus::new(0.55, 0.7));
    let ring_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.6, 0.6, 0.7, 0.45),
        emissive: LinearRgba::new(0.5, 0.5, 0.6, 0.0) * 1.5,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let ground_circle = commands.spawn((
        Mesh3d(ring_mesh),
        MeshMaterial3d(ring_material.clone()),
        Transform::from_xyz(0.0, -0.89, 0.0)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        GroundCircle {
            inner_radius: 0.55,
            outer_radius: 0.7,
            base_alpha: 0.45,
            pulse_speed: 5.0,
            material_handle: ring_material,
            last_hp_fraction: -1.0,
            last_facing: 0.0,
            last_alpha: 0.0,
        },
    )).id();

    commands.entity(enemy_entity).add_child(model_child);
    commands.entity(enemy_entity).add_child(ground_circle);
}

/// Волновая система спавна врагов
pub fn wave_spawner_system(
    time: Res<Time>,
    mut wave: ResMut<WaveState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    alive_enemies: Query<Entity, (With<Enemy>, Without<EnemyDying>, Without<EnemyCorpse>)>,
) {
    match wave.phase {
        WavePhase::Cooldown => {
            wave.wave_cooldown.tick(time.delta());
            if wave.wave_cooldown.is_finished() {
                // Начинаем новую волну
                wave.current_wave += 1;
                wave.enemies_to_spawn = 2 + wave.current_wave;
                wave.spawn_timer.reset();
                wave.portal_0_count = 0;
                wave.portal_1_count = 0;
                wave.phase = WavePhase::Spawning;
                debug!("🌊 Wave {} started! Spawning {} enemies", wave.current_wave, wave.enemies_to_spawn);
            }
        }
        WavePhase::Spawning => {
            wave.spawn_timer.tick(time.delta());
            if wave.spawn_timer.just_finished() && wave.enemies_to_spawn > 0 {
                let pos = portal_spawn_position(&mut wave);

                // Выбор типа врага: один бросок, ranges не перекрываются
                let roll = rand_01();
                if wave.current_wave >= 3 && roll < 0.3 {
                    // Леший: 30% с волны 3+
                    debug!("🌿 Wave {} — spawning Leshiy at {:?}", wave.current_wave, pos);
                    spawn_leshiy_at(
                        &mut commands, &asset_server, &mut graphs,
                        &mut meshes, &mut materials, pos,
                    );
                } else if wave.current_wave >= 2 && roll < 0.5 {
                    // Волколак: 20% с волны 2+ (roll 0.3–0.5, или 0.0–0.5 на волне 2)
                    debug!("🐺 Wave {} — spawning Volkolak at {:?}", wave.current_wave, pos);
                    spawn_volkolak_at(
                        &mut commands, &asset_server, &mut graphs,
                        &mut meshes, &mut materials, pos,
                    );
                } else {
                    spawn_upyr_at(
                        &mut commands, &asset_server, &mut graphs,
                        &mut meshes, &mut materials, pos,
                    );
                }
                wave.enemies_to_spawn -= 1;

                if wave.enemies_to_spawn == 0 {
                    wave.phase = WavePhase::Fighting;
                    debug!("⚔️ Wave {} — all enemies spawned, fight!", wave.current_wave);
                }
            }
        }
        WavePhase::Fighting => {
            // Проверяем сколько живых врагов осталось
            let alive_count = alive_enemies.iter().count();
            if alive_count == 0 {
                // Все мертвы — начинаем cooldown
                wave.wave_cooldown.reset();
                wave.phase = WavePhase::Cooldown;
                debug!("✅ Wave {} cleared! Next wave in 3s...", wave.current_wave);
            }
        }
    }
}

/// Выбирает позицию спавна из одного из двух порталов (~50/50 ±10%)
fn portal_spawn_position(wave: &mut WaveState) -> Vec3 {
    let total = wave.portal_0_count + wave.portal_1_count;
    let use_portal_0 = if total == 0 {
        rand_01() < 0.5
    } else {
        let ratio = wave.portal_0_count as f32 / total as f32;
        if ratio > 0.6 {
            false // Портал 0 перегружен — в портал 1
        } else if ratio < 0.4 {
            true  // Портал 1 перегружен — в портал 0
        } else {
            rand_01() < 0.5
        }
    };

    let (base_pos, count) = if use_portal_0 {
        wave.portal_0_count += 1;
        (PORTAL_1_SPAWN, wave.portal_0_count)
    } else {
        wave.portal_1_count += 1;
        (PORTAL_2_SPAWN, wave.portal_1_count)
    };

    // Случайный оффсет: ±1.5м по X, 0-3м по Z (вглубь арены)
    let offset = Vec3::new(
        (rand_01() - 0.5) * 3.0,
        0.0,
        rand_01() * 3.0,
    );

    let _ = count; // suppress unused warning
    base_pos + offset
}


