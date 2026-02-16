use bevy::prelude::*;
use crate::modules::enemies::components::*;
use crate::modules::player::components::Player;
use crate::modules::world::GroundCircle;

/// Обновляет LOD уровень по дистанции до игрока (O(n), дешёвый)
pub fn update_enemy_lod_system(
    player: Query<&Transform, With<Player>>,
    mut enemies: Query<(&Transform, &mut EnemyLod), (With<Enemy>, Without<EnemyDying>, Without<Player>)>,
) {
    let Ok(player_tf) = player.single() else { return };
    let player_pos = player_tf.translation;

    for (tf, mut lod) in &mut enemies {
        let dist_sq = (player_pos - tf.translation).length_squared();
        let new_lod = if dist_sq < 225.0 {   // 15²
            EnemyLod::Full
        } else if dist_sq < 625.0 {           // 25²
            EnemyLod::Reduced
        } else {
            EnemyLod::Minimal
        };

        if *lod != new_lod {
            *lod = new_lod;
        }
    }
}

/// Скрывает/показывает ground circle по LOD (только при смене LOD уровня)
pub fn lod_ground_circle_system(
    enemies: Query<(&EnemyLod, &Children), (Changed<EnemyLod>, With<Enemy>)>,
    mut circle_query: Query<&mut Visibility, With<GroundCircle>>,
) {
    for (lod, children) in &enemies {
        let visible = *lod == EnemyLod::Full;
        for child in children.iter() {
            if let Ok(mut vis) = circle_query.get_mut(child) {
                *vis = if visible { Visibility::Inherited } else { Visibility::Hidden };
            }
        }
    }
}

/// Замораживает/размораживает анимации по LOD (только при смене LOD уровня)
pub fn lod_animation_freeze_system(
    enemies: Query<(&EnemyLod, &CachedAnimPlayer), Changed<EnemyLod>>,
    mut animation_query: Query<&mut AnimationPlayer>,
) {
    for (lod, cached) in &enemies {
        let Ok(mut player) = animation_query.get_mut(cached.entity) else { continue };
        match lod {
            EnemyLod::Minimal => { player.pause_all(); },
            _ => { player.resume_all(); },
        }
    }
}
