use bevy::prelude::*;
use bevy::anti_alias::fxaa::Fxaa;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::render::view::{ColorGrading, ColorGradingGlobal, ColorGradingSection};
use crate::shared::constants::{CAMERA_OFFSET_Y, CAMERA_OFFSET_Z};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),

        // MSAA выключен — используем FXAA
        Msaa::Off,

        // Tonemapping: AgX — лучший для тёмных сцен
        Tonemapping::AgX,

        // Anti-aliasing (FXAA — лёгкий, работает везде)
        Fxaa::default(),

        // Color Grading — параметры tonemapping шейдера
        ColorGrading {
            global: ColorGradingGlobal {
                exposure: 0.7,
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
            midtones: ColorGradingSection {
                contrast: 1.15,
                saturation: 1.1,
                ..default()
            },
            highlights: ColorGradingSection {
                saturation: 1.2,
                gain: 0.95,
                ..default()
            },
            ..default()
        },

        // Bloom — ореол вокруг эмиссивных источников
        Bloom {
            intensity: 0.25,
            low_frequency_boost: 0.6,
            high_pass_frequency: 1.0,
            ..default()
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
