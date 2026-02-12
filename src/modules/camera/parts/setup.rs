use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use bevy::anti_alias::fxaa::Fxaa;
use crate::shared::constants::{CAMERA_OFFSET_Y, CAMERA_OFFSET_Z};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Fxaa::default(),    // Anti-aliasing
        Bloom {
            intensity: 0.15,
            ..default()
        },
        Transform::from_xyz(0.0, CAMERA_OFFSET_Y, CAMERA_OFFSET_Z)
            .looking_at(Vec3::ZERO, Vec3::Y),
        DistanceFog {
            color: Color::srgb(0.05, 0.04, 0.08),
            falloff: FogFalloff::Linear {
                start: 22.0,
                end: 40.0,
            },
            ..default()
        },
    ));
}
