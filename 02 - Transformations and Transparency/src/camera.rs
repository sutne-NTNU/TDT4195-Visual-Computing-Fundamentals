extern crate nalgebra_glm as glm;

/// Not that the cameras positions and rotations are based on its "perceived" location.
/// This location must be used to translate/rotate the scene (the camera nevver actually moves)
/// and depending upon if the scene uses the `glm::perspective` transformation, the cameras z-
/// position should be invverted before performing translations/rotations.
pub struct Camera {
    pub position: glm::TVec3<f32>,
    rotation: glm::TVec3<f32>,
}

const PITCH: usize = 0;
const YAW: usize = 1;
const PI: f32 = std::f32::consts::PI;
const PITCH_LIMIT: f32 = PI / 2.0;

impl Camera {
    /// Initilize a new camera looking down the z-axis placed in the
    /// position (0, 0, 0).
    pub fn new() -> Camera {
        Camera {
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
        }
    }

    /// Set position and rotation of camera back to default.
    pub fn reset(&mut self) {
        self.position = glm::vec3(0.0, 0.0, 0.0);
        self.rotation = glm::vec3(0.0, 0.0, 0.0);
    }

    pub fn pitch(&self) -> f32 {
        return self.rotation[PITCH];
    }

    pub fn yaw(&self) -> f32 {
        return self.rotation[YAW];
    }

    /// Change pitch angle with `delta` amount.
    ///
    /// The pitch angle will be limited by the `pitch_limit` (straight
    /// up or straight down).
    ///
    /// Positive delta will tilt camera up, negative will tilt down.
    pub fn update_pitch(
        &mut self,
        delta: f32,
    ) {
        let mut new_pitch = self.rotation[PITCH] - delta;
        if new_pitch > PITCH_LIMIT {
            new_pitch = PITCH_LIMIT;
        } else if new_pitch < -PITCH_LIMIT {
            new_pitch = -PITCH_LIMIT;
        }
        self.rotation[PITCH] = new_pitch;
    }

    /// Change yaw angle with `delta` amount.
    ///
    /// The yaw is limited to: `-2*pi < yaw < 2*pi`
    ///
    /// Positive will yaw to the right, negative will yaw to the left.
    pub fn update_yaw(
        &mut self,
        delta: f32,
    ) {
        let mut new_yaw = self.rotation[YAW] + delta;
        if new_yaw < 0.0 {
            new_yaw = -(-new_yaw % (2.0 * PI));
        } else {
            new_yaw = new_yaw % (2.0 * PI);
        }
        self.rotation[YAW] = new_yaw;
    }

    /// Get normalized direction vector of direction
    /// the camera is currently pointing.
    pub fn get_direction(&self) -> glm::TVec3<f32> {
        let pitch = self.rotation[PITCH];
        let yaw = self.rotation[YAW] + (PI / 2.0);

        let xz_len = pitch.cos();
        let x = xz_len * yaw.cos();
        let y = pitch.sin();
        let z = xz_len * yaw.sin();

        let mut direction = glm::vec3(x, y, z);
        direction = glm::normalize(&direction);
        return direction;
    }

    /// Change the `position` vector given with the `distance` specified
    /// in the direction of the the `direction` vector
    fn move_position(
        &mut self,
        mut direction: glm::TVec3<f32>,
        distance: f32,
    ) {
        direction = glm::normalize(&mut direction);
        self.position.x += direction.x * distance;
        self.position.y += direction.y * distance;
        self.position.z += direction.z * distance;
    }

    // Convenience functions that just use functions above

    /// movve the camera directly forward. ie. stright in the direction the
    /// camera is facing.
    pub fn move_forward(
        &mut self,
        distance: f32,
    ) {
        self.move_position(self.get_direction(), -distance);
    }

    /// move camera backwards (reverse of forward)
    pub fn move_backward(
        &mut self,
        distance: f32,
    ) {
        self.move_forward(-distance);
    }

    /// Move camera straight right of direction camera is looking
    pub fn move_right(
        &mut self,
        distance: f32,
    ) {
        let camera_direction = self.get_direction();
        // take cameras direction and ignore y-value (moving right/left maintains the same altiditude)
        let xz_forward_direction: glm::TVec3<f32> = glm::vec3(camera_direction.x, 0.0, camera_direction.z);
        // rotate vector 90 degrees to the right, to move right instead of forwards
        let direction = glm::rotate_vec3(&xz_forward_direction, PI / 2.0, &glm::vec3(0.0, 1.0, 0.0));
        self.move_position(direction, distance);
    }

    /// Move camera straight left of direction camera is looking
    pub fn move_left(
        &mut self,
        distance: f32,
    ) {
        self.move_right(-distance);
    }

    /// Move camera straight up (in y-direction)
    pub fn move_up(
        &mut self,
        distance: f32,
    ) {
        let direction: glm::TVec3<f32> = glm::vec3(0.0, 1.0, 0.0);
        self.move_position(direction, distance);
    }

    /// Move camera striaght down (in -y-direction)
    pub fn move_down(
        &mut self,
        distance: f32,
    ) {
        self.move_up(-distance);
    }
}
