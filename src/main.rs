extern crate c_str_macro;
use c_str_macro::c_str;
extern crate gl;
extern crate glfw;
use glfw::Context;

use cgmath::{perspective, vec3, Deg, Matrix4, Point3};

mod shader;
use shader::Shader;

mod camera;
use camera::Camera;

mod mesh;

mod model;
use model::Model;

mod common;
mod constants;
mod voxelization;

fn main() {
    // Camera setup
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    let mut first_mouse = true;
    let mut last_x: f32 = constants::SOURCE_WIDTH as f32 / 2.0;
    let mut last_y: f32 = constants::SOURCE_HEIGHT as f32 / 2.0;

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
            constants::SOURCE_WIDTH as u32,
            constants::SOURCE_HEIGHT as u32,
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
            "src/shaders/model_loading.vert.glsl",
            "src/shaders/model_loading.frag.glsl",
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
        common::process_events(
            &events,
            &mut first_mouse,
            &mut last_x,
            &mut last_y,
            &mut camera,
        );

        // Input
        common::process_input(&mut window, delta_time, &mut camera);

        // Render
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // our_shader.useProgram();
            //
            // let projection: Matrix4<f32> = perspective(
            //     Deg(camera.Zoom),
            //     constants::SOURCE_WIDTH as f32 / constants::SOURCE_HEIGHT as f32,
            //     0.1,
            //     10000.0,
            // );
            // let view = camera.GetViewMatrix();
            // our_shader.setMat4(c_str!("projection"), &projection);
            // our_shader.setMat4(c_str!("view"), &view);
            //
            // let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -1.75, 0.0));
            // model = model * Matrix4::from_scale(0.2); // i
            // our_shader.setMat4(c_str!("model"), &model);
            // our_model.Draw(&our_shader);

            voxelization::build_voxel_fragment_list();
        }

        // GLFW: Swap buffers and poll I/O events
        window.swap_buffers();
        glfw.poll_events();
    }
}
