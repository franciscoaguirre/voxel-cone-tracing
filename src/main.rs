extern crate c_str_macro;

use std::env;
use std::path::Path;

use c_str_macro::c_str;
use egui_glfw_gl::glfw::{self, Context};
extern crate gl;
use cgmath::{perspective, point3, vec3, Deg, Matrix4, Point3};
use log::info;
use rendering::quad::Quad;
use structopt::StructOpt;

mod cli_arguments;
mod config;
mod constants;
mod helpers;
mod menu;
mod octree;
mod rendering;
mod voxelization;

use cli_arguments::Options;
use config::CONFIG;
use menu::Menu;
use rendering::{
    camera::Camera, common, gizmo::RenderGizmo, light::SpotLight, model::Model, shader::Shader,
};
use voxelization::visualize::RenderVoxelFragmentsShader;

use octree::{BricksToShow, Octree};

fn main() {
    let options = Options::from_args();
    simple_logger::init().unwrap();

    // NOTE: This is true if the binary was compiled in debug mode
    let debug = cfg!(debug_assertions);

    // Camera setup
    let mut camera = Camera::default();
    camera.transform.position = point3(0.0, 0.0, -2.0);
    let mut first_mouse = true;
    let mut last_x: f32 = CONFIG.viewport_width as f32 / 2.0;
    let mut last_y: f32 = CONFIG.viewport_height as f32 / 2.0;

    // Timing setup
    let mut delta_time: f64;
    let mut last_frame: f64 = 0.0;

    let (mut glfw, mut window, events) = unsafe { common::setup_glfw(debug) };

    // FPS variables
    let mut frame_count = 0;
    let mut starting_time: f64 = glfw.get_time();
    let mut elapsed_time: f64;
    let mut fps: f64 = 0.0;

    unsafe {
        common::log_device_information();
    };

    let mut menu = Menu::new(&mut window);

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

    let (
        voxel_positions_texture,
        number_of_voxel_fragments,
        voxel_colors_texture,
        voxel_normals_texture,
    ) = unsafe {
        let (
            number_of_voxel_fragments,
            voxel_positions_texture,
            voxel_colors_texture,
            voxel_normals_texture,
        ) = voxelization::build_voxel_fragment_list(&options.model);
        info!("Number of voxel fragments: {}", number_of_voxel_fragments);

        (
            voxel_positions_texture,
            number_of_voxel_fragments,
            voxel_colors_texture,
            voxel_normals_texture,
        )
    };

    let mut octree = unsafe {
        Octree::new(
            voxel_positions_texture,
            number_of_voxel_fragments,
            voxel_colors_texture,
            voxel_normals_texture,
        )
    };

    let _node_pool = unsafe {
        helpers::get_values_from_texture_buffer(
            octree.textures.node_pool.1,
            number_of_voxel_fragments as usize,
            0_u32,
        )
    };

    let node_positions = unsafe {
        helpers::get_values_from_texture_buffer(
            octree.textures.node_positions.1,
            number_of_voxel_fragments as usize,
            0_u32,
        )
    };

    let node_positions: Vec<String> = node_positions
        .iter()
        .map(|&node_position| helpers::r32ui_to_rgb10_a2ui(node_position))
        .map(|(x, y, z)| format!("({}, {}, {})", x, y, z))
        .collect();

    let scene_aabb = &our_model.aabb;
    let aabb_middle_point = scene_aabb.middle_point();
    let aabb_longer_side = scene_aabb.longer_axis_length();

    let center_scene_matrix = cgmath::Matrix4::from_translation(-aabb_middle_point);
    // aabb_longer_side is divided by two and we then use the inverse because
    // NDC coordinates goes from -1 to 1
    let normalize_size_matrix = cgmath::Matrix4::from_scale(2f32 / aabb_longer_side);

    let model_normalization_matrix = normalize_size_matrix * center_scene_matrix;

    let mut light = unsafe {
        SpotLight::new(
            2.0,
            2.0,
            Point3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        )
    };
    light.transform.position = point3(0.0, 0.0, -1.0);
    light.transform.set_rotation_x(0.0);

    let projection = light.get_projection_matrix();
    let view = light.transform.get_view_matrix();
    let light_view_map = unsafe {
        octree.inject_light(
            &[&our_model],
            &projection,
            &view,
            &model_normalization_matrix,
        )
    };
    let quad = unsafe { Quad::new() };

    // Animation variables
    let mut current_voxel_fragment_count: u32 = 0;
    let mut current_octree_level: u32 = 0;
    let mut show_empty_nodes = false;
    let mut show_model = false;
    let mut show_voxel_fragment_list = false;
    let mut show_octree = false;
    let mut node_filter_text = String::new();

    let mut selected_nodes: Vec<(u32, String)> = Vec::new();
    let mut should_show_neighbors = false;
    let mut bricks_to_show = BricksToShow::default();

    let render_voxel_fragments_shader = RenderVoxelFragmentsShader::init(
        voxel_positions_texture,
        voxel_colors_texture,
        number_of_voxel_fragments,
    );

    // Render loop
    while !window.should_close() {
        let current_frame = glfw.get_time();

        frame_count += 1;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        elapsed_time = current_frame - starting_time;

        if elapsed_time > 1.0 {
            fps = frame_count as f64 / elapsed_time;
            frame_count = 0;
            starting_time = current_frame;
        }

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
        }

        for (_, event) in glfw::flush_messages(&events) {
            // Events
            if let glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) = event {
                menu.toggle_showing(&mut window);
            };
            if !menu.is_showing() {
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
                );
                common::handle_showing_entities(
                    &event,
                    &mut show_model,
                    &mut show_voxel_fragment_list,
                    &mut show_octree,
                );
            }
            menu.handle_event(event);
        }

        menu.begin_frame(current_frame);

        // egui render
        if menu.is_showing() {
            menu.show_main_window();
            if menu.is_showing_node_positions_window() {
                menu.create_node_positions_window(
                    &node_positions,
                    &mut selected_nodes,
                    "Node positions",
                    &mut node_filter_text,
                    &mut should_show_neighbors,
                    &mut bricks_to_show,
                );
            }
            if menu.is_showing_diagnostics_window() {
                menu.create_diagnostics_window(fps);
            }
        }

        // Input
        if !menu.is_showing() {
            common::process_camera_input(&mut window, delta_time as f32, &mut camera);
        }

        // Render
        unsafe {
            let projection: Matrix4<f32> = perspective(
                Deg(camera.zoom),
                CONFIG.viewport_width as f32 / CONFIG.viewport_height as f32,
                0.0001,
                10000.0,
            );
            let view = camera.transform.get_view_matrix();
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
            model = model * Matrix4::from_scale(1.);

            if show_voxel_fragment_list {
                render_voxel_fragments_shader.run(&projection, &view, &model);
            }

            if show_octree {
                octree.render(&model, &view, &projection, current_octree_level);
            }

            octree.set_node_indices(&selected_nodes.iter().map(|(index, _)| *index).collect());
            octree.run_node_positions_shader(&projection, &view, &model);
            octree.set_bricks_to_show(bricks_to_show);

            if should_show_neighbors {
                octree.run_node_neighbors_shader(&projection, &view, &model);
            }

            if bricks_to_show.at_least_one() {
                octree.run_node_bricks_shader(&projection, &view, &model);
            }

            if show_model {
                render_model_shader.use_program();
                render_model_shader.set_mat4(c_str!("projection"), &projection);
                render_model_shader.set_mat4(c_str!("view"), &view);
                render_model_shader.set_mat4(c_str!("model"), &model_normalization_matrix);
                our_model.draw(&render_model_shader);
            }

            light.draw_gizmo(&projection, &view);
            quad.render(light_view_map);
        }

        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }

        menu.end_frame();

        current_voxel_fragment_count =
            (current_voxel_fragment_count + 10000).min(number_of_voxel_fragments);

        // Swap buffers and poll I/O events
        window.swap_buffers();
        glfw.poll_events();
    }
}
