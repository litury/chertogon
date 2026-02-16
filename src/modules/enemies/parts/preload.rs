use bevy::prelude::*;
use crate::toolkit::asset_paths;
use super::spawner::EnemyAnimationIndices;

/// Кэшированные Handle ассетов врагов — загружаются при старте раунда.
/// AnimationGraph создаётся 1 раз на тип врага и шарится между всеми entity.
#[derive(Resource)]
pub struct EnemyAssets {
    // Модели
    pub upyr_model: Handle<Scene>,
    pub leshiy_model: Handle<Scene>,
    pub volkolak_model: Handle<Scene>,

    // Shared ring meshes (Annulus, 1 на тип — все entity шарят handle)
    pub upyr_ring_mesh: Handle<Mesh>,
    pub leshiy_ring_mesh: Handle<Mesh>,
    pub volkolak_ring_mesh: Handle<Mesh>,

    // Shared AnimationGraph + индексы (1 граф на тип, handle клонируется между entity)
    pub upyr_graph: Handle<AnimationGraph>,
    pub upyr_indices: EnemyAnimationIndices,
    pub leshiy_graph: Handle<AnimationGraph>,
    pub leshiy_indices: EnemyAnimationIndices,
    pub volkolak_graph: Handle<AnimationGraph>,
    pub volkolak_indices: EnemyAnimationIndices,
}

/// Загрузка всех ассетов врагов при старте раунда
pub fn preload_enemy_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // --- Упырь ---
    let mut upyr_graph = AnimationGraph::new();
    let upyr_idle = upyr_graph.add_clip(asset_server.load(asset_paths::UPYR_ANIM_IDLE), 1.0, upyr_graph.root);
    let upyr_walk = upyr_graph.add_clip(asset_server.load(asset_paths::UPYR_ANIM_WALK), 1.0, upyr_graph.root);
    let upyr_run = upyr_graph.add_clip(asset_server.load(asset_paths::UPYR_ANIM_RUN), 1.0, upyr_graph.root);
    let upyr_attack = upyr_graph.add_clip(asset_server.load(asset_paths::UPYR_ANIM_ATTACK), 1.0, upyr_graph.root);
    let upyr_death = upyr_graph.add_clip(asset_server.load(asset_paths::UPYR_ANIM_DEATH), 1.0, upyr_graph.root);
    let upyr_hit = upyr_graph.add_clip(asset_server.load(asset_paths::UPYR_ANIM_HIT), 1.0, upyr_graph.root);
    let upyr_scream = upyr_graph.add_clip(asset_server.load(asset_paths::UPYR_ANIM_SCREAM), 1.0, upyr_graph.root);
    let upyr_graph_handle = graphs.add(upyr_graph);

    // --- Леший ---
    let mut leshiy_graph = AnimationGraph::new();
    let leshiy_idle = leshiy_graph.add_clip(asset_server.load(asset_paths::LESHIY_ANIM_IDLE), 1.0, leshiy_graph.root);
    let leshiy_walk = leshiy_graph.add_clip(asset_server.load(asset_paths::LESHIY_ANIM_WALK), 1.0, leshiy_graph.root);
    let leshiy_run = leshiy_graph.add_clip(asset_server.load(asset_paths::LESHIY_ANIM_RUN), 1.0, leshiy_graph.root);
    let leshiy_attack = leshiy_graph.add_clip(asset_server.load(asset_paths::LESHIY_ANIM_ATTACK), 1.0, leshiy_graph.root);
    let leshiy_death = leshiy_graph.add_clip(asset_server.load(asset_paths::LESHIY_ANIM_DEATH), 1.0, leshiy_graph.root);
    let leshiy_hit = leshiy_graph.add_clip(asset_server.load(asset_paths::LESHIY_ANIM_HIT), 1.0, leshiy_graph.root);
    let leshiy_graph_handle = graphs.add(leshiy_graph);

    // --- Волколак ---
    let mut volkolak_graph = AnimationGraph::new();
    let volkolak_idle_handle = asset_server.load(asset_paths::VOLKOLAK_ANIM_IDLE);
    let volkolak_idle = volkolak_graph.add_clip(volkolak_idle_handle.clone(), 1.0, volkolak_graph.root);
    let volkolak_walk = volkolak_graph.add_clip(asset_server.load(asset_paths::VOLKOLAK_ANIM_WALK), 1.0, volkolak_graph.root);
    let volkolak_run = volkolak_graph.add_clip(asset_server.load(asset_paths::VOLKOLAK_ANIM_RUN), 1.0, volkolak_graph.root);
    let volkolak_attack = volkolak_graph.add_clip(asset_server.load(asset_paths::VOLKOLAK_ANIM_ATTACK), 1.0, volkolak_graph.root);
    let volkolak_death = volkolak_graph.add_clip(asset_server.load(asset_paths::VOLKOLAK_ANIM_DEATH), 1.0, volkolak_graph.root);
    let volkolak_hit = volkolak_graph.add_clip(asset_server.load(asset_paths::VOLKOLAK_ANIM_HIT), 1.0, volkolak_graph.root);
    let volkolak_scream = volkolak_graph.add_clip(volkolak_idle_handle, 0.5, volkolak_graph.root);
    let volkolak_graph_handle = graphs.add(volkolak_graph);

    // Shared ring meshes (1 на тип, все entity клонируют handle)
    let upyr_ring_mesh = meshes.add(Annulus::new(0.45, 0.6));
    let leshiy_ring_mesh = meshes.add(Annulus::new(0.5, 0.65));
    let volkolak_ring_mesh = meshes.add(Annulus::new(0.55, 0.7));

    commands.insert_resource(EnemyAssets {
        upyr_model: asset_server.load(asset_paths::UPYR_MODEL),
        leshiy_model: asset_server.load(asset_paths::LESHIY_MODEL),
        volkolak_model: asset_server.load(asset_paths::VOLKOLAK_MODEL),

        upyr_ring_mesh,
        leshiy_ring_mesh,
        volkolak_ring_mesh,

        upyr_graph: upyr_graph_handle,
        upyr_indices: EnemyAnimationIndices {
            idle: upyr_idle,
            walk: upyr_walk,
            run: upyr_run,
            attack: upyr_attack,
            death: upyr_death,
            hit: upyr_hit,
            scream: upyr_scream,
        },
        leshiy_graph: leshiy_graph_handle,
        leshiy_indices: EnemyAnimationIndices {
            idle: leshiy_idle,
            walk: leshiy_walk,
            run: leshiy_run,
            attack: leshiy_attack,
            death: leshiy_death,
            hit: leshiy_hit,
            scream: leshiy_idle, // Леший не кричит при спавне
        },
        volkolak_graph: volkolak_graph_handle,
        volkolak_indices: EnemyAnimationIndices {
            idle: volkolak_idle,
            walk: volkolak_walk,
            run: volkolak_run,
            attack: volkolak_attack,
            death: volkolak_death,
            hit: volkolak_hit,
            scream: volkolak_scream,
        },
    });

    debug!("Enemy assets preloading started (shared AnimationGraphs)");
}
