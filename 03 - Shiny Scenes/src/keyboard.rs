extern crate nalgebra_glm as glm;

use glutin::event::VirtualKeyCode::{self};
use std::sync::MutexGuard;

use crate::helicopter::HelicopterNode;

pub fn handle(
    keys: MutexGuard<Vec<VirtualKeyCode>>, //
    delta_time: f32,                       //
    helicopter: &mut HelicopterNode,       //
) {
    for key in keys.iter() {
        match key {
            VirtualKeyCode::W => helicopter.tilt_forward(delta_time),
            VirtualKeyCode::A => helicopter.tilt_left(delta_time),
            VirtualKeyCode::S => helicopter.tilt_backward(delta_time),
            VirtualKeyCode::D => helicopter.tilt_right(delta_time),
            VirtualKeyCode::Q => helicopter.turn_left(delta_time),
            VirtualKeyCode::E => helicopter.turn_right(delta_time),

            VirtualKeyCode::Space => helicopter.increase_throttle(delta_time),
            VirtualKeyCode::LShift => helicopter.decrease_throttle(delta_time),

            VirtualKeyCode::Up => helicopter.engine_is_running = true,
            VirtualKeyCode::Down => helicopter.engine_is_running = false,
            VirtualKeyCode::Left => helicopter.door_is_opening = true,
            VirtualKeyCode::Right => helicopter.door_is_closing = true,

            // Ignore all other keys
            _ => {}
        }
    }
}
