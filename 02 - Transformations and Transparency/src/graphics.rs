extern crate nalgebra_glm as glm;
use glm::{perspective, rotation, translation, vec3, Mat4};
use std::os::raw::c_void;

use crate::camera::Camera;
use crate::shader;
use crate::util;
use crate::vectors;
use crate::window;

/// Initialize Vertex Array Object, Vertex Buffer Object and
/// Index Buffer Object from vectors of vertices and indices
pub unsafe fn initialize_vao(
    vertices: &Vec<f32>,
    indices: &Vec<u32>,
    colors: &Vec<f32>,
) -> u32 {
    // Create Vertex Attribute Array
    let mut vao_id: u32 = 0;
    gl::GenVertexArrays(1, &mut vao_id);
    gl::BindVertexArray(vao_id);

    // Create and fill Vertex Buffer Object
    let mut vbo_id: u32 = 0;
    gl::GenBuffers(1, &mut vbo_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
    gl::BufferData(
        gl::ARRAY_BUFFER,                   // Target
        util::byte_size_of_array(vertices), // Size
        util::pointer_to_array(vertices),   // Data
        gl::STATIC_DRAW,                    // Usage
    );

    // set vertex attributes
    let vertex_attrbute_index: u32 = 1;
    let nr_of_values = 3;
    gl::VertexAttribPointer(
        vertex_attrbute_index,                 // Index
        nr_of_values,                          // Size
        gl::FLOAT,                             // Type
        gl::FALSE,                             // Normalized
        util::size_of::<f32>() * nr_of_values, // Stride
        util::offset::<c_void>(0),             // Offset
    );
    gl::EnableVertexAttribArray(vertex_attrbute_index);

    // Create and fill Color Buffer Object
    let mut cbo_id: u32 = 0;
    gl::GenBuffers(1, &mut cbo_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, cbo_id);
    gl::BufferData(
        gl::ARRAY_BUFFER,                 // Target
        util::byte_size_of_array(colors), // Size
        util::pointer_to_array(colors),   // Data
        gl::STATIC_DRAW,                  // Usage
    );

    // set color attributes
    let color_attrbute_index: u32 = 2;
    let nr_of_color_values = 4;
    gl::VertexAttribPointer(
        color_attrbute_index,                        // Index
        nr_of_color_values,                          // Size
        gl::FLOAT,                                   // Type
        gl::FALSE,                                   // Normalized
        util::size_of::<f32>() * nr_of_color_values, // Stride
        util::offset::<c_void>(0),                   // Offset
    );
    gl::EnableVertexAttribArray(color_attrbute_index);

    // Create and fill Index Buffer Object
    let mut ibo_id: u32 = 0;
    gl::GenBuffers(1, &mut ibo_id);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo_id);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,          // Target
        util::byte_size_of_array(indices), // Size
        util::pointer_to_array(indices),   // Data
        gl::STATIC_DRAW,                   // Usage
    );
    return vao_id;
}

/// Initialize and fill vertices while storing how many vertices we are
/// going to draw. The links and activates the sahders.
pub unsafe fn setup(vertex_count: &mut i32) -> shader::Shader {
    // Create the vectors
    let mut vertices: Vec<f32> = vec![];
    let mut indices: Vec<u32> = vec![];
    let mut colors: Vec<f32> = vec![];
    // Fill the vectors
    //vectors::three_overlapping_triangles(&mut vertices, &mut indices, &mut colors);
    vectors::three_triangles_in_3d(&mut vertices, &mut indices, &mut colors);
    // count number of vertices we need to draw
    *vertex_count = indices.len() as i32;
    // initialize VAO
    initialize_vao(&vertices, &indices, &colors);

    // Create and link the shader
    let shader = shader::ShaderBuilder::new()
        .attach_file("./shaders/simple.vert")
        .attach_file("./shaders/simple.frag")
        .link();
    // activate the shader
    shader.activate();
    return shader;
}

/// Fill background and draw contants of buffers
pub unsafe fn draw(vertex_count: &i32) {
    gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    // Draw the contents of the buffers
    gl::DrawElements(
        gl::TRIANGLES,             // Mode
        *vertex_count,             // Count
        gl::UNSIGNED_INT,          // Type (u32)
        util::offset::<c_void>(0), // Offset from first index to start drawing
    );
}

/// Use the cameras position and rotation to create a single transformation matrix
/// for the scene.
pub fn global_transformation_matrix(camera: &Camera) -> Mat4 {
    let identity: Mat4 = glm::identity();

    // Translation of scene based on camera position
    let world_translation: Mat4 = translation(&(-camera.position));

    // Rotation based on cameras angle
    let camera_pitch_rotation: Mat4 = rotation(camera.pitch(), &vec3(1.0, 0.0, 0.0));
    let camera_yaw_rotation: Mat4 = rotation(camera.yaw(), &vec3(0.0, 1.0, 0.0));
    let world_rotation: Mat4 = camera_pitch_rotation * camera_yaw_rotation;

    // Translation to enable perspective
    let ratio = window::WINDOW_HEIGHT as f32 / window::WINDOW_WIDTH as f32;
    let perspective: Mat4 = perspective(ratio, 1.0, 1.0, 100.0);
    let perspective_translation: Mat4 = translation(&vec3(0.0, 0.0, -2.0));

    // Merge and return resulting matrix
    return perspective * world_rotation * world_translation * perspective_translation * identity;
}
