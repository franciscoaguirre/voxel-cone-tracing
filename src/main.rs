// Using unstable feature: integer logarithm
#![feature(int_log)]

extern crate c_str_macro;

use std::env;
use std::path::Path;

use c_str_macro::c_str;
extern crate gl;
extern crate glfw;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3};
use glfw::Context;
use log::info;
use structopt::StructOpt;

mod cli_arguments;
mod config;
mod constants;
mod helpers;
mod rendering;
mod voxelization;

use cli_arguments::Options;
use config::CONFIG;
use rendering::{camera::Camera, common, model::Model, shader::Shader};
use voxelization::{
    octree::{build_octree, render_octree, visualize::ShowBricks},
    render_voxel_fragments,
};

fn main() {
    let options = Options::from_args();
    simple_logger::init().unwrap();

    // NOTE: This is true if the binary was compiled in debug mode
    let debug = cfg!(debug_assertions);

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

    let (mut glfw, mut window, events) = unsafe { common::setup_glfw(debug) };

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

    let (voxel_positions_texture, number_of_voxel_fragments, voxel_colors_texture) = unsafe {
        let (number_of_voxel_fragments, voxel_positions_texture, voxel_colors_texture) =
            voxelization::build_voxel_fragment_list(&options.model);
        info!("Number of voxel fragments: {}", number_of_voxel_fragments);

        (
            voxel_positions_texture,
            number_of_voxel_fragments,
            voxel_colors_texture,
        )
    };

    unsafe {
        build_octree(
            voxel_positions_texture,
            number_of_voxel_fragments,
            voxel_colors_texture,
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
    let mut show_bricks = ShowBricks::DontShow;
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
            common::handle_update_octree_level(
                &event,
                &mut current_octree_level,
                &mut show_empty_nodes,
                &mut show_bricks,
            );
            common::handle_showing_entities(
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
                0.0001,
                10000.0,
            );
            let view = camera.GetViewMatrix();
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
            model = model * Matrix4::from_scale(1.);

            if show_voxel_fragment_list {
                render_voxel_fragments(
                    voxel_positions_texture,
                    voxel_colors_texture,
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
                    current_octree_level,
                    show_empty_nodes,
                    voxel_positions_texture,
                    number_of_voxel_fragments,
                    &show_bricks,
                );
            }

            if show_model {
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
