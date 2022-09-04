extern crate c_str_macro;
use std::{ffi::c_void, mem::size_of};

use c_str_macro::c_str;
extern crate gl;
extern crate glfw;
use gl::types::*;
use glfw::Context;

use cgmath::{perspective, vec3, Deg, Matrix4, Point3};

mod rendering;
use rendering::{camera::Camera, common, model::Model, shader::Shader};
use voxelization::octree::{build_octree, render_octree};

mod constants;
mod voxelization;

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
        "src/shaders/voxel_fragment/render_voxel.vert.glsl",
        "src/shaders/voxel_fragment/render_voxel.frag.glsl",
        "src/shaders/voxel_fragment/render_voxel.geom.glsl",
    );

    render_voxel_shader.useProgram();

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

    render_voxel_shader.setMat4(c_str!("projection"), projection);
    render_voxel_shader.setMat4(c_str!("view"), view);
    render_voxel_shader.setMat4(c_str!("model"), model);

    render_voxel_shader.setInt(c_str!("voxel_dimension"), constants::VOXEL_DIMENSION);
    render_voxel_shader.setFloat(
        c_str!("half_dimension"),
        1.0 / constants::VOXEL_DIMENSION as f32,
    );

    render_voxel_shader.setInt(
        c_str!("voxel_fragment_count"),
        number_of_voxel_fragments as i32,
    );

    gl::BindVertexArray(vao);
    gl::DrawArrays(gl::POINTS, 0, number_of_voxel_fragments as i32);

    let _ = gl::GetError();
}

fn main() {
    // Camera setup
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, -3.0),
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

    let (render_model_shader, _our_model) = unsafe {
        gl::Enable(gl::DEPTH_TEST);

        let our_shader = Shader::new(
            "src/shaders/model/model_loading.vert.glsl",
            "src/shaders/model/model_loading.frag.glsl",
        );

        let our_model = Model::new("assets/triangle.obj");

        (our_shader, our_model)
    };

    let (voxel_position_texture, number_of_voxel_fragments, voxel_diffuse_texture) = unsafe {
        let (number_of_voxel_fragments, voxel_position_texture, voxel_diffuse_texture) =
            voxelization::build_voxel_fragment_list();
        dbg!(number_of_voxel_fragments);

        gl::Enable(gl::PROGRAM_POINT_SIZE);
        (
            voxel_position_texture,
            number_of_voxel_fragments,
            voxel_diffuse_texture,
        )
    };

    unsafe {
        build_octree(voxel_position_texture, number_of_voxel_fragments);
    }

    // vao to render voxel fragment list
    let vao = unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);

        vao
    };

    let mut current_voxel_fragment_count = 0;

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
            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            //gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            //gl::Enable(gl::BLEND);
            //gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

            let projection: Matrix4<f32> = perspective(
                Deg(camera.Zoom),
                constants::SOURCE_WIDTH as f32 / constants::SOURCE_HEIGHT as f32,
                0.1,
                10000.0,
            );
            let view = camera.GetViewMatrix();
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
            model = model * Matrix4::from_scale(1.); // i

            //render_model_shader.useProgram();
            // Not using cow model, using voxel fragment list
            // render_model_shader.useProgram();
            // render_model_shader.setMat4(c_str!("projection"), &projection);
            // render_model_shader.setMat4(c_str!("view"), &view);
            // render_model_shader.setMat4(c_str!("model"), &model);

            //our_model.Draw(&render_model_shader);
            render_voxel_fragments(
                voxel_position_texture,
                voxel_diffuse_texture,
                &projection,
                &view,
                &model,
                current_voxel_fragment_count,
                vao,
            );

            //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            render_octree(&model, &view, &projection);
        }

        current_voxel_fragment_count =
            (current_voxel_fragment_count + 100).min(number_of_voxel_fragments);

        // GLFW: Swap buffers and poll I/O events
        window.swap_buffers();
        glfw.poll_events();
    }
}
