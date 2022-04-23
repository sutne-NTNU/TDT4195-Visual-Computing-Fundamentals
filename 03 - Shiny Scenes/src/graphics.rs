extern crate nalgebra_glm as glm;
use std::os::raw::c_void;
use std::ptr;

use crate::helicopter::HelicopterNode;
use crate::mesh::{Helicopter, Mesh, Terrain};
use crate::scene_graph::SceneNode;
use crate::shader;
use crate::util;

pub unsafe fn gl_setup() {
    gl::Enable(gl::DEPTH_TEST);
    gl::DepthFunc(gl::LESS);
    gl::Enable(gl::CULL_FACE);
    gl::Disable(gl::MULTISAMPLE);
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
    gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

    println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
    println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
    println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
}

/// Initialize Vertex Array Object, Vertex Buffer Object and
/// Index Buffer Object from vectors of vertices and indices
pub unsafe fn initialize_vao(
    vertices: &Vec<f32>, //
    indices: &Vec<u32>,  //
    normals: &Vec<f32>,  //
    colors: &Vec<f32>,   //
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
    let mut colors_id: u32 = 0;
    gl::GenBuffers(1, &mut colors_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, colors_id);
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

    // Send Normals as well
    let mut normals_id: u32 = 0;
    gl::GenBuffers(1, &mut normals_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, normals_id);
    gl::BufferData(
        gl::ARRAY_BUFFER,                  // Target
        util::byte_size_of_array(normals), // Size
        util::pointer_to_array(normals),   // Data
        gl::STATIC_DRAW,                   // Usage
    );

    let normal_attribute_index: u32 = 3;
    let nr_of_normal_values = 3;
    gl::VertexAttribPointer(
        normal_attribute_index,                       // Index
        nr_of_normal_values,                          // Size
        gl::FLOAT,                                    // Type
        gl::FALSE,                                    // Normalized
        util::size_of::<f32>() * nr_of_normal_values, // Stride
        util::offset::<c_void>(0),                    // Offset
    );
    gl::EnableVertexAttribArray(normal_attribute_index);

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

pub unsafe fn create_shader() -> shader::Shader {
    let shader = shader::ShaderBuilder::new()
        .attach_file("./shaders/simple.vert")
        .attach_file("./shaders/simple.frag")
        .link();
    shader.activate();
    return shader;
}

/// Create the entier scene graph and return our main character (the helicopter)
pub unsafe fn create_scene_graph(root_node: &mut SceneNode) -> Vec<HelicopterNode> {
    // Load the terrain
    let terrain: Mesh = Terrain::load("./resources/lunarsurface.obj");
    // Load the helicopter
    let heli: Helicopter = Helicopter::load("./resources/helicopter.obj");

    // Terrain
    let mut terrain_node = SceneNode::from_vao(
        initialize_vao(&terrain.vertices, &terrain.indices, &terrain.normals, &terrain.colors),
        terrain.index_count,
    );

    // Connect the nodes
    root_node.add_child(&terrain_node);

    // Create 6 helicopters (1 controllable and 5 animated)
    let mut helicopters = vec![];
    for _ in 0..6 {
        let helicopter = HelicopterNode::new(&heli);
        terrain_node.add_child(&helicopter.body);
        helicopters.push(helicopter);
    }
    return helicopters;
}

// Recursively draw the root_node and all its children
pub unsafe fn draw(
    node: &SceneNode,
    view_projection_matrix: &glm::Mat4,
    mvp_matrix_location: i32,
    model_matrix_location: i32,
) {
    // Make sure node is drawable
    if node.index_count > 0 {
        // Construct model-view-projection matrix
        let mvp_transformation_matrix = view_projection_matrix * node.current_transformation_matrix;

        // Send both the mvp matrix (to transform vao) and model matrix (to transform normals) to shader
        gl::UniformMatrix4fv(mvp_matrix_location, 1, 0, mvp_transformation_matrix.as_ptr());
        gl::UniformMatrix4fv(model_matrix_location, 1, 0, node.current_transformation_matrix.as_ptr());

        // Bind and draw the contents of the VAO
        gl::BindVertexArray(node.vao_id);
        gl::DrawElements(
            gl::TRIANGLES,             // Mode
            node.index_count,          // Count
            gl::UNSIGNED_INT,          // Type
            util::offset::<c_void>(0), // Offset
        );
    }

    // Recursively draw all children of this node
    for &child in &node.children {
        draw(&*child, view_projection_matrix, mvp_matrix_location, model_matrix_location);
    }
}
