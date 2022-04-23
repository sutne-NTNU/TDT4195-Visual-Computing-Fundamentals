use std::f32::consts::PI;

use crate::graphics::initialize_vao;
use crate::mesh::Helicopter;
use crate::scene_graph::{Node, SceneNode};
use crate::util;

pub struct HelicopterNode {
    pub body: Node,
    pub main_rotor: Node,
    pub tail_rotor: Node,
    pub door: Node,

    rotor_speed: f32,

    pub door_is_opening: bool,
    pub door_is_closing: bool,
    pub engine_is_running: bool,
}

/// Default rotor speed when engine is turned on
const DEFAULT_ROTOR_SPEED: f32 = 1.0;
/// Rotor speed needed to lift upwards / takeoff, and therefor also move
/// Meaning that if the rotor speed is below this you are not able to control
/// the helicopter in any way (except the doors)
const MOVEMENT_TRESHOLD: f32 = 0.4;

impl HelicopterNode {
    /// Construct new helicopter with SceneNodes and connect them to the body.
    pub unsafe fn new(heli: &Helicopter) -> HelicopterNode {
        // Body
        let mut heli_body = SceneNode::from_vao(
            initialize_vao(&heli.body.vertices, &heli.body.indices, &heli.body.normals, &heli.body.colors),
            heli.body.index_count,
        );
        // Main Rotor
        let heli_rotor = SceneNode::from_vao(
            initialize_vao(
                &heli.rotor.vertices,
                &heli.rotor.indices,
                &heli.rotor.normals,
                &heli.rotor.colors,
            ),
            heli.rotor.index_count,
        );
        // Tail Rotor
        let mut heli_tail = SceneNode::from_vao(
            initialize_vao(&heli.tail.vertices, &heli.tail.indices, &heli.tail.normals, &heli.tail.colors),
            heli.tail.index_count,
        );
        heli_tail.reference_point = glm::vec3(0.35, 2.3, 10.4);
        // Door
        let heli_door = SceneNode::from_vao(
            initialize_vao(&heli.door.vertices, &heli.door.indices, &heli.door.normals, &heli.door.colors),
            heli.door.index_count,
        );
        // Connect the nodes
        heli_body.add_child(&heli_rotor);
        heli_body.add_child(&heli_tail);
        heli_body.add_child(&heli_door);

        return HelicopterNode {
            body: heli_body,
            main_rotor: heli_rotor,
            tail_rotor: heli_tail,
            door: heli_door,

            door_is_closing: false,
            door_is_opening: false,

            rotor_speed: 0.0,
            engine_is_running: false,
        };
    }

    /*



        Public functions for changing the helicopters rotor speed and tilt with the keyoard



    */

    /// Tip nose downwards
    pub fn tilt_forward(&mut self, delta_time: f32) {
        if self.rotor_speed < MOVEMENT_TRESHOLD {
            return;
        }
        self.body.rotation.x -= delta_time;
    }
    /// Lift nose upwards
    pub fn tilt_backward(&mut self, delta_time: f32) {
        self.tilt_forward(-delta_time);
    }

    /// Tilt the helicopter down on the left side
    pub fn tilt_left(&mut self, delta_time: f32) {
        if self.rotor_speed < MOVEMENT_TRESHOLD {
            return;
        }
        self.body.rotation.z += delta_time;
    }

    /// Tilt the helicopter down on the right side
    pub fn tilt_right(&mut self, delta_time: f32) {
        self.tilt_left(-delta_time);
    }

    /// Yaw the helicopter to the left
    pub fn turn_left(&mut self, delta_time: f32) {
        if self.rotor_speed < MOVEMENT_TRESHOLD {
            return;
        }
        self.body.rotation.y = (self.body.rotation.y + delta_time) % (2.0 * PI);
    }

    /// Yaw the helicopter to the right
    pub fn turn_right(&mut self, delta_time: f32) {
        self.turn_left(-delta_time);
    }

    /// Temporarlily increase the throttle of the engine (and increase altitude)
    pub fn increase_throttle(&mut self, delta_time: f32) {
        if self.rotor_speed < MOVEMENT_TRESHOLD {
            return;
        }
        self.rotor_speed += delta_time * 0.4;
    }

    /// Temporarlily decrease the throttle of the engine (and decrease altitude)
    pub fn decrease_throttle(&mut self, delta_time: f32) {
        self.increase_throttle(-delta_time);
    }

    /*



        Functions for dynamically updating the helicopters positions based on its
        speed and tilt.


    */

    /// Update helicopters positions, doors and tilt
    pub fn update(&mut self, delta_time: f32) {
        // adjust rotor speed based on if the engine is running or not, if the engine is running
        // the rotor_speed should stabilize around default_rotor speed.
        if self.engine_is_running {
            if self.rotor_speed < MOVEMENT_TRESHOLD {
                self.rotor_speed += (self.rotor_speed + 0.01) * 0.7 * delta_time;
            } else {
                self.rotor_speed += (DEFAULT_ROTOR_SPEED - self.rotor_speed) * 0.7 * delta_time;
            }
        } else {
            let decrease = self.rotor_speed * 0.7 * delta_time;
            if self.rotor_speed > decrease {
                self.rotor_speed -= decrease;
            }
        }

        // Rotate rotors according to the rotorspeed
        self.main_rotor.rotation.y = (self.main_rotor.rotation.y + self.rotor_speed * 0.7) % (2.0 * PI);
        self.tail_rotor.rotation.x = (self.main_rotor.rotation.y * 2.0) % (2.0 * PI);

        // Apply "gravity" to the helicopter
        self.body.position.y -= DEFAULT_ROTOR_SPEED * delta_time * 100.0;

        // Increase the altitude again based on how horizontal the helicopter is
        // and the current rotor_speed
        let lift = 1.0 - (0.2 * (self.body.rotation.x.sin().abs() + self.body.rotation.z.sin().abs()));
        self.body.position.y += lift * self.rotor_speed * delta_time * 100.0;

        // Set hard limit to not fall through map
        if self.body.position.y < 0.0 {
            self.body.position.y = 0.0;
        }

        let speed = 50.0 * self.rotor_speed;
        // Move forwards in direction helicopter is tilting and facing
        let xz_forward_direction = glm::vec3(self.yaw().sin(), 0.0, self.yaw().cos());
        self.body.position.x += xz_forward_direction.x * speed * delta_time * self.body.rotation.x;
        self.body.position.z += xz_forward_direction.z * speed * delta_time * self.body.rotation.x;
        // Move sideways based on direction helicopter is tilting and facing
        let xz_right_direction = glm::vec3(self.yaw().cos(), 0.0, -self.yaw().sin());
        self.body.position.x += xz_right_direction.x * speed * delta_time * -self.body.rotation.z;
        self.body.position.z += xz_right_direction.z * speed * delta_time * -self.body.rotation.z;
        self.body.rotation.y += delta_time * self.body.rotation.z;

        // Adjust tilt angles
        self.body.rotation.x = self.body.rotation.x + 1.0 * delta_time * (0.0 - self.body.rotation.x);
        self.body.rotation.z = self.body.rotation.z + 1.3 * delta_time * (0.0 - self.body.rotation.z);

        // Move door if needed
        if self.door_is_opening {
            if self.door.position.z < 1.7 {
                self.door.position.z += 1.7 * delta_time;
            } else {
                self.door_is_opening = false;
            }
        } else if self.door_is_closing {
            if self.door.position.z > 0.0 {
                self.door.position.z -= 1.7 * delta_time;
            } else {
                self.door_is_closing = false;
            }
        }
    }

    /// The yaw of the entire helicopter
    pub fn yaw(&self) -> f32 {
        return self.body.rotation.y;
    }

    /*



       The methods below are just for the animated helicopters in task 6



    */

    pub fn set_heading(&mut self, heading: &util::Heading) {
        self.body.position.x = heading.x;
        self.body.position.z = heading.z;
        self.body.rotation.x = heading.pitch;
        self.body.rotation.y = heading.yaw;
        self.body.rotation.z = heading.roll;
    }

    pub fn spin_rotors(&mut self) {
        self.main_rotor.rotation.y = (self.main_rotor.rotation.y + 1.0) % (2.0 * PI);
        self.tail_rotor.rotation.x = (self.main_rotor.rotation.y + 2.0) % (2.0 * PI);
    }
}
