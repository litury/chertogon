use bevy::prelude::*;
/// Камера медленно вращается вокруг арены для атмосферного Title Screen
pub fn menu_camera_orbit_system(
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
) {
    let Ok(mut transform) = camera.single_mut() else { return };

    let t = time.elapsed_secs() * 0.1;
    let radius = 20.0;
    let height = 16.0;
    transform.translation = Vec3::new(t.cos() * radius, height, t.sin() * radius);
    transform.look_at(Vec3::ZERO, Vec3::Y);
}
