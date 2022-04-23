extern crate nalgebra_glm as glm;
use std::ptr;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

mod camera;
mod graphics;
mod keyboard;
mod mouse;
mod shader;
mod util;
mod vectors;
mod window;

use glutin::event::VirtualKeyCode::{self};

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("02 - Transformations and Transparency")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(window::WINDOW_WIDTH, window::WINDOW_HEIGHT));
    let context_builder = glutin::ContextBuilder::new().with_vsync(true);
    let windowed_context = context_builder.build_windowed(window_builder, &event_loop).unwrap();
    windowed_context
        .window()
        .set_cursor_grab(true)
        .expect("failed to grab cursor");
    windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    let render_thread = thread::spawn(move || {
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        //Keep track of how many vertices we are going to draw
        let mut vertex_count: i32 = 0;
        let shader: shader::Shader;

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

            shader = graphics::setup(&mut vertex_count);
        }

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;

        let mut camera = camera::Camera::new();

        loop {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                keyboard::handle(keys, &delta_time, &mut camera);
            }
            // Handle mouse movement.
            if let Ok(mut delta) = mouse_delta.lock() {
                mouse::handle(&delta, &mut camera);
                *delta = (0.0, 0.0);
            }

            // Set up transformation matrices
            let affine_transformation_matrix: glm::Mat4 = graphics::global_transformation_matrix(&camera);

            unsafe {
                // Pass values to shader
                gl::Uniform1f(shader.get_uniform_location("elapsed"), elapsed.sin());
                gl::UniformMatrix4fv(
                    shader.get_uniform_location("affine_transformation_matrix"),
                    1,
                    0,
                    affine_transformation_matrix.as_ptr(),
                );

                // Draw the scene
                graphics::draw(&vertex_count);
            };
            context.swap_buffers().unwrap();
        }
    });

    // Keep track of the health of the rendering thread
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

    // Handle window and events
    window::run(event_loop, render_thread_healthy, arc_pressed_keys, arc_mouse_delta);
}
