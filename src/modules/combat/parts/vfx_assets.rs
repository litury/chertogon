use bevy::prelude::*;

/// Кэшированные ассеты для VFX попаданий (создаются один раз)
#[derive(Resource)]
pub struct HitVfxAssets {
    pub particle_mesh: Handle<Mesh>,
    pub particle_material: Handle<StandardMaterial>,
}

/// Инициализация кэшированных VFX ассетов
pub fn init_hit_vfx_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let particle_mesh = meshes.add(Sphere::new(0.06));
    let particle_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.4, 0.1),
        emissive: LinearRgba::new(8.0, 2.0, 0.3, 1.0),
        unlit: true,
        ..default()
    });

    commands.insert_resource(HitVfxAssets {
        particle_mesh,
        particle_material,
    });
}
