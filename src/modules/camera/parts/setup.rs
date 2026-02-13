use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use bevy::anti_alias::fxaa::Fxaa;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::render::view::{ColorGrading, ColorGradingGlobal, ColorGradingSection};
use crate::shared::constants::{CAMERA_OFFSET_Y, CAMERA_OFFSET_Z};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),

        // MSAA выключен — используем FXAA (MSAA ломает WebGPU)
        Msaa::Off,

        // Tonemapping: AgX — лучший для тёмных сцен
        Tonemapping::AgX,

        // Color Grading — тени/средние/хайлайты
        ColorGrading {
            global: ColorGradingGlobal {
                exposure: 0.2,
                temperature: -0.05,
                post_saturation: 1.0,
                ..default()
            },
            shadows: ColorGradingSection {
                saturation: 0.9,
                contrast: 1.1,
                lift: 0.04,
                ..default()
            },
            highlights: ColorGradingSection {
                saturation: 1.2,
                gain: 0.95,
                ..default()
            },
            ..default()
        },

        // Anti-aliasing (FXAA — безопасен на WebGPU)
        Fxaa::default(),

        // Bloom — ореол вокруг эмиссивных источников
        Bloom {
            intensity: 0.25,
            low_frequency_boost: 0.6,
            high_pass_frequency: 1.0,
            ..default()
        },

        // Atmospheric Fog — работает на WebGPU
        DistanceFog {
            color: Color::srgb(0.10, 0.08, 0.15),
            directional_light_color: Color::srgb(0.15, 0.12, 0.25),
            directional_light_exponent: 12.0,
            falloff: FogFalloff::from_visibility_colors(
                80.0,
                Color::srgb(0.14, 0.10, 0.20),
                Color::srgb(0.08, 0.07, 0.14),
            ),
        },

        Transform::from_xyz(0.0, CAMERA_OFFSET_Y, CAMERA_OFFSET_Z)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Загрузочный overlay — скрывает 3D-сцену пока UI не готов
    commands.spawn((
        crate::modules::menu::LoadingOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.04, 0.08)),
        ZIndex(100),
    ));
}
