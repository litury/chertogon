use bevy::prelude::*;

/// Кэшированные ассеты для XP/HP орбов (создаются один раз на OnEnter Playing)
#[derive(Resource)]
pub struct OrbAssets {
    pub xp_mesh: Handle<Mesh>,
    pub xp_material: Handle<StandardMaterial>,
    pub hp_mesh: Handle<Mesh>,
    pub hp_material: Handle<StandardMaterial>,
}

/// Инициализация ассетов орбов
pub fn init_orb_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // XP орб: зелёная светящаяся сфера (#00FFAA из GAME_DESIGN)
    let xp_mesh = meshes.add(Sphere::new(0.15));
    let xp_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 1.0, 0.67, 0.9),
        emissive: LinearRgba::new(0.0, 4.0, 2.0, 1.0),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // HP орб: красная светящаяся сфера
    let hp_mesh = meshes.add(Sphere::new(0.18));
    let hp_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.2, 0.1, 0.9),
        emissive: LinearRgba::new(4.0, 0.5, 0.2, 1.0),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    commands.insert_resource(OrbAssets {
        xp_mesh,
        xp_material,
        hp_mesh,
        hp_material,
    });
}
