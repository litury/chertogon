use bevy::prelude::*;
use crate::modules::player::components::Player;
use crate::modules::enemies::components::*;
use crate::shared::rand_01;
use super::preload::EnemyAssets;

/// Маркер для дебаг-счётчика врагов (абсолютный overlay)
#[derive(Component)]
pub struct DebugEnemyCounter;

/// Создаёт дебаг-счётчик (вызывается при OnEnter(Playing))
pub fn setup_debug_counter(mut commands: Commands) {
    commands.spawn((
        DebugEnemyCounter,
        Text::new(""),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.9, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            bottom: Val::Px(12.0),
            ..default()
        },
    ));
}

/// Обновляет дебаг-счётчик (Alive / Dying / Corpses)
pub fn update_debug_counter(
    alive: Query<Entity, (With<Enemy>, Without<EnemyDying>, Without<EnemyCorpse>)>,
    dying: Query<Entity, With<EnemyDying>>,
    corpses: Query<Entity, With<EnemyCorpse>>,
    mut text_query: Query<&mut Text, With<DebugEnemyCounter>>,
) {
    let alive_count = alive.iter().count();
    let dying_count = dying.iter().count();
    let corpse_count = corpses.iter().count();

    for mut text in &mut text_query {
        **text = format!(
            "Alive: {} | Dying: {} | Corpses: {}",
            alive_count, dying_count, corpse_count,
        );
    }
}

/// Удаляет дебаг-счётчик
pub fn cleanup_debug_counter(
    mut commands: Commands,
    query: Query<Entity, With<DebugEnemyCounter>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// F1: спавн 1 Упыря рядом с игроком
/// F2: спавн 10 Упырей за раз
/// F3: убить всех живых врагов (HP = 0)
/// F4: деспавн всех трупов
pub fn debug_spawn_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    enemy_assets: Option<Res<EnemyAssets>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    mut enemies: Query<&mut Health, (With<Enemy>, Without<EnemyDying>)>,
    corpses: Query<Entity, With<EnemyCorpse>>,
) {
    let Ok(player_tf) = player_query.single() else { return };
    let player_pos = player_tf.translation;

    // F1: 1 Упырь
    if keys.just_pressed(KeyCode::F1) {
        if let Some(assets) = &enemy_assets {
            let pos = random_pos_near(player_pos, 5.0);
            super::spawner::spawn_upyr_at(
                &mut commands, assets,
                &mut materials, pos,
            );
            info!("[DEBUG] Spawned 1 Upyr at {:?}", pos);
        }
    }

    // F2: 10 Упырей
    if keys.just_pressed(KeyCode::F2) {
        if let Some(assets) = &enemy_assets {
            for _ in 0..10 {
                let pos = random_pos_near(player_pos, 8.0);
                super::spawner::spawn_upyr_at(
                    &mut commands, assets,
                    &mut materials, pos,
                );
            }
            info!("[DEBUG] Spawned 10 Upyr");
        }
    }

    // F3: убить всех живых
    if keys.just_pressed(KeyCode::F3) {
        let mut count = 0;
        for mut health in &mut enemies {
            health.current = 0.0;
            count += 1;
        }
        info!("[DEBUG] Killed {} enemies", count);
    }

    // F4: деспавн всех трупов
    if keys.just_pressed(KeyCode::F4) {
        let mut count = 0;
        for entity in &corpses {
            commands.entity(entity).despawn();
            count += 1;
        }
        info!("[DEBUG] Despawned {} corpses", count);
    }
}

/// Случайная позиция в radius метрах от center
fn random_pos_near(center: Vec3, radius: f32) -> Vec3 {
    let angle = rand_01() * std::f32::consts::TAU;
    let dist = 3.0 + rand_01() * (radius - 3.0);
    Vec3::new(
        center.x + angle.cos() * dist,
        0.0,
        center.z + angle.sin() * dist,
    )
}
