use bevy::prelude::*;
use bevy::light::NotShadowCaster;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;

/// Параметры тумана — передаются в шейдер (vertex + fragment)
#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct GroundFogSettings {
    pub fog_color: LinearRgba,
    pub time: f32,
    pub speed: f32,
    pub max_height: f32,
    pub density: f32,
    pub layer_index: f32,
    pub _pad1: f32,
    pub _pad2: f32,
    pub _pad3: f32,
}

/// Material для дымки: vertex displacement + alpha fade
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct GroundFogMaterial {
    #[uniform(0)]
    pub settings: GroundFogSettings,
}

impl Material for GroundFogMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/ground_fog.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/ground_fog.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

/// Конфиг слоёв: (высота Y, max подъём, плотность, скорость)
const FOG_LAYERS: [(f32, f32, f32, f32); 2] = [
    (0.05, 1.4, 0.40, 0.20),  // Нижний — плотный, высокий, медленный
    (0.10, 0.9, 0.25, 0.35),  // Верхний — легче, ниже, быстрее
];

/// Спавн двух слоёв тумана с subdivided mesh (vertex displacement)
pub fn setup_ground_fog(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GroundFogMaterial>>,
) {
    // 64×64 вершин = ~8000 треугольников — ничтожно для GPU
    let fog_mesh = meshes.add(
        Plane3d::default()
            .mesh()
            .size(52.0, 52.0)
            .subdivisions(63),
    );

    for (i, &(y, max_h, density, speed)) in FOG_LAYERS.iter().enumerate() {
        let mat = materials.add(GroundFogMaterial {
            settings: GroundFogSettings {
                fog_color: LinearRgba::new(0.25, 0.20, 0.45, 1.0),
                time: 0.0,
                speed,
                max_height: max_h,
                density,
                layer_index: i as f32,
                _pad1: 0.0,
                _pad2: 0.0,
                _pad3: 0.0,
            },
        });

        commands.spawn((
            Mesh3d(fog_mesh.clone()),
            MeshMaterial3d(mat),
            Transform::from_xyz(0.0, y, 0.0),
            NotShadowCaster,
        ));
    }
}

/// Обновление time uniform для анимации displacement
pub fn update_ground_fog_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<GroundFogMaterial>>,
) {
    let t = time.elapsed_secs();
    for (_, material) in materials.iter_mut() {
        material.settings.time = t;
    }
}
