extern crate c_str_macro;

use c_str_macro::c_str;
use egui_glfw_gl::glfw::{self, Context};
extern crate gl;
use cgmath::{perspective, point3, vec3, Deg, InnerSpace, Matrix4, Point3};
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
mod types;
mod voxelization;

use cli_arguments::Options;
use config::CONFIG;
use menu::Menu;
use rendering::{camera::Camera, common, gizmo::RenderGizmo, light::SpotLight, shader::Shader};
use voxelization::visualize::RenderVoxelFragmentsShader;

use octree::{BricksToShow, Octree};

use crate::{
    menu::DebugNode,
    rendering::{framebuffer::Framebuffer, transform::Transform},
};

fn main() {
    let options = Options::from_args();
    simple_logger::init().unwrap();

    // NOTE: This is true if the binary was compiled in debug mode
    let debug = cfg!(debug_assertions);

    // Timing setup
    let mut delta_time: f64;
    let mut last_frame: f64 = 0.0;

    let (mut glfw, mut window, events) = unsafe { common::setup_glfw(debug) };

    // Camera setup
    let mut camera = Camera::default();
    // camera.transform.position = point3(0.0, -0.25, 0.0);
    // camera.transform.position = point3(0.0, 0.0, -2.0);
    camera.transform.position = point3(0.0, 0.0, 2.0);
    camera.transform.set_rotation_y(-90.0);
    let mut first_mouse = true;
    let mut last_x: f32 = CONFIG.viewport_width as f32 / 2.0;
    let mut last_y: f32 = CONFIG.viewport_height as f32 / 2.0;

    // Static eye
    let mut static_eye = Transform::default();
    // static_eye.position = point3(0.0, -0.25, 0.0);
    static_eye.position = point3(0.0, 0.0, -2.0);
    // static_eye.set_rotation_x(-60.0);
    // static_eye.set_rotation_y(0.0);

    // FPS variables
    let mut frame_count = 0;
    let mut starting_time: f64 = glfw.get_time();
    let mut elapsed_time: f64;
    let mut fps: f64 = 0.0;

    unsafe {
        common::log_device_information();
    };

    let mut menu = Menu::new(&mut window);

    let render_model_shader = Shader::with_geometry_shader(
        "assets/shaders/model/modelLoading.vert.glsl",
        "assets/shaders/model/modelLoading.frag.glsl",
        "assets/shaders/model/modelLoading.geom.glsl",
    );
    let render_normals_shader = Shader::with_geometry_shader(
        "assets/shaders/model/renderNormals.vert.glsl",
        "assets/shaders/model/renderNormals.frag.glsl",
        "assets/shaders/model/renderNormals.geom.glsl",
    );
    let voxel_cone_tracing_shader = Shader::new_single("assets/shaders/octree/coneTracing.glsl");
    let debug_cone_shader = Shader::new_single("assets/shaders/debug/debugConeTracing.glsl");
    let our_model = unsafe { helpers::load_model(&options.model) };

    let scene_aabb = &our_model.aabb;
    let aabb_middle_point = scene_aabb.middle_point();
    let aabb_longer_side = scene_aabb.longer_axis_length();

    let center_scene_matrix = cgmath::Matrix4::from_translation(-aabb_middle_point);
    // aabb_longer_side is divided by two and we then use the inverse because
    // NDC coordinates goes from -1 to 1
    let normalize_size_matrix = cgmath::Matrix4::from_scale(2f32 / aabb_longer_side);

    let model_normalization_matrix = normalize_size_matrix * center_scene_matrix;

    let (voxel_positions, number_of_voxel_fragments, voxel_colors, voxel_normals) =
        unsafe { voxelization::build_voxel_fragment_list(&our_model) };
    info!("Number of voxel fragments: {}", number_of_voxel_fragments);

    let mut octree = unsafe {
        Octree::new(
            voxel_positions,
            number_of_voxel_fragments,
            voxel_colors,
            voxel_normals,
        )
    };

    let node_positions = unsafe {
        helpers::get_values_from_texture_buffer(
            octree.textures.node_positions.1,
            octree.number_of_nodes(),
            0_u32,
        )
    };

    let debug_nodes: Vec<DebugNode> = node_positions
        .iter()
        .enumerate()
        .map(|(index, &node_position)| {
            let position = helpers::r32ui_to_rgb10_a2ui(node_position);
            let text = format!("({}, {}, {})", position.0, position.1, position.2);
            DebugNode::new(index as u32, text)
        })
        .collect();
    let mut selected_debug_nodes: Vec<DebugNode> = Vec::new();
    let mut selected_debug_nodes_updated = false;
    let mut photons: Vec<u32> = Vec::new();
    let mut children: Vec<u32> = Vec::new();

    let mut light = unsafe {
        SpotLight::new(
            2.0,
            2.0,
            Point3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            1_000_000.0,
        )
    };
     light.transform.position = point3(0.0, 0.00, 2.0);
     light.transform.set_rotation_y(-90.0);
    //light.transform.position = point3(0.0, 1.0, -0.4);
    //light.transform.set_rotation_x(-75.0);

    let light_framebuffer = unsafe { Framebuffer::new_light() };
    let mut light_maps = unsafe {
        octree.inject_light(
            &[&our_model],
            &light,
            &model_normalization_matrix,
            &light_framebuffer,
        )
    };
    let quad = unsafe { Quad::new() };
    let camera_framebuffer = unsafe { Framebuffer::new() };

    let ortho = cgmath::ortho(-1.0, 1.0, -1.0, 1.0, 0.0001, 10_000.0);
    let projection: Matrix4<f32> = perspective(
        Deg(camera.zoom),
        CONFIG.viewport_width as f32 / CONFIG.viewport_height as f32,
        0.0001,
        10000.0,
    );
    let (eye_view_map, eye_view_map_view, eye_view_map_normals, eye_view_map_colors) = unsafe {
        static_eye.take_photo(
            &[&our_model],
            &projection,
            &model_normalization_matrix,
            &camera_framebuffer,
            None,
        )
    };

    let mut current_voxel_fragment_count: u32 = 0;
    let mut current_octree_level: u32 = 0;
    let mut show_model = false;
    let mut show_voxel_fragment_list = false;
    let mut show_octree = false;

    let mut node_filter_text = String::new();
    let mut should_show_final_image = false;

    let mut should_move_light = false;

    let mut cone_angle = 0.26;
    let mut show_indirect_light = false;

    let mut debug_cone_transform = Transform::default();
    debug_cone_transform.position.x = 0.5;
    debug_cone_transform.position.y = 0.5;
    debug_cone_transform.position.z = 0.43;
    let mut debug_cone_direction = vec3(0.0, 1.0, 1.0).normalize();

    let mut previous_values: Vec<u32> = Vec::new();
    let (nodes_queried_texture, nodes_queried_texture_buffer) =
        unsafe { helpers::generate_texture_buffer(2000, gl::R32UI, 69u32) };

    let mut should_show_neighbors = false;
    let mut bricks_to_show = BricksToShow::default();

    let render_voxel_fragments_shader = RenderVoxelFragmentsShader::init(
        voxel_positions.0,
        voxel_colors.0,
        number_of_voxel_fragments,
    );
    let render_border_voxel_fragments_shader = RenderVoxelFragmentsShader::init(
        octree.border_data.voxel_data.voxel_positions.0,
        octree.border_data.voxel_data.voxel_colors.0,
        octree.border_data.voxel_data.number_of_voxel_fragments,
    );
    let render_depth_buffer_shader = Shader::new_single("assets/shaders/renderDepthQuad.glsl");

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

        let (camera_view_map_positions, _, camera_view_map_normals, camera_view_map_colors) = unsafe {
            camera.transform.take_photo(
                &[&our_model],
                &projection,
                &model_normalization_matrix,
                &camera_framebuffer,
                None,
            )
        };

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
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
                common::handle_update_octree_level(&event, &mut current_octree_level);
                common::handle_show_model(&event, &mut show_model);
                common::handle_show_voxel_fragment_list(&event, &mut show_voxel_fragment_list);
                common::handle_show_octree(&event, &mut show_octree);
                common::handle_show_final_image(&event, &mut should_show_final_image);
                common::handle_light_movement(&event, &mut should_move_light);
                common::handle_show_indirect_light(&event, &mut show_indirect_light);
                common::handle_cone_angle(&event, &mut cone_angle);
            }
            menu.handle_event(event);
        }

        menu.begin_frame(current_frame);

        // egui render
        if menu.is_showing() {
            menu.show_main_window();
            if menu.is_showing_node_positions_window() {
                menu.create_node_positions_window(
                    &debug_nodes,
                    &mut selected_debug_nodes,
                    "Node positions",
                    &mut node_filter_text,
                    &mut should_show_neighbors,
                    &mut bricks_to_show,
                    &mut selected_debug_nodes_updated,
                );
            }
            if menu.is_showing_diagnostics_window() {
                menu.create_diagnostics_window(fps);
            }
            if menu.is_showing_photons_window() {
                menu.create_photons_window(&photons);
            }
            if menu.is_showing_children_window() {
                menu.create_children_window(&children);
            }
        }

        // This is for debugging
        if selected_debug_nodes_updated {
            selected_debug_nodes_updated = false;
            let last_debug_node = selected_debug_nodes.last();
            if let Some(last_debug_node) = last_debug_node {
                unsafe {
                    octree.run_get_photons_shader(last_debug_node.index());
                    photons = helpers::get_values_from_texture_buffer(
                        octree.textures.photons_buffer.1,
                        27, // Voxels in a brick
                        42_u32,
                    );
                    octree.run_get_children_shader(last_debug_node.index());
                    children = helpers::get_values_from_texture_buffer(
                        octree.textures.children_buffer.1,
                        8, // Children in a node
                        42_u32,
                    );
                    octree.run_colors_quad_shader(last_debug_node.index());
                };
            }
        }

        // Input
        if !menu.is_showing() {
            let transform = if should_move_light {
                // unsafe {
                //     octree.clear_light();
                // }
                // light_maps = unsafe {
                //     // TODO: This takes too long, optimize
                //     octree.inject_light(
                //         &[&our_model],
                //         &light,
                //         &model_normalization_matrix,
                //         &light_framebuffer,
                //     )
                // };
                // &mut light.transform
                &mut debug_cone_transform
            } else {
                &mut camera.transform
            };
            common::process_movement_input(&mut window, delta_time as f32, transform);
        }

        // Render
        unsafe {
            // let projection: Matrix4<f32> = perspective(
            //     Deg(camera.zoom),
            //     CONFIG.viewport_width as f32 / CONFIG.viewport_height as f32,
            //     0.0001,
            //     10000.0,
            // );
            let projection = projection;
            let view = camera.transform.get_view_matrix();
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
            model = model * Matrix4::from_scale(1.);

            if show_voxel_fragment_list {
                if should_move_light {
                    render_voxel_fragments_shader.run(&projection, &view, &model);
                } else {
                    render_border_voxel_fragments_shader.run(&projection, &view, &model);
                }
            }

            if show_octree {
                octree.render(&model, &view, &projection, current_octree_level);
            }

            octree.set_node_indices(
                &selected_debug_nodes
                    .iter()
                    .map(|node| node.index())
                    .collect(),
            );
            octree.run_node_positions_shader(&projection, &view, &model);
            octree.set_bricks_to_show(bricks_to_show);
            // octree.run_eye_ray_shader(
            //     &projection,
            //     &view,
            //     &static_eye,
            //     eye_view_map,
            //     eye_view_map_normals,
            // );

            if should_show_neighbors {
                octree.run_node_neighbors_shader(&projection, &view, &model);
            }

            if bricks_to_show.at_least_one() {
                octree.run_node_bricks_shader(&projection, &view, &model);
            }

            if show_model {
                // Render model normally
                render_model_shader.use_program();
                render_model_shader.set_mat4(c_str!("projection"), &projection);
                render_model_shader.set_mat4(c_str!("view"), &view);
                render_model_shader.set_mat4(c_str!("model"), &model_normalization_matrix);
                our_model.draw(&render_model_shader);
            }

            {
                // Render illumination image to quad
                voxel_cone_tracing_shader.use_program();

                voxel_cone_tracing_shader
                    .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
                voxel_cone_tracing_shader
                    .set_uint(c_str!("maxOctreeLevel"), CONFIG.octree_levels - 1);
                voxel_cone_tracing_shader.set_float(
                    c_str!("photonPower"),
                    light.intensity / (CONFIG.viewport_width * CONFIG.viewport_height) as f32,
                );
                voxel_cone_tracing_shader
                    .set_bool(c_str!("showIndirectLight"), show_indirect_light);
                voxel_cone_tracing_shader.set_vec3(
                    c_str!("eyePosition"),
                    camera.transform.position.x,
                    camera.transform.position.y,
                    camera.transform.position.z,
                );
                let light_direction = vec3(
                    light.transform.position.x,
                    light.transform.position.y,
                    light.transform.position.z,
                );
                voxel_cone_tracing_shader.set_vec3(
                    c_str!("lightDirection"),
                    light_direction.x,
                    light_direction.y,
                    light_direction.z,
                );
                voxel_cone_tracing_shader.set_float(c_str!("shininess"), 30.0);
                voxel_cone_tracing_shader.set_mat4(
                    c_str!("lightViewMatrix"),
                    &light.transform.get_view_matrix(),
                );
                voxel_cone_tracing_shader.set_mat4(
                    c_str!("lightProjectionMatrix"),
                    &light.get_projection_matrix(),
                );
                voxel_cone_tracing_shader.set_float(c_str!("coneAngle"), cone_angle);
                helpers::bind_image_texture(
                    0,
                    octree.textures.node_pool.0,
                    gl::READ_ONLY,
                    gl::R32UI,
                );

                let brick_pool_textures = vec![
                    (
                        c_str!("brickPoolColors"),
                        octree.textures.brick_pool_colors,
                        gl::LINEAR as i32,
                    ),
                    (
                        c_str!("brickPoolPhotons"),
                        octree.textures.brick_pool_photons,
                        gl::NEAREST as i32,
                    ),
                ];

                let mut texture_counter = 0;

                for &(texture_name, texture, sample_interpolation) in brick_pool_textures.iter() {
                    gl::ActiveTexture(gl::TEXTURE0 + texture_counter);
                    gl::BindTexture(gl::TEXTURE_3D, texture);
                    voxel_cone_tracing_shader.set_int(texture_name, texture_counter as i32);
                    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, sample_interpolation);
                    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, sample_interpolation);
                    texture_counter += 1;
                }

                // let g_buffer_textures = vec![
                //     (c_str!("gBufferColors"), eye_view_map_colors),
                //     (c_str!("gBufferPositions"), eye_view_map),
                //     (c_str!("gBufferNormals"), eye_view_map_normals),
                // ];
                let g_buffer_textures = vec![
                    (c_str!("gBufferColors"), camera_view_map_colors),
                    (c_str!("gBufferPositions"), camera_view_map_positions),
                    (c_str!("gBufferNormals"), camera_view_map_normals),
                ];

                for &(texture_name, texture) in g_buffer_textures.iter() {
                    gl::ActiveTexture(gl::TEXTURE0 + texture_counter);
                    gl::BindTexture(gl::TEXTURE_2D, texture);
                    voxel_cone_tracing_shader.set_int(texture_name, texture_counter as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                    texture_counter += 1;
                }

                gl::ActiveTexture(gl::TEXTURE0 + texture_counter);
                gl::BindTexture(gl::TEXTURE_2D, light_maps.2);
                voxel_cone_tracing_shader.set_int(c_str!("shadowMap"), texture_counter as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

                let quad_vao = quad.get_vao();

                if should_show_final_image {
                    gl::BindVertexArray(quad_vao);
                    gl::DrawElements(
                        gl::TRIANGLES,
                        quad.get_num_indices() as i32,
                        gl::UNSIGNED_INT,
                        std::ptr::null(),
                    );
                    gl::BindVertexArray(0);
                }

                // let (debug, buffer) = helpers::generate_texture_buffer(100, gl::R32F, 69f32);
                // helpers::bind_image_texture(4, debug, gl::WRITE_ONLY, gl::R32F);
                // our_model.draw(&voxel_cone_tracing_shader);
                // let debug_values = helpers::get_values_from_texture_buffer(buffer, 100, 420f32);
                // dbg!(&debug_values[..20]);

                // Show normals
                // render_normals_shader.use_program();
                // render_normals_shader.set_mat4(c_str!("projection"), &projection);
                // render_normals_shader.set_mat4(c_str!("view"), &view);
                // render_normals_shader.set_mat4(c_str!("model"), &model_normalization_matrix);
                // our_model.draw(&render_normals_shader);
            }

            {
                let mut vao = 0;
                gl::GenVertexArrays(1, &mut vao);
                gl::BindVertexArray(vao);

                helpers::bind_image_texture(0, nodes_queried_texture, gl::WRITE_ONLY, gl::R32UI);
                helpers::bind_image_texture(
                    1,
                    octree.textures.node_pool.0,
                    gl::READ_ONLY,
                    gl::R32UI,
                );

                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_3D, octree.textures.brick_pool_colors);
                debug_cone_shader.set_int(c_str!("brickPoolColors"), 0 as i32);
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                debug_cone_shader.use_program();
                debug_cone_shader.set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
                debug_cone_shader.set_uint(c_str!("maxOctreeLevel"), CONFIG.octree_levels - 1);
                debug_cone_shader.set_mat4(c_str!("projection"), &projection);
                debug_cone_shader.set_mat4(c_str!("view"), &view);
                debug_cone_shader.set_vec3(
                    c_str!("position"),
                    debug_cone_transform.position.x,
                    debug_cone_transform.position.y,
                    debug_cone_transform.position.z,
                );
                debug_cone_shader.set_vec3(
                    c_str!("axis"),
                    debug_cone_direction.x,
                    debug_cone_direction.y,
                    debug_cone_direction.z,
                );
                debug_cone_shader.set_float(c_str!("coneAngle"), cone_angle);

                gl::DrawArrays(gl::POINTS, 0, 1);
                //let values = helpers::get_values_from_texture_buffer(nodes_queried_texture_buffer, 100, 42u32);

                //if (previous_values != values) {
                //let values_length = values[0] as usize;
                //dbg!(&values[1..values_length]);
                //selected_debug_nodes = (&values[1..values_length]).iter().map(|&index| DebugNode::new(index, "picked by cone".to_string())).collect();
                //previous_values = values;
                //}

                gl::BindVertexArray(0);
            }

            static_eye.draw_gizmo(&projection, &view);
            light.draw_gizmo(&projection, &view);
            quad.render(light_maps.1);

            // let quad_vao = quad.get_vao();
            // render_depth_buffer_shader.use_program();

            // gl::ActiveTexture(gl::TEXTURE0);
            // gl::BindTexture(gl::TEXTURE_2D, shadow_map);
            // render_depth_buffer_shader.set_int(c_str!("depthMap"), 0);
            // gl::BindVertexArray(quad_vao);
            // gl::DrawElements(
            //     gl::TRIANGLES,
            //     quad.get_num_indices() as i32,
            //     gl::UNSIGNED_INT,
            //     std::ptr::null(),
            // );
            // gl::BindVertexArray(0);
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
