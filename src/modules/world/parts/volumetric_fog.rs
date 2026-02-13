use bevy::prelude::*;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::light::FogVolume;
use crate::toolkit::asset_paths;

/// Объёмный туман арены — FogVolume с 3D noise текстурой
pub fn setup_volumetric_fog(mut commands: Commands, asset_server: Res<AssetServer>) {
    let noise_texture = asset_server.load_with_settings(
        asset_paths::FOG_NOISE,
        |s: &mut ImageLoaderSettings| {
            s.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                address_mode_w: ImageAddressMode::Repeat,
                mag_filter: ImageFilterMode::Linear,
                min_filter: ImageFilterMode::Linear,
                mipmap_filter: ImageFilterMode::Linear,
                ..default()
            });
        },
    );

    // FogVolume покрывает арену 50×50м, высота ~4м от пола
    commands.spawn((
        Transform::from_xyz(0.0, 2.0, 0.0).with_scale(Vec3::new(50.0, 4.0, 50.0)),
        FogVolume {
            density_texture: Some(noise_texture),
            density_factor: 0.3,              // Повышена — камера сверху, луч проходит всего ~4м
            scattering: 0.5,
            absorption: 0.15,
            fog_color: Color::srgb(0.35, 0.30, 0.45),  // Светлее — пурпурный в тон сцене
            ..default()
        },
    ));
}

/// Скроллинг noise текстуры — эффект ветра
pub fn scroll_fog_system(time: Res<Time>, mut query: Query<&mut FogVolume>) {
    for mut fog in query.iter_mut() {
        fog.density_texture_offset += Vec3::new(0.01, 0.0, 0.02) * time.delta_secs();
    }
}
