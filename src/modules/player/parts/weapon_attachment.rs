use bevy::prelude::*;
use crate::modules::player::components::{PlayerModel, WeaponModel};
use crate::toolkit::asset_paths;

/// Имя кости для крепления оружия (из bogatyr_merged.glb, Meshy auto-rig)
const WEAPON_BONE_NAME: &str = "RightHand";

/// Система крепления оружия к кости руки.
/// Бежит каждый кадр — если WeaponModel уже есть в мире, пропускает.
/// Если Bevy пересоздаст SceneRoot, старый WeaponModel удалится вместе с костью,
/// и система автоматически прикрепит оружие к новой иерархии костей.
pub fn attach_weapon_to_hand(
    model_query: Query<&Children, With<PlayerModel>>,
    children_query: Query<&Children>,
    name_query: Query<&Name>,
    weapon_query: Query<(), With<WeaponModel>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    // Оружие уже прикреплено — ничего не делаем
    if !weapon_query.is_empty() {
        return;
    }

    for model_children in &model_query {
        if let Some(bone_entity) = find_named_entity_recursive(
            model_children,
            &children_query,
            &name_query,
            WEAPON_BONE_NAME,
        ) {
            info!("⚔️ Found '{}' bone — attaching weapon", WEAPON_BONE_NAME);

            let scene = asset_server.load(asset_paths::SWORD_MODEL);
            let sword = commands.spawn((
                SceneRoot(scene),
                Transform::from_xyz(0.0, 0.0, -40.0)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                    .with_scale(Vec3::splat(50.0)),
                WeaponModel,
            )).id();

            commands.entity(bone_entity).add_child(sword);

            info!("✅ Weapon attached to {} bone", WEAPON_BONE_NAME);
            return;
        }
    }
}

/// Рекурсивный поиск entity с заданным Name в иерархии потомков
fn find_named_entity_recursive(
    children: &Children,
    children_query: &Query<&Children>,
    name_query: &Query<&Name>,
    target_name: &str,
) -> Option<Entity> {
    for child in children.iter() {
        if let Ok(name) = name_query.get(child) {
            if name.as_str() == target_name {
                return Some(child);
            }
        }
        if let Ok(grandchildren) = children_query.get(child) {
            if let Some(found) = find_named_entity_recursive(
                grandchildren, children_query, name_query, target_name,
            ) {
                return Some(found);
            }
        }
    }
    None
}
