extern crate c_str_macro;

use std::env;
use std::path::Path;
use std::ptr;

use c_str_macro::c_str;
extern crate gl;
extern crate glfw;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3};
use gl::types::*;
use glfw::{Action, Context, Key};
use structopt::StructOpt;

mod cli_arguments;
mod config;
mod constants;
mod helpers;
mod rendering;
mod voxelization;

use cli_arguments::Options;
use config::CONFIG;
use helpers::debug::gl_debug_output_callback;
use rendering::{camera::Camera, common, model::Model, shader::Shader};
use voxelization::octree::{build_octree, render_octree};

unsafe fn render_voxel_fragments(
    voxel_position_texture: GLuint,
    voxel_diffuse_texture: GLuint,
    projection: &Matrix4<f32>,
    view: &Matrix4<f32>,
    model: &Matrix4<f32>,
    number_of_voxel_fragments: u32,
    vao: u32,
) {
    gl::Enable(gl::DEPTH_TEST);

    // Set shader program
    let render_voxel_shader = Shader::with_geometry_shader(
        "assets/shaders/voxel_fragment/render_voxel.vert.glsl",
        "assets/shaders/voxel_fragment/render_voxel.frag.glsl",
        "assets/shaders/voxel_fragment/render_voxel.geom.glsl",
    );

    render_voxel_shader.use_program();

    gl::BindImageTexture(
        0,
        voxel_position_texture,
        0,
        gl::TRUE,
        0,
        gl::READ_ONLY,
        gl::RGB10_A2UI,
    );

    gl::BindImageTexture(
        1,
        voxel_diffuse_texture,
        0,
        gl::TRUE,
        0,
        gl::READ_ONLY,
        gl::RGBA8,
    );

    render_voxel_shader.set_mat4(c_str!("projection"), projection);
    render_voxel_shader.set_mat4(c_str!("view"), view);
    render_voxel_shader.set_mat4(c_str!("model"), model);

    render_voxel_shader.set_int(c_str!("voxel_dimension"), CONFIG.voxel_dimension);
    render_voxel_shader.set_float(
        c_str!("half_dimension"),
        1.0 / CONFIG.voxel_dimension as f32,
    );

    render_voxel_shader.set_int(
        c_str!("voxel_fragment_count"),
        number_of_voxel_fragments as i32,
    );

    gl::BindVertexArray(vao);
    gl::DrawArrays(gl::POINTS, 0, number_of_voxel_fragments as i32);
}

fn main() {
    let options = Options::from_args();

    // Camera setup
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, -3.0),
        ..Camera::default()
    };
    let mut first_mouse = true;
    let mut last_x: f32 = CONFIG.viewport_width as f32 / 2.0;
    let mut last_y: f32 = CONFIG.viewport_height as f32 / 2.0;

    // Timing setup
    let mut delta_time: f32;
    let mut last_frame: f32 = 0.0;

    // GLFW: Setup
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(options.debug));

    // GLFW: Window creation
    let (mut window, events) = glfw
        .create_window(
            CONFIG.viewport_width as u32,
            CONFIG.viewport_height as u32,
            "Voxel Cone Tracing",
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

    // Enable OpenGL Debug Context if allowed
    unsafe {
        let mut flags = 0;
        gl::GetIntegerv(gl::CONTEXT_FLAGS, &mut flags);
        if flags as u32 & gl::CONTEXT_FLAG_DEBUG_BIT != 0 {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(gl_debug_output_callback), ptr::null());
            gl::DebugMessageControl(
                gl::DONT_CARE,
                gl::DONT_CARE,
                gl::DONT_CARE,
                0,
                ptr::null(),
                gl::TRUE,
            );
        } else {
            println!("Debug Context not active");
        }
    }

    let (render_model_shader, our_model) = unsafe {
        gl::Enable(gl::DEPTH_TEST);

        let our_shader = Shader::new(
            "assets/shaders/model/model_loading.vert.glsl",
            "assets/shaders/model/model_loading.frag.glsl",
        );

        let previous_current_dir = env::current_dir().unwrap();
        env::set_current_dir(Path::new("assets/models")).unwrap();
        let our_model = Model::new(&options.model);
        env::set_current_dir(previous_current_dir).unwrap();

        (our_shader, our_model)
    };

    let (voxel_position_texture, number_of_voxel_fragments, voxel_diffuse_texture) = unsafe {
        let (number_of_voxel_fragments, voxel_position_texture, voxel_diffuse_texture) =
            voxelization::build_voxel_fragment_list(&options.model);
        dbg!(number_of_voxel_fragments);

        gl::Enable(gl::PROGRAM_POINT_SIZE);
        (
            voxel_position_texture,
            number_of_voxel_fragments,
            voxel_diffuse_texture,
        )
    };

    unsafe {
        build_octree(
            voxel_position_texture,
            number_of_voxel_fragments,
            voxel_diffuse_texture,
        );
    }

    // vao to render voxel fragment list
    let vao = unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);

        vao
    };

    let scene_aabb = &our_model.aabb;
    let aabb_middle_point = scene_aabb.middle_point();
    let aabb_longer_side = scene_aabb.longer_axis_length();

    let center_scene_matrix = cgmath::Matrix4::from_translation(-aabb_middle_point);
    // aabb_longer_side is divided by two and we then use the inverse because
    // NDC coordinates goes from -1 to 1
    let normalize_size_matrix = cgmath::Matrix4::from_scale(2f32 / aabb_longer_side);

    let model_normalization_matrix = normalize_size_matrix * center_scene_matrix;

    // Animation variables
    let mut current_voxel_fragment_count: u32 = 0;
    let mut current_octree_level: u32 = 0;
    let mut show_empty_nodes = false;
    let mut show_model = false;
    let mut show_voxel_fragment_list = false;
    let mut show_octree = false;

    // Render loop
    while !window.should_close() {
        let current_frame = glfw.get_time() as f32;

        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        for (_, event) in glfw::flush_messages(&events) {
            // Events
            common::process_events(
                &event,
                &mut first_mouse,
                &mut last_x,
                &mut last_y,
                &mut camera,
            );
            handle_update_octree_level(&event, &mut current_octree_level, &mut show_empty_nodes);
            handle_showing_entities(
                &event,
                &mut show_model,
                &mut show_voxel_fragment_list,
                &mut show_octree,
            );
        }

        // Input
        common::process_input(&mut window, delta_time, &mut camera);

        // Render
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);

            let projection: Matrix4<f32> = perspective(
                Deg(camera.Zoom),
                CONFIG.viewport_width as f32 / CONFIG.viewport_height as f32,
                0.1,
                10000.0,
            );
            let view = camera.GetViewMatrix();
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
            model = model * Matrix4::from_scale(1.); // i

            if show_voxel_fragment_list {
                render_voxel_fragments(
                    voxel_position_texture,
                    voxel_diffuse_texture,
                    &projection,
                    &view,
                    &model,
                    current_voxel_fragment_count,
                    vao,
                );
            }

            if show_octree {
                render_octree(
                    &model,
                    &view,
                    &projection,
                    current_octree_level as i32,
                    show_empty_nodes,
                );
            }

            if show_model {
                render_model_shader.use_program();
                render_model_shader.use_program();
                render_model_shader.set_mat4(c_str!("projection"), &projection);
                render_model_shader.set_mat4(c_str!("view"), &view);
                render_model_shader.set_mat4(c_str!("model"), &model_normalization_matrix);

                our_model.draw(&render_model_shader);
            }
        }

        current_voxel_fragment_count =
            (current_voxel_fragment_count + 10000).min(number_of_voxel_fragments);

        // GLFW: Swap buffers and poll I/O events
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn handle_update_octree_level(
    event: &glfw::WindowEvent,
    current_octree_level: &mut u32,
    show_empty_nodes: &mut bool,
) {
    match *event {
        glfw::WindowEvent::Key(Key::Left, _, Action::Press, _) => {
            if *current_octree_level != 0 {
                *current_octree_level -= 1
            }
            dbg!(current_octree_level);
        }
        glfw::WindowEvent::Key(Key::Right, _, Action::Press, _) => {
            *current_octree_level = (*current_octree_level + 1).min(CONFIG.octree_levels);
            dbg!(current_octree_level);
        }
        glfw::WindowEvent::Key(Key::M, _, Action::Press, _) => {
            *show_empty_nodes = !*show_empty_nodes;
        }
        _ => {}
    }
}

fn handle_showing_entities(
    event: &glfw::WindowEvent,
    show_model: &mut bool,
    show_voxel_fragment_list: &mut bool,
    show_octree: &mut bool,
) {
    match *event {
        glfw::WindowEvent::Key(Key::Num1, _, Action::Press, _) => {
            *show_model = !*show_model;
        }
        glfw::WindowEvent::Key(Key::Num2, _, Action::Press, _) => {
            *show_voxel_fragment_list = !*show_voxel_fragment_list;
        }
        glfw::WindowEvent::Key(Key::Num3, _, Action::Press, _) => {
            *show_octree = !*show_octree;
        }
        _ => {}
    }
}
