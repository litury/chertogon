use bevy::prelude::*;
use crate::toolkit::asset_paths;

/// Кэшированные Handle ассетов врагов — загружаются при старте раунда.
/// Bevy asset server кэширует по пути, но preload гарантирует что HTTP-запросы
/// (в WASM) завершатся ДО первого спавна врагов, убирая хитчи.
#[derive(Resource)]
pub struct EnemyAssets {
    // Упырь
    pub upyr_model: Handle<Scene>,
    pub upyr_idle: Handle<AnimationClip>,
    pub upyr_walk: Handle<AnimationClip>,
    pub upyr_run: Handle<AnimationClip>,
    pub upyr_attack: Handle<AnimationClip>,
    pub upyr_death: Handle<AnimationClip>,
    pub upyr_hit: Handle<AnimationClip>,
    pub upyr_scream: Handle<AnimationClip>,

    // Леший
    pub leshiy_model: Handle<Scene>,
    pub leshiy_idle: Handle<AnimationClip>,
    pub leshiy_walk: Handle<AnimationClip>,
    pub leshiy_run: Handle<AnimationClip>,
    pub leshiy_attack: Handle<AnimationClip>,
    pub leshiy_death: Handle<AnimationClip>,
    pub leshiy_hit: Handle<AnimationClip>,

    // Волколак
    pub volkolak_model: Handle<Scene>,
    pub volkolak_idle: Handle<AnimationClip>,
    pub volkolak_walk: Handle<AnimationClip>,
    pub volkolak_run: Handle<AnimationClip>,
    pub volkolak_attack: Handle<AnimationClip>,
    pub volkolak_death: Handle<AnimationClip>,
    pub volkolak_hit: Handle<AnimationClip>,
}

/// Загрузка всех ассетов врагов при старте раунда
pub fn preload_enemy_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(EnemyAssets {
        // Упырь
        upyr_model: asset_server.load(asset_paths::UPYR_MODEL),
        upyr_idle: asset_server.load(asset_paths::UPYR_ANIM_IDLE),
        upyr_walk: asset_server.load(asset_paths::UPYR_ANIM_WALK),
        upyr_run: asset_server.load(asset_paths::UPYR_ANIM_RUN),
        upyr_attack: asset_server.load(asset_paths::UPYR_ANIM_ATTACK),
        upyr_death: asset_server.load(asset_paths::UPYR_ANIM_DEATH),
        upyr_hit: asset_server.load(asset_paths::UPYR_ANIM_HIT),
        upyr_scream: asset_server.load(asset_paths::UPYR_ANIM_SCREAM),

        // Леший
        leshiy_model: asset_server.load(asset_paths::LESHIY_MODEL),
        leshiy_idle: asset_server.load(asset_paths::LESHIY_ANIM_IDLE),
        leshiy_walk: asset_server.load(asset_paths::LESHIY_ANIM_WALK),
        leshiy_run: asset_server.load(asset_paths::LESHIY_ANIM_RUN),
        leshiy_attack: asset_server.load(asset_paths::LESHIY_ANIM_ATTACK),
        leshiy_death: asset_server.load(asset_paths::LESHIY_ANIM_DEATH),
        leshiy_hit: asset_server.load(asset_paths::LESHIY_ANIM_HIT),

        // Волколак
        volkolak_model: asset_server.load(asset_paths::VOLKOLAK_MODEL),
        volkolak_idle: asset_server.load(asset_paths::VOLKOLAK_ANIM_IDLE),
        volkolak_walk: asset_server.load(asset_paths::VOLKOLAK_ANIM_WALK),
        volkolak_run: asset_server.load(asset_paths::VOLKOLAK_ANIM_RUN),
        volkolak_attack: asset_server.load(asset_paths::VOLKOLAK_ANIM_ATTACK),
        volkolak_death: asset_server.load(asset_paths::VOLKOLAK_ANIM_DEATH),
        volkolak_hit: asset_server.load(asset_paths::VOLKOLAK_ANIM_HIT),
    });

    debug!("Enemy assets preloading started");
}
