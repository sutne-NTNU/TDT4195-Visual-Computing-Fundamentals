extern crate nalgebra_glm as glm;
use std::sync::{Arc, RwLock};
use std::thread;
use std::{os::raw::c_void, ptr};

mod shader;
mod util;
mod vectors;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

const SCREEN_W: u32 = 800;
const SCREEN_H: u32 = 800;

/// Initialize Vertex Array Object, Vertex Buffer Object and
/// Index Buffer Object from vectors of vertices and indices
unsafe fn initialize_vao(
    vertices: &Vec<f32>,
    indices: &Vec<u32>,
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
    let vertex_attrbute_index: u32 = 0;
    gl::VertexAttribPointer(
        vertex_attrbute_index,      // Index
        3,                          // Size
        gl::FLOAT,                  // Type
        gl::FALSE,                  // Normalized
        util::size_of::<f32>() * 3, // Stride
        util::offset::<c_void>(0),  // Offset
    );
    gl::EnableVertexAttribArray(vertex_attrbute_index);

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

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("01 - Introduction to OpenGL and Rust")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    let render_thread = thread::spawn(move || {
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        // Set up openGL
        unsafe {
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

        //Keep track of how many vertices we are going to draw
        let vertex_count: i32;

        unsafe {
            // Create the vectors
            let mut vertices: Vec<f32> = vec![];
            let mut indices: Vec<u32> = vec![];
            // Fill the vectors
            vectors::circle(&mut vertices, &mut indices);
            // count number of vertices we need to draw
            vertex_count = vertices.len() as i32;
            // initialize VAO
            initialize_vao(&vertices, &indices);

            // Create and link the shader
            let shader = shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link();
            // activate the shader
            shader.activate();
        }

        loop {
            unsafe {
                gl::ClearColor(
                    13 as f32 / 255.0, // R
                    17 as f32 / 255.0, // G
                    23 as f32 / 255.0, // B
                    1.0,
                );
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // Draw the contents of the buffers
                gl::DrawElements(
                    gl::TRIANGLE_FAN,          // Mode
                    vertex_count,              // Count
                    gl::UNSIGNED_INT,          // Type (u32)
                    util::offset::<c_void>(0), // Offset from first index to start drawing
                );
            }
            context.swap_buffers().unwrap();
        }
    });

    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
