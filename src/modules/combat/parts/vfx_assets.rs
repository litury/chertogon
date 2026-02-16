use bevy::prelude::*;
use crate::toolkit::asset_paths;

/// Кэшированные ассеты для VFX попаданий (создаются один раз)
#[derive(Resource)]
pub struct HitVfxAssets {
    pub particle_mesh: Handle<Mesh>,
    pub particle_material: Handle<StandardMaterial>,
    /// Shared mesh+material для impact flash (вместо PointLight)
    pub flash_mesh: Handle<Mesh>,
    pub flash_material: Handle<StandardMaterial>,
    /// Кэшированный шрифт для floating text (damage numbers, XP, heal)
    pub font: Handle<Font>,
}

/// Инициализация кэшированных VFX ассетов
pub fn init_hit_vfx_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let particle_mesh = meshes.add(Sphere::new(0.06));
    let particle_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.4, 0.1),
        emissive: LinearRgba::new(8.0, 2.0, 0.3, 1.0),
        unlit: true,
        ..default()
    });

    // Impact flash — emissive сфера вместо PointLight (дешевле для renderer)
    let flash_mesh = meshes.add(Sphere::new(0.3));
    let flash_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.7, 0.3, 0.8),
        emissive: LinearRgba::new(15.0, 8.0, 2.0, 1.0),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let font = asset_server.load(asset_paths::FONT_UI_BOLD);

    commands.insert_resource(HitVfxAssets {
        particle_mesh,
        particle_material,
        flash_mesh,
        flash_material,
        font,
    });
}
