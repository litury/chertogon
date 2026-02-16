use bevy::prelude::*;
use avian3d::prelude::*;
use crate::modules::enemies::components::*;
use crate::modules::combat::components::EnemyAttackCooldown;
use crate::modules::world::GroundCircle;
use crate::modules::enemies::components::PortalSpawnAnim;
use crate::shared::constants::{PORTAL_1_SPAWN, PORTAL_2_SPAWN};
use crate::shared::rand_01;
use crate::modules::menu::KillFeedMessage;
use super::preload::EnemyAssets;

/// Индексы анимаций врага в AnimationGraph — хранится на EnemyModel перманентно.
/// setup_enemy_animation перезапускается если Bevy пересоздаст сцену из SceneRoot.
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

/// Спавнит одного Упыря в указанной позиции (shared AnimationGraph из EnemyAssets)
pub fn spawn_upyr_at(
    commands: &mut Commands,
    assets: &EnemyAssets,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_pos: Vec3,
) {
    let enemy_entity = commands.spawn((
        Enemy,
        EnemyType::Upyr,
        EnemyLod::default(),
        CachedAnimSpeed::default(),
        Health::new(20.0),
        ChasePlayer {
            speed: 3.0,
            aggro_range: 12.0,
            attack_range: 1.5,
            anim_base_speed: 3.0,
        },
        OrbitDirection {
            clockwise: rand_01() > 0.5,
            change_timer: Timer::from_seconds(3.0 + rand_01() * 3.0, TimerMode::Repeating),
        },
        EnemyAnimState::new(EnemyAnim::Screaming),
        SpawnScream { timer: Timer::from_seconds(1.5, TimerMode::Once) },
        PortalSpawnAnim::new(),
        Transform::from_translation(spawn_pos).with_scale(Vec3::splat(0.01)),
        RigidBody::Dynamic,
        Collider::cylinder(0.5, 1.8),
    )).insert((
        LinearVelocity::default(),
        LinearDamping(12.0),
        AngularDamping(8.0),
        crate::shared::GameLayer::enemy_layers(),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z(),
        EnemyAttackCooldown::new(5.0, 1.0, 1.5),
    )).id();

    let model_child = commands.spawn((
        SceneRoot(assets.upyr_model.clone()),
        Transform::from_xyz(0.0, -0.9, 0.0),
        EnemyModel,
        assets.upyr_indices,
        AnimationGraphHandle(assets.upyr_graph.clone()),
    )).id();

    let ring_mesh = assets.upyr_ring_mesh.clone();
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
    assets: &EnemyAssets,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_pos: Vec3,
) {
    let enemy_entity = commands.spawn((
        Enemy,
        EnemyType::Leshiy,
        EnemyLod::default(),
        CachedAnimSpeed::default(),
        Health::new(15.0),
        ChasePlayer {
            speed: 6.0,
            aggro_range: 15.0,
            attack_range: 1.8,
            anim_base_speed: 3.5,
        },
        OrbitDirection {
            clockwise: rand_01() > 0.5,
            change_timer: Timer::from_seconds(3.0 + rand_01() * 3.0, TimerMode::Repeating),
        },
        EnemyAnimState::new(EnemyAnim::Idle),
        PortalSpawnAnim::new(),
        Transform::from_translation(spawn_pos).with_scale(Vec3::splat(0.01)),
        RigidBody::Dynamic,
        Collider::cylinder(0.5, 2.2),
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

    let model_child = commands.spawn((
        SceneRoot(assets.leshiy_model.clone()),
        Transform::from_xyz(0.0, -1.1, 0.0)
            .with_scale(Vec3::splat(1.3)),
        EnemyModel,
        assets.leshiy_indices,
        AnimationGraphHandle(assets.leshiy_graph.clone()),
    )).id();

    let ring_mesh = assets.leshiy_ring_mesh.clone();
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
        Transform::from_xyz(0.0, -1.09, 0.0)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        GroundCircle {
            inner_radius: 0.5,
            outer_radius: 0.65,
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
    assets: &EnemyAssets,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_pos: Vec3,
) {
    let enemy_entity = commands.spawn((
        Enemy,
        EnemyType::Volkolak,
        EnemyLod::default(),
        CachedAnimSpeed::default(),
        Health::new(12.0),
        ChasePlayer {
            speed: 7.0,
            aggro_range: 20.0,
            attack_range: 1.3,
            anim_base_speed: 3.5,
        },
        OrbitDirection {
            clockwise: rand_01() > 0.5,
            change_timer: Timer::from_seconds(3.0 + rand_01() * 3.0, TimerMode::Repeating),
        },
        EnemyAnimState::new(EnemyAnim::Idle),
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
        EnemyAttackCooldown::new(6.0, 0.8, 1.3),
    )).id();

    let model_child = commands.spawn((
        SceneRoot(assets.volkolak_model.clone()),
        Transform::from_xyz(0.0, -0.9, 0.0)
            .with_scale(Vec3::splat(1.0)),
        EnemyModel,
        assets.volkolak_indices,
        AnimationGraphHandle(assets.volkolak_graph.clone()),
    )).id();

    let ring_mesh = assets.volkolak_ring_mesh.clone();
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
    enemy_assets: Option<Res<EnemyAssets>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    alive_enemies: Query<Entity, (With<Enemy>, Without<EnemyDying>, Without<EnemyCorpse>)>,
    mut feed: MessageWriter<KillFeedMessage>,
) {
    match wave.phase {
        WavePhase::Cooldown => {
            wave.wave_cooldown.tick(time.delta());
            if wave.wave_cooldown.is_finished() {
                wave.current_wave += 1;
                wave.enemies_to_spawn = 2 + wave.current_wave;
                wave.spawn_timer.reset();
                wave.portal_0_count = 0;
                wave.portal_1_count = 0;
                wave.phase = WavePhase::Spawning;
                // Баннер "ВОЛНА N" спавнится через wave_banner система (по wave.is_changed)
                debug!("Wave {} started! Spawning {} enemies", wave.current_wave, wave.enemies_to_spawn);
            }
        }
        WavePhase::Spawning => {
            let Some(assets) = &enemy_assets else { return };

            wave.spawn_timer.tick(time.delta());
            if wave.spawn_timer.just_finished() && wave.enemies_to_spawn > 0 {
                let pos = portal_spawn_position(&mut wave);

                let roll = rand_01();
                if wave.current_wave >= 3 && roll < 0.3 {
                    spawn_leshiy_at(
                        &mut commands, assets,
                        &mut materials, pos,
                    );
                } else if wave.current_wave >= 2 && roll < 0.5 {
                    spawn_volkolak_at(
                        &mut commands, assets,
                        &mut materials, pos,
                    );
                } else {
                    spawn_upyr_at(
                        &mut commands, assets,
                        &mut materials, pos,
                    );
                }
                wave.enemies_to_spawn -= 1;

                if wave.enemies_to_spawn == 0 {
                    wave.phase = WavePhase::Fighting;
                    debug!("Wave {} — all enemies spawned, fight!", wave.current_wave);
                }
            }
        }
        WavePhase::Fighting => {
            let alive_count = alive_enemies.iter().count();
            if alive_count == 0 {
                wave.wave_cooldown.reset();
                wave.phase = WavePhase::Cooldown;
                feed.write(KillFeedMessage {
                    text: format!("Волна {} пройдена!", wave.current_wave),
                    color: Color::srgb(0.4, 0.9, 0.5),
                    group_key: None,
                });
                debug!("Wave {} cleared! Next wave in 3s...", wave.current_wave);
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
            false
        } else if ratio < 0.4 {
            true
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

    let offset = Vec3::new(
        (rand_01() - 0.5) * 3.0,
        0.0,
        rand_01() * 3.0,
    );

    let _ = count;
    base_pos + offset
}
