extern crate c_str_macro;
use std::{ffi::c_void, mem::size_of};

use c_str_macro::c_str;
extern crate gl;
extern crate glfw;
use gl::types::GLfloat;
use glfw::Context;

use cgmath::{perspective, vec3, Deg, Matrix4, Point3};

mod shader;
use shader::Shader;

mod camera;
use camera::Camera;

mod mesh;

mod AABB;

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
    let (our_shader, _our_model) = unsafe {
        gl::Enable(gl::DEPTH_TEST);

        let our_shader = Shader::with_geometry_shader(
            "src/shaders/render_voxel.vert.glsl",
            "src/shaders/render_voxel.frag.glsl",
            "src/shaders/render_voxel.geom.glsl",
        );
         //let our_shader = Shader::new(
             //"src/shaders/model_loading.vert.glsl",
             //"src/shaders/model_loading.frag.glsl",
         //);

        let our_model = Model::new("assets/sponza.obj");

        (our_shader, our_model)
    };

    // let mut point_cube = 0;
    // unsafe {
    //     let mut data: Vec<f32> = vec![
    //         0.0;
    //         3usize
    //             * constants::VOXEL_DIMENSION as usize
    //             * constants::VOXEL_DIMENSION as usize
    //             * constants::VOXEL_DIMENSION as usize
    //     ];
    //
    //     let mut y_offset: usize;
    //     let mut offset: usize;
    //     for y in 0..constants::VOXEL_DIMENSION {
    //         y_offset = (y as usize)
    //             * (constants::VOXEL_DIMENSION as usize)
    //             * (constants::VOXEL_DIMENSION as usize);
    //
    //         for z in 0..constants::VOXEL_DIMENSION {
    //             offset = y_offset + z as usize * constants::VOXEL_DIMENSION as usize;
    //
    //             for x in 0..constants::VOXEL_DIMENSION {
    //                 data[3 * (offset + x as usize)] = x as f32 / constants::VOXEL_DIMENSION as f32;
    //                 data[3 * (offset + x as usize) + 1] =
    //                     y as f32 / constants::VOXEL_DIMENSION as f32;
    //                 data[3 * (offset + x as usize) + 2] =
    //                     z as f32 / constants::VOXEL_DIMENSION as f32;
    //             }
    //         }
    //     }
    //
    //     gl::GenBuffers(1, &mut point_cube);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, point_cube);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         size_of::<f32>() as isize
    //             * 3isize
    //             * constants::VOXEL_DIMENSION as isize
    //             * constants::VOXEL_DIMENSION as isize
    //             * constants::VOXEL_DIMENSION as isize,
    //         data.as_ptr() as *const c_void,
    //         gl::STATIC_DRAW,
    //     );
    //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    // };

    let number_of_voxel_fragments;
    let voxel_position_texture;

    unsafe {
        let return_value = voxelization::build_voxel_fragment_list();
        number_of_voxel_fragments = return_value.0;
        voxel_position_texture = return_value.1;

        gl::Enable(gl::PROGRAM_POINT_SIZE);
    }

    const NUMBER_OF_VERTICES: usize = 32946;
    // const NUMBER_OF_VERTICES: usize = 105077;
    // const NUMBER_OF_VERTICES: usize = 26302;

    // vao to render voxel fragment list
    let vao = unsafe {
        let vertices: [f32; NUMBER_OF_VERTICES] = [0.0; NUMBER_OF_VERTICES];

        let (mut vao, mut vbo) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of::<[f32; NUMBER_OF_VERTICES]>() as isize,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * size_of::<GLfloat>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        vao
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
            gl::Enable(gl::DEPTH_TEST);

            our_shader.useProgram();

            let projection: Matrix4<f32> = perspective(
                Deg(camera.Zoom),
                constants::SOURCE_WIDTH as f32 / constants::SOURCE_HEIGHT as f32,
                0.1,
                10000.0,
            );
            let view = camera.GetViewMatrix();
            our_shader.setMat4(c_str!("projection"), &projection);
            our_shader.setMat4(c_str!("view"), &view);

            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
            model = model * Matrix4::from_scale(1.); // i
            our_shader.setMat4(c_str!("model"), &model);

            our_shader.setInt(c_str!("voxel_dimension"), constants::VOXEL_DIMENSION);
            our_shader.setFloat(
                c_str!("half_dimension"),
                100.0 / constants::VOXEL_DIMENSION as f32,
            );

            // Not using cow model, using voxel fragment list
            // our_model.Draw(&our_shader);

            // TODO: Not rendering anything
            // gl::BindBuffer(gl::ARRAY_BUFFER, point_cube);
            // gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null::<c_void>());
            // gl::EnableVertexAttribArray(0);
            // gl::DrawArrays(
            //     gl::POINTS,
            //     0,
            //     constants::VOXEL_DIMENSION
            //         * constants::VOXEL_DIMENSION
            //         * constants::VOXEL_DIMENSION,
            // );

            gl::BindImageTexture(
                0,
                voxel_position_texture,
                0,
                gl::TRUE,
                0,
                gl::READ_ONLY,
                gl::RGB10_A2UI,
            );
            our_shader.setInt(c_str!("voxel_position_texture"), 0);

            our_shader.setInt(
                c_str!("voxel_fragment_count"),
                number_of_voxel_fragments as i32,
            );

            // let mut empty_vao = 0;
            // gl::GenVertexArrays(1, &mut empty_vao);
            // gl::BindVertexArray(empty_vao);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::POINTS, 0, NUMBER_OF_VERTICES as i32);
            // gl::DrawArrays(gl::POINTS, 0, 3);

            // let error = gl::GetError();
            // dbg!(error);
        }

        // GLFW: Swap buffers and poll I/O events
        window.swap_buffers();
        glfw.poll_events();
    }
}
