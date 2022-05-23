extern crate glfw;
use glfw::{Action, Context, Key};

extern crate gl;

extern crate c_str_macro;
use c_str_macro::c_str;

use cgmath::{perspective, vec3, Deg, Matrix4, Point3};

mod shader;
use shader::Shader;

mod camera;
use camera::{Camera, Camera_Movement};

mod mesh;

mod model;
use model::Model;

mod voxelization;

use std::sync::mpsc::Receiver;

// Settings
const SOURCE_WIDTH: u32 = 1200;
const SOURCE_HEIGHT: u32 = 800;

fn main() {
    // Camera setup
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    let mut first_mouse = true;
    let mut last_x: f32 = SOURCE_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SOURCE_HEIGHT as f32 / 2.0;

    // Timing
    let mut delta_time: f32;
    let mut last_frame: f32 = 0.0;

    // GLFW: Setup
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // GLFW: Window creation
    let (mut window, events) = glfw
        .create_window(
            SOURCE_WIDTH,
            SOURCE_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // GL: Load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Set shader program
    let (our_shader, our_model) = unsafe {
        gl::Enable(gl::DEPTH_TEST);

        let our_shader = Shader::new(
            "src/shaders/model_loading.vs",
            "src/shaders/model_loading.fs",
        );

        let our_model = Model::new("assets/cow.obj");

        (our_shader, our_model)
    };

    // Render loop
    while !window.should_close() {
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // Events
        process_events(
            &events,
            &mut first_mouse,
            &mut last_x,
            &mut last_y,
            &mut camera,
        );

        // Input
        process_input(&mut window, delta_time, &mut camera);

        // Render
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            our_shader.useProgram();

            let projection: Matrix4<f32> = perspective(
                Deg(camera.Zoom),
                SOURCE_WIDTH as f32 / SOURCE_HEIGHT as f32,
                0.1,
                10000.0,
            );
            let view = camera.GetViewMatrix();
            our_shader.setMat4(c_str!("projection"), &projection);
            our_shader.setMat4(c_str!("view"), &view);

            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -1.75, 0.0));
            model = model * Matrix4::from_scale(0.2); // i
            our_shader.setMat4(c_str!("model"), &model);
            our_model.Draw(&our_shader);
        }

        // GLFW: Swap buffers and poll I/O events
        window.swap_buffers();
        glfw.poll_events();
    }
}

pub fn process_events(
    events: &Receiver<(f64, glfw::WindowEvent)>,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    camera: &mut Camera,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::CursorPos(x_position, y_position) => {
                let (x_position, y_position) = (x_position as f32, y_position as f32);
                if *first_mouse {
                    *last_x = x_position;
                    *last_y = y_position;
                    *first_mouse = false;
                }

                let x_offset = x_position - *last_x;
                let y_offset = *last_y - y_position; // reversed since y-coordinates go from bottom to top

                *last_x = x_position;
                *last_y = y_position;

                camera.ProcessMouseMovement(x_offset, y_offset, true);
            }
            glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
                camera.ProcessMouseScroll(y_offset as f32);
            }
            _ => {}
        }
    }
}

pub fn process_input(window: &mut glfw::Window, delta_time: f32, camera: &mut Camera) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    if window.get_key(Key::W) == Action::Press {
        camera.ProcessKeyboard(Camera_Movement::Forward, delta_time);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.ProcessKeyboard(Camera_Movement::Backward, delta_time);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.ProcessKeyboard(Camera_Movement::Left, delta_time);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.ProcessKeyboard(Camera_Movement::Right, delta_time);
    }
}
