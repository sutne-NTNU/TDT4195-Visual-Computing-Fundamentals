extern crate nalgebra_glm as glm;
use crate::camera::Camera;
use std::sync::MutexGuard;

pub fn handle(
    mouse_delta: &MutexGuard<(f32, f32)>,
    camera: &mut Camera,
) {
    let sensitivity = 0.005;
    // Change in rotations (mouse-movement) since last frame.
    let delta_pitch = mouse_delta.1 * sensitivity;
    let delta_yaw = mouse_delta.0 * sensitivity;

    camera.update_pitch(delta_pitch);
    camera.update_yaw(delta_yaw);
}
