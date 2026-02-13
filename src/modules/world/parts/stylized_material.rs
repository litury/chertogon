use bevy::prelude::*;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;

use crate::modules::player::components::PlayerModel;
use crate::modules::enemies::components::EnemyModel;

/// Настройки rim light — передаются в шейдер как uniform
#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct RimLightSettings {
    pub color: LinearRgba,
    pub power: f32,
}

/// MaterialExtension для стилизованного PBR с rim light
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct RimLightExtension {
    #[uniform(100)]
    pub settings: RimLightSettings,
}

impl MaterialExtension for RimLightExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/stylized_pbr.wgsl".into()
    }
}

/// Тип нашего стилизованного материала
pub type StylizedMaterial = ExtendedMaterial<StandardMaterial, RimLightExtension>;

/// Маркер: модель уже обработана системой замены материалов
#[derive(Component)]
pub struct MaterialReplaced;

/// Система замены StandardMaterial → StylizedMaterial на моделях персонажей
/// Запускается каждый кадр, обрабатывает только новые модели (без MaterialReplaced)
pub fn replace_character_materials(
    mut commands: Commands,
    // Ищем PlayerModel/EnemyModel без маркера MaterialReplaced
    models_query: Query<
        Entity,
        (Or<(With<PlayerModel>, With<EnemyModel>)>, Without<MaterialReplaced>),
    >,
    children_query: Query<&Children>,
    mesh_query: Query<(Entity, &MeshMaterial3d<StandardMaterial>)>,
    standard_materials: Res<Assets<StandardMaterial>>,
    mut stylized_materials: ResMut<Assets<StylizedMaterial>>,
) {
    for model_entity in &models_query {
        let mut found_any = false;

        // Обходим иерархию children (GLB создаёт вложенную иерархию)
        for descendant in children_query.iter_descendants(model_entity) {
            if let Ok((mesh_entity, mat_handle)) = mesh_query.get(descendant) {
                if let Some(base_material) = standard_materials.get(&mat_handle.0) {
                    let stylized = StylizedMaterial {
                        base: base_material.clone(),
                        extension: RimLightExtension {
                            settings: RimLightSettings {
                                color: LinearRgba::new(0.4, 0.4, 0.8, 0.6),
                                power: 3.0,
                            },
                        },
                    };

                    let new_handle = stylized_materials.add(stylized);

                    commands.entity(mesh_entity)
                        .remove::<MeshMaterial3d<StandardMaterial>>()
                        .insert(MeshMaterial3d(new_handle));

                    found_any = true;
                }
            }
        }

        if found_any {
            commands.entity(model_entity).insert(MaterialReplaced);
        }
    }
}
