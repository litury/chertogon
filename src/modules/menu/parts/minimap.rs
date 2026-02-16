use bevy::prelude::*;
use crate::modules::menu::components::HudUI;
use crate::modules::player::Player;
use crate::modules::enemies::components::{Enemy, EnemyDying, SpawnPortal};
use crate::toolkit::asset_paths;

const MAP_SIZE: f32 = 100.0;
const ARENA_EXTENT: f32 = 25.0; // Арена 50×50м, от -25 до +25
const PLAYER_DOT_SIZE: f32 = 6.0;
const ENEMY_DOT_SIZE: f32 = 4.0;
const PORTAL_DOT_SIZE: f32 = 4.0;
const DOT_POOL_SIZE: usize = 30;

/// Маркер корневого контейнера миникарты
#[derive(Component)]
pub struct MinimapUI;

/// Маркер внутреннего контейнера (для позиционирования точек)
#[derive(Component)]
pub struct MinimapField;

/// Точка на миникарте
#[derive(Component)]
pub struct MinimapDot {
    pub dot_type: MinimapDotType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MinimapDotType {
    Player,
    Enemy,
    Portal,
}

/// Спавн миникарты: круглый контейнер + пул точек (все точки — children MinimapField)
pub fn setup_minimap(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Внутреннее поле для абсолютного позиционирования точек
    let field = commands.spawn((
        HudUI,
        MinimapField,
        Node {
            width: Val::Px(MAP_SIZE),
            height: Val::Px(MAP_SIZE),
            position_type: PositionType::Relative,
            ..default()
        },
    )).id();

    // Точка игрока (child of field)
    let player_dot = commands.spawn((
        HudUI,
        MinimapDot { dot_type: MinimapDotType::Player },
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(PLAYER_DOT_SIZE),
            height: Val::Px(PLAYER_DOT_SIZE),
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            border_radius: BorderRadius::all(Val::Percent(50.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.95, 0.75, 0.2, 1.0)),
        Visibility::Hidden,
        GlobalZIndex(53),
    )).id();
    commands.entity(field).add_child(player_dot);

    // Пул точек врагов (children of field)
    for _ in 0..DOT_POOL_SIZE {
        let dot = commands.spawn((
            HudUI,
            MinimapDot { dot_type: MinimapDotType::Enemy },
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(ENEMY_DOT_SIZE),
                height: Val::Px(ENEMY_DOT_SIZE),
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                border_radius: BorderRadius::all(Val::Percent(50.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.9, 0.15, 0.1, 0.9)),
            Visibility::Hidden,
            GlobalZIndex(52),
        )).id();
        commands.entity(field).add_child(dot);
    }

    // Точки порталов (children of field)
    for _ in 0..2 {
        let dot = commands.spawn((
            HudUI,
            MinimapDot { dot_type: MinimapDotType::Portal },
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(PORTAL_DOT_SIZE),
                height: Val::Px(PORTAL_DOT_SIZE),
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                border_radius: BorderRadius::all(Val::Percent(50.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.6, 0.2, 0.9, 0.9)),
            Visibility::Hidden,
            GlobalZIndex(52),
        )).id();
        commands.entity(field).add_child(dot);
    }

    // Frame overlay — рамка поверх содержимого миникарты
    let frame = commands.spawn((
        HudUI,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            border_radius: BorderRadius::all(Val::Percent(50.0)),
            ..default()
        },
        ImageNode::new(asset_server.load(asset_paths::UI_MINIMAP_FRAME)),
        GlobalZIndex(54),
    )).id();

    // Корневой контейнер — круглый, top-right
    commands.spawn((
        HudUI,
        MinimapUI,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(2.5),
            right: Val::Percent(2.5),
            width: Val::Px(MAP_SIZE),
            height: Val::Px(MAP_SIZE),
            border_radius: BorderRadius::all(Val::Percent(50.0)),
            overflow: Overflow::clip(),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.04, 0.1, 0.85)),
        GlobalZIndex(50),
    )).add_children(&[field, frame]);
}

/// Проецирует world positions на миникарту. Игрок в центре, мир двигается.
pub fn update_minimap(
    player_query: Query<&Transform, With<Player>>,
    enemies: Query<&Transform, (With<Enemy>, Without<EnemyDying>, Without<Player>)>,
    portals: Query<&Transform, (With<SpawnPortal>, Without<Player>, Without<Enemy>)>,
    minimap_field: Query<&ComputedNode, With<MinimapField>>,
    mut dots: Query<(&MinimapDot, &mut Node, &mut Visibility)>,
) {
    let Ok(player_tf) = player_query.single() else { return };
    let Ok(_field_node) = minimap_field.single() else { return };

    let player_pos = player_tf.translation;
    let half = MAP_SIZE / 2.0;

    // Собираем позиции (для пула)
    let enemy_positions: Vec<Vec3> = enemies.iter().map(|t| t.translation).collect();
    let portal_positions: Vec<Vec3> = portals.iter().map(|t| t.translation).collect();

    let mut enemy_idx = 0usize;
    let mut portal_idx = 0usize;

    for (dot, mut node, mut visibility) in &mut dots {
        match dot.dot_type {
            MinimapDotType::Player => {
                // Игрок всегда в центре
                node.left = Val::Px(half - PLAYER_DOT_SIZE / 2.0);
                node.top = Val::Px(half - PLAYER_DOT_SIZE / 2.0);
                *visibility = Visibility::Inherited;
            }
            MinimapDotType::Enemy => {
                if enemy_idx < enemy_positions.len() {
                    let pos = enemy_positions[enemy_idx];
                    let (mx, my) = world_to_minimap(pos, player_pos);
                    let dx = mx - half;
                    let dy = my - half;
                    if dx * dx + dy * dy <= half * half {
                        node.left = Val::Px(mx - ENEMY_DOT_SIZE / 2.0);
                        node.top = Val::Px(my - ENEMY_DOT_SIZE / 2.0);
                        *visibility = Visibility::Inherited;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                    enemy_idx += 1;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
            MinimapDotType::Portal => {
                if portal_idx < portal_positions.len() {
                    let pos = portal_positions[portal_idx];
                    let (mx, my) = world_to_minimap(pos, player_pos);
                    let dx = mx - half;
                    let dy = my - half;
                    if dx * dx + dy * dy <= half * half {
                        node.left = Val::Px(mx - PORTAL_DOT_SIZE / 2.0);
                        node.top = Val::Px(my - PORTAL_DOT_SIZE / 2.0);
                        *visibility = Visibility::Inherited;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                    portal_idx += 1;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

/// Мировые координаты → координаты на миникарте (player-centered)
fn world_to_minimap(world_pos: Vec3, player_pos: Vec3) -> (f32, f32) {
    let half = MAP_SIZE / 2.0;
    let scale = MAP_SIZE / (ARENA_EXTENT * 2.0); // 100 / 50 = 2.0 px/m

    let dx = world_pos.x - player_pos.x;
    let dz = world_pos.z - player_pos.z;

    let mx = half + dx * scale;
    let my = half + dz * scale;

    (mx, my)
}

/// Despawn миникарты при выходе из Playing
pub fn cleanup_minimap(
    mut commands: Commands,
    query: Query<Entity, Or<(With<MinimapUI>, With<MinimapDot>, With<MinimapField>)>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
