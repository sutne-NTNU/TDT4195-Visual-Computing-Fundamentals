extern crate nalgebra_glm as glm;

use crate::camera::Camera;
use std::sync::MutexGuard;

use glutin::event::VirtualKeyCode::{self};

pub fn handle(
    keys: MutexGuard<Vec<VirtualKeyCode>>,
    delta_time: &f32,
    camera: &mut Camera,
) {
    // Constants
    let distance = 3.5 * delta_time;
    let rotation = 2.5 * delta_time;

    for key in keys.iter() {
        match key {
            VirtualKeyCode::W => camera.move_forward(distance),
            VirtualKeyCode::A => camera.move_left(distance),
            VirtualKeyCode::S => camera.move_backward(distance),
            VirtualKeyCode::D => camera.move_right(distance),
            VirtualKeyCode::Space => camera.move_up(distance),
            VirtualKeyCode::LShift => camera.move_down(distance),

            // Rotation using arrowkeys
            VirtualKeyCode::Up => camera.update_pitch(rotation),
            VirtualKeyCode::Down => camera.update_pitch(-rotation),
            VirtualKeyCode::Left => camera.update_yaw(-rotation),
            VirtualKeyCode::Right => camera.update_yaw(rotation),

            // Reset the camera
            VirtualKeyCode::R => camera.reset(),

            // Ignore all other keys
            _ => {}
        }
    }
}
