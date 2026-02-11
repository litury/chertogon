use bevy::prelude::*;
use crate::shared::constants::{CAMERA_OFFSET_Y, CAMERA_OFFSET_Z};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, CAMERA_OFFSET_Y, CAMERA_OFFSET_Z)
            .looking_at(Vec3::ZERO, Vec3::Y),
        DistanceFog {
            color: Color::srgb(0.25, 0.25, 0.31), // #404050 холодный серый туман
            falloff: FogFalloff::Linear {
                start: 30.0,
                end: 50.0,
            },
            ..default()
        },
    ));
}
