use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;

/// Параметры вихревого эффекта портала — передаются в WGSL шейдер
#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct PortalVortexSettings {
    pub color: LinearRgba,
    pub speed: f32,
    pub twist: f32,
    pub _pad1: f32,
    pub _pad2: f32,
}

/// Кастомный Material для вихревого заполнения порталов
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct PortalVortexMaterial {
    #[uniform(0)]
    pub settings: PortalVortexSettings,
}

impl Material for PortalVortexMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/portal_vortex.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}
