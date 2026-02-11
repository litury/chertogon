use bevy::prelude::*;
use crate::modules::player::components::Player;
use crate::modules::enemies::components::{Enemy, EnemyAnimState, EnemyAnim, EnemyDying};
use crate::modules::combat::components::{PlayerHealth, EnemyAttackCooldown};

/// Враг наносит контактный урон игроку когда в состоянии Attacking
pub fn enemy_contact_damage_system(
    time: Res<Time>,
    mut enemies: Query<(&EnemyAnimState, &mut EnemyAttackCooldown), (With<Enemy>, Without<EnemyDying>)>,
    mut player: Query<&mut PlayerHealth, With<Player>>,
) {
    let Ok(mut player_health) = player.single_mut() else { return };

    for (anim_state, mut attack_cd) in &mut enemies {
        if anim_state.current == EnemyAnim::Attacking {
            attack_cd.timer.tick(time.delta());

            if attack_cd.timer.is_finished() {
                player_health.take_damage(attack_cd.damage);
                info!(
                    "💢 Enemy hits Player for {} damage! (HP: {}/{})",
                    attack_cd.damage, player_health.current, player_health.max
                );
                attack_cd.timer.reset();
            }
        } else {
            // Вне атаки — сбрасываем таймер (первый удар будет мгновенным)
            attack_cd.timer.finish();
        }
    }
}
