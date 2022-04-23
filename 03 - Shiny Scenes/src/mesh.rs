use tobj;

// internal helper
fn generate_color_vec(color: [f32; 4], num: usize) -> Vec<f32> {
    color.iter().cloned().cycle().take(num * 4).collect()
}

pub struct Mesh {
    pub vertices: Vec<f32>,
    pub normals: Vec<f32>,
    pub colors: Vec<f32>,
    pub indices: Vec<u32>,
    pub index_count: i32,
}

impl Mesh {
    pub fn from(mesh: tobj::Mesh, color: [f32; 4]) -> Self {
        let num_verts = mesh.positions.len() / 3;
        let index_count = mesh.indices.len() as i32;
        Mesh {
            vertices: mesh.positions,
            normals: mesh.normals,
            indices: mesh.indices,
            colors: generate_color_vec(color, num_verts),
            index_count,
        }
    }
}

pub struct Terrain;
impl Terrain {
    pub fn load(path: &str) -> Mesh {
        let (models, _materials) = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )
        .expect("Failed to load terrain model");

        if models.len() > 1 || models.len() == 0 {
            panic!("Please use a model with a single mesh!")
            // You could try merging the vertices and indices
            // of the separate meshes into a single mesh.
            // I'll leave that as an optional exercise. ;)
        }

        let terrain = models[0].to_owned();

        Mesh::from(terrain.mesh, [1.0, 1.0, 1.0, 1.0])
    }
}

use std::ops::Index;
pub struct Helicopter {
    pub body: Mesh,
    pub door: Mesh,
    pub rotor: Mesh,
    pub tail: Mesh,
}

// You can use square brackets to access the components of the helicopter, if you want to use loops!
impl Index<usize> for Helicopter {
    type Output = Mesh;
    fn index<'a>(&'a self, i: usize) -> &'a Mesh {
        match i {
            0 => &self.body,
            1 => &self.rotor,
            2 => &self.tail,
            3 => &self.door,
            _ => panic!("Invalid index, try [0,3]"),
        }
    }
}

impl Helicopter {
    pub fn load(path: &str) -> Self {
        let (models, _materials) = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )
        .expect("Failed to load helicopter model");

        let body_model = models
            .iter()
            .find(|m| m.name == "Body_body")
            .expect("Incorrect model file!")
            .to_owned();
        let door_model = models
            .iter()
            .find(|m| m.name == "Door_door")
            .expect("Incorrect model file!")
            .to_owned();
        let main_rotor_model = models
            .iter()
            .find(|m| m.name == "Main_Rotor_main_rotor")
            .expect("Incorrect model file!")
            .to_owned();
        let tail_rotor_model = models
            .iter()
            .find(|m| m.name == "Tail_Rotor_tail_rotor")
            .expect("Incorrect model file!")
            .to_owned();

        Helicopter {
            body: Mesh::from(body_model.mesh, [0.172, 0.192, 0.125, 1.0]),
            door: Mesh::from(door_model.mesh, [0.278, 0.274, 0.121, 1.0]),
            rotor: Mesh::from(main_rotor_model.mesh, [0.282, 0.294, 0.266, 1.0]),
            tail: Mesh::from(tail_rotor_model.mesh, [0.282, 0.294, 0.266, 1.0]),
        }
    }
}
