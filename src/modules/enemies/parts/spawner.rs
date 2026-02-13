use bevy::prelude::*;
use avian3d::prelude::*;
use crate::modules::enemies::components::*;
use crate::modules::combat::components::EnemyAttackCooldown;
use crate::modules::player::components::Player;
use crate::modules::world::GroundCircle;
use crate::toolkit::asset_paths;

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
        EnemyAnimState { current: EnemyAnim::Screaming },
        SpawnScream { timer: Timer::from_seconds(1.5, TimerMode::Once) },
        Transform::from_translation(spawn_pos),
        RigidBody::Dynamic,
        Collider::cylinder(0.5, 1.8),
        LinearVelocity::default(),
        LinearDamping(12.0),
        AngularDamping(8.0),
        crate::shared::GameLayer::enemy_layers(),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z(),
        EnemyAttackCooldown::new(5.0, 1.0),
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
    let enemy_entity = commands.spawn((
        Enemy,
        EnemyType::Leshiy,
        Health::new(15.0),
        ChasePlayer {
            speed: 6.0,
            aggro_range: 15.0,
            attack_range: 2.5,
        },
        EnemyAnimState { current: EnemyAnim::Idle },
        Transform::from_translation(spawn_pos),
        RigidBody::Dynamic,
        Collider::cylinder(0.4, 1.6),
        LinearVelocity::default(),
        LinearDamping(12.0),
        AngularDamping(8.0),
        crate::shared::GameLayer::enemy_layers(),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z(),
        EnemyAttackCooldown::new(8.0, 0.8),
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
    let enemy_entity = commands.spawn((
        Enemy,
        EnemyType::Volkolak,
        Health::new(12.0),
        ChasePlayer {
            speed: 7.0,
            aggro_range: 20.0,
            attack_range: 1.8,
        },
        EnemyAnimState { current: EnemyAnim::Idle },
        Transform::from_translation(spawn_pos),
        RigidBody::Dynamic,
        Collider::cylinder(0.8, 1.0),
        LinearVelocity::default(),
        LinearDamping(12.0),
        AngularDamping(8.0),
        crate::shared::GameLayer::enemy_layers(),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z(),
        EnemyAttackCooldown::new(6.0, 0.8),  // 7.5 DPS (между Упырём 5 и Лешим 10)
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
    player: Query<&Transform, With<Player>>,
) {
    match wave.phase {
        WavePhase::Cooldown => {
            wave.wave_cooldown.tick(time.delta());
            if wave.wave_cooldown.is_finished() {
                // Начинаем новую волну
                wave.current_wave += 1;
                wave.enemies_to_spawn = 2 + wave.current_wave;
                wave.spawn_timer.reset();
                wave.phase = WavePhase::Spawning;
                info!("🌊 Wave {} started! Spawning {} enemies", wave.current_wave, wave.enemies_to_spawn);
            }
        }
        WavePhase::Spawning => {
            wave.spawn_timer.tick(time.delta());
            if wave.spawn_timer.just_finished() && wave.enemies_to_spawn > 0 {
                let pos = random_spawn_position(player.single().ok());

                // Выбор типа врага: один бросок, ranges не перекрываются
                let roll = rand_01();
                if wave.current_wave >= 3 && roll < 0.3 {
                    // Леший: 30% с волны 3+
                    info!("🌿 Wave {} — spawning Leshiy at {:?}", wave.current_wave, pos);
                    spawn_leshiy_at(
                        &mut commands, &asset_server, &mut graphs,
                        &mut meshes, &mut materials, pos,
                    );
                } else if wave.current_wave >= 2 && roll < 0.5 {
                    // Волколак: 20% с волны 2+ (roll 0.3–0.5, или 0.0–0.5 на волне 2)
                    info!("🐺 Wave {} — spawning Volkolak at {:?}", wave.current_wave, pos);
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
                    info!("⚔️ Wave {} — all enemies spawned, fight!", wave.current_wave);
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
                info!("✅ Wave {} cleared! Next wave in 3s...", wave.current_wave);
            }
        }
    }
}

/// Случайная позиция для спавна (radius 15-20м от центра, min 8м от игрока)
fn random_spawn_position(player_transform: Option<&Transform>) -> Vec3 {
    let player_pos = player_transform
        .map(|t| t.translation)
        .unwrap_or(Vec3::ZERO);

    // Пробуем до 10 раз найти позицию далеко от игрока
    for _ in 0..10 {
        let angle = rand_angle();
        let radius = 10.0 + rand_01() * 5.0; // 10–15м от центра
        let pos = Vec3::new(angle.cos() * radius, 0.9, angle.sin() * radius);

        if (pos - player_pos).length() >= 8.0 {
            return pos;
        }
    }

    // Fallback — противоположная сторона от игрока
    let away = -player_pos.normalize_or_zero();
    Vec3::new(away.x * 12.0, 0.9, away.z * 12.0)
}

/// Pseudo-random [0.0, 1.0) — xorshift64, seed из адреса стека (WASM-safe)
fn rand_01() -> f32 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static RNG_STATE: AtomicU64 = AtomicU64::new(0);

    let mut state = RNG_STATE.load(Ordering::Relaxed);
    if state == 0 {
        // Seed из адреса локальной переменной — уникален при каждом запуске
        let stack_var: u64 = 0;
        let addr = &stack_var as *const u64 as u64;
        state = addr.wrapping_mul(0x517cc1b727220a95).wrapping_add(0xDEAD_BEEF_CAFE_BABE);
        if state == 0 { state = 1; }
    }
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    RNG_STATE.store(state, Ordering::Relaxed);
    ((state % 10000) as f32) / 10000.0
}

fn rand_angle() -> f32 {
    rand_01() * std::f32::consts::TAU
}
