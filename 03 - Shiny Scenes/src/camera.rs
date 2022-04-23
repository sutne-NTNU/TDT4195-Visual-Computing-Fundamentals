extern crate nalgebra_glm as glm;

use crate::helicopter::HelicopterNode;

/// Not that the cameras positions and rotations are based on its "perceived" location.
/// This location must be used to translate/rotate the scene (the camera nevver actually moves)
/// and depending upon if the scene uses the `glm::perspective` transformation, the cameras z-
/// position should be invverted before performing translations/rotations.
pub struct Camera {
    pub position: glm::TVec3<f32>,
    pitch: f32,
    yaw: f32,
    helicopter: *const HelicopterNode,
}

const PI: f32 = std::f32::consts::PI;
const DISTANCE_TO_HELICOPTER: f32 = 25.0;
const PITCH_LIMIT: f32 = PI / 2.0;

impl Camera {
    /// Initilize a new camera that will always look towards the helicopter
    pub fn new(helicopter: &HelicopterNode) -> Camera {
        Camera {
            position: glm::vec3(0.0, 0.0, 0.0),
            pitch: 0.2,
            yaw: 0.0,
            helicopter: helicopter,
        }
    }

    pub fn pitch(&self) -> f32 {
        return self.pitch;
    }

    pub unsafe fn yaw(&self) -> f32 {
        let helicopter = &*self.helicopter;
        return self.yaw - helicopter.yaw();
    }

    /// Update the cameras position relative to the helicopter, this will make sure the
    /// the camera is always pointed directly at the helicopter at `DISTANCE_TO_HELICOPTER` away
    /// from it.
    pub unsafe fn update(&mut self) {
        let helicopter = &*self.helicopter;
        // Place camera in helicopters position
        self.position = glm::vec3(
            helicopter.body.position.x,
            helicopter.body.position.y,
            helicopter.body.position.z,
        );

        // Offset camera to a point on a circle correct distance away from the helicopter
        let x_offset: f32 = DISTANCE_TO_HELICOPTER * self.yaw().sin();
        let z_offset: f32 = DISTANCE_TO_HELICOPTER * self.yaw().cos();
        self.position.x -= x_offset;
        self.position.z += z_offset;

        // do the some for height
        let y_offset = DISTANCE_TO_HELICOPTER * self.pitch.sin();
        self.position.y += y_offset;

        // we also have to move the camera closer to the helicopter again
        // to adjust for the change in height based on the pitch
        let new_radius: f32 = DISTANCE_TO_HELICOPTER - DISTANCE_TO_HELICOPTER * self.pitch.cos();
        let x_offset: f32 = new_radius * self.yaw().sin();
        let z_offset: f32 = new_radius * self.yaw().cos();
        self.position.x += x_offset;
        self.position.z -= z_offset;
    }

    /// Change pitch angle with `delta` amount.
    ///
    /// The pitch angle will be limited by the `pitch_limit` (straight
    /// up or straight down).
    ///
    /// Positive delta will tilt camera up, negative will tilt down.
    pub fn update_pitch(&mut self, delta: f32) {
        let mut new_pitch = self.pitch - delta;
        if new_pitch > PITCH_LIMIT {
            new_pitch = PITCH_LIMIT;
        } else if new_pitch < -PITCH_LIMIT {
            new_pitch = -PITCH_LIMIT;
        }
        self.pitch = new_pitch;
    }

    /// Change yaw angle with `delta` amount.
    ///
    /// The yaw is limited to: `-2*pi < yaw < 2*pi`
    ///
    /// Positive will yaw to the right, negative will yaw to the left.
    pub fn update_yaw(&mut self, delta: f32) {
        let mut new_yaw = self.yaw + delta;
        if new_yaw < 0.0 {
            new_yaw = -(-new_yaw % (2.0 * PI));
        } else {
            new_yaw = new_yaw % (2.0 * PI);
        }
        self.yaw = new_yaw;
    }
}
