use bevy::prelude::*;
use crate::modules::player::components::{PlayerModel, WeaponModel, WeaponAttachmentComplete};
use crate::toolkit::asset_paths;

/// Имя кости для крепления оружия (из bogatyr_merged.glb, Meshy auto-rig)
const WEAPON_BONE_NAME: &str = "RightHand";

/// Система крепления оружия к кости руки.
/// Паттерн "poll until ready" — бежит каждый кадр пока SceneRoot не заспавнит иерархию костей.
pub fn attach_weapon_to_hand(
    model_query: Query<(Entity, &Children), (With<PlayerModel>, Without<WeaponAttachmentComplete>)>,
    children_query: Query<&Children>,
    name_query: Query<&Name>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (model_entity, model_children) in &model_query {
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
                // Transform подбирается визуально: offset, rotation, scale
                // Кость RightHand направлена вдоль предплечья,
                // поэтому клинок нужно повернуть чтобы торчал из кулака
                Transform::from_xyz(0.0, 0.0, -40.0)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                    .with_scale(Vec3::splat(50.0)),
                WeaponModel,
            )).id();

            commands.entity(bone_entity).add_child(sword);
            commands.entity(model_entity).insert(WeaponAttachmentComplete);

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
