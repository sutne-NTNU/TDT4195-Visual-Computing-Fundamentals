extern crate nalgebra_glm as glm;
use glutin::event::VirtualKeyCode::{self};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

mod camera;
mod graphics;
mod helicopter;
mod keyboard;
mod mesh;
mod mouse;
mod scene_graph;
mod shader;
mod transformations;
mod util;
mod window;

use helicopter::HelicopterNode;
use scene_graph::SceneNode;

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("03 - Shiny Scenes")
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
            let context_wrapper = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| context_wrapper.get_proc_address(symbol) as *const _);
            context_wrapper
        };

        let shader: shader::Shader;

        let mut root_node = SceneNode::new();
        let mut helicopter: HelicopterNode;
        let mut helicopters: Vec<HelicopterNode>;

        unsafe {
            graphics::gl_setup();
            shader = graphics::create_shader();
            helicopters = graphics::create_scene_graph(&mut root_node);
            // Extract our main helicopter and set its starting position
            helicopter = helicopters.pop().unwrap();
            helicopter.body.position.z += 100.0;
        };

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;

        let mut camera = camera::Camera::new(&helicopter);

        loop {
            let now = std::time::Instant::now();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            last_frame_time = now;

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                keyboard::handle(keys, delta_time, &mut helicopter);
            }
            // Handle mouse movement.
            if let Ok(mut delta) = mouse_delta.lock() {
                mouse::handle(&delta, &mut camera);
                *delta = (0.0, 0.0);
            }

            unsafe {
                gl::ClearColor(13 as f32 / 255.0, 17 as f32 / 255.0, 23 as f32 / 255.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // Set up transformation matrices
                let view_projection_matrix: glm::Mat4 = transformations::view_projection_matrix(&camera);

                for (i, heli) in helicopters.iter_mut().enumerate() {
                    heli.set_heading(&util::simple_heading_animation(elapsed + 1.5 * i as f32));
                    heli.spin_rotors();
                }

                helicopter.update(delta_time);
                camera.update();

                // Update all node transformations
                transformations::update_node_transformations(&mut root_node, &glm::identity());

                // Draw the scene
                graphics::draw(
                    &root_node,
                    &view_projection_matrix,
                    shader.get_uniform_location("mvp_transformation_matrix"),
                    shader.get_uniform_location("model_transformation_matrix"),
                );
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
