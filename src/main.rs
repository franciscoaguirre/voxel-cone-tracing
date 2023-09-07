extern crate c_str_macro;

use c_str_macro::c_str;
use egui_glfw_gl::glfw::{self, Context};
extern crate gl;
use crate::{
    cone_tracing::DebugCone,
    menu::{
        submenus::{
            ChildrenMenuInput, DiagnosticsMenuInput, NodeSearchMenuInput, PhotonsMenuInput,
            SavePresetMenuInput,
        },
        MenuInternals, SubMenus,
    },
    octree::OctreeDataType,
    preset::PRESET,
    rendering::shader::compile_shaders,
};
use cgmath::{point3, vec3, Deg, Matrix4};
use log::info;

use rendering::quad::Quad;
use structopt::StructOpt;

mod cli_arguments;
mod cone_tracing;
mod config;
mod constants;
mod helpers;
mod menu;
mod octree;
mod preset;
mod rendering;
mod scene;
mod types;
mod voxelization;

use config::CONFIG;
use menu::Menu;
use rendering::{camera::Camera, common, gizmo::RenderGizmo, shader::Shader};
use scene::SCENE;
use voxelization::visualize::RenderVoxelFragmentsShader;

use octree::{BricksToShow, Octree};

use crate::{
    cone_tracing::ConeTracer,
    menu::DebugNode,
    octree::BrickAttribute,
    rendering::{framebuffer::Framebuffer, transform::Transform},
};

fn main() {
    simple_logger::init().unwrap();

    // NOTE: This is true if the binary was compiled in debug mode
    let debug = cfg!(debug_assertions);

    // Timing setup
    let mut delta_time: f64;
    let mut last_frame: f64 = 0.0;

    let (mut glfw, mut window, events) = unsafe { common::setup_glfw(debug) };

    // Camera setup
    let mut camera = PRESET.camera.clone();
    let mut first_mouse = true;
    let mut last_x: f32 = CONFIG.viewport_width as f32 / 2.0;
    let mut last_y: f32 = CONFIG.viewport_height as f32 / 2.0;

    // Static eye
    let mut static_eye = Transform::default();
    static_eye.position = point3(0.0, 0.0, -2.0);

    // FPS variables
    let mut frame_count = 0;
    let mut starting_time: f64 = glfw.get_time();
    let mut elapsed_time: f64;
    let mut fps: f64 = 0.0;

    unsafe {
        common::log_device_information();
    };

    let mut menu = Menu::new(&mut window);

    let render_model_shader = compile_shaders!(
        "assets/shaders/model/modelLoading.vert.glsl",
        "assets/shaders/model/modelLoading.frag.glsl",
        "assets/shaders/model/modelLoading.geom.glsl",
    );
    let render_normals_shader = compile_shaders!(
        "assets/shaders/model/renderNormals.vert.glsl",
        "assets/shaders/model/renderNormals.frag.glsl",
        "assets/shaders/model/renderNormals.geom.glsl",
    );
    let mut cone_tracer = ConeTracer::init();
    let mut debug_cone = unsafe { DebugCone::new() };
    let our_model = unsafe { helpers::load_model() };

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
    let mut color_direction = vec3(1.0, 0.0, 0.0);
    let mut brick_attribute = BrickAttribute::None;
    let mut brick_padding = 0.0;
    let mut should_show_normals = false;

    let mut photons: Vec<u32> = Vec::new();
    let mut children: Vec<u32> = Vec::new();

    let mut light = SCENE.light.clone();

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

    let mut current_voxel_fragment_count: u32 = 0;
    let mut current_octree_level: u32 = 0;
    let mut show_model = false;
    let mut show_voxel_fragment_list = false;
    let mut show_octree = false;
    let mut octree_nodes_to_visualize = OctreeDataType::Geometry;

    let mut node_filter_text = String::new();
    let mut should_show_final_image_quad = false;

    let mut should_move_light = false;

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
    let render_depth_buffer_shader = compile_shaders!("assets/shaders/renderDepthQuad.glsl");

    let photon_power = light.intensity / (CONFIG.viewport_width * CONFIG.viewport_height) as f32;

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

        let geometry_buffers = unsafe {
            camera.transform.take_photo(
                &[&our_model],
                &camera.get_projection_matrix(),
                &model_normalization_matrix,
                &camera_framebuffer,
                None,
            )
        };

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::Disable(gl::BLEND);
            // gl::Enable(gl::BLEND);
            // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        for (_, event) in glfw::flush_messages(&events) {
            // Events
            if let glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) = event {
                menu.toggle_showing(&mut window, &mut last_x, &mut last_y);
            };
            if !menu.is_showing() {
                common::process_events(
                    &event,
                    &mut first_mouse,
                    &mut last_x,
                    &mut last_y,
                    &mut camera,
                );
                common::handle_show_model(&event, &mut show_model);
                common::handle_show_voxel_fragment_list(&event, &mut show_voxel_fragment_list);
                common::handle_light_movement(&event, &mut should_move_light);
                common::handle_cone_angle(&event, &mut debug_cone.cone_angle);
            }
            menu.handle_event(event);
        }

        menu.begin_frame(current_frame);

        dbg!(&camera.orthographic);

        // egui render
        if menu.is_showing() {
            menu.show_main_window();
            menu.render((
                (),
                NodeSearchMenuInput::new(debug_nodes.clone()),
                (),
                ChildrenMenuInput::new(children.clone()),
                DiagnosticsMenuInput::new(fps),
                (),
                PhotonsMenuInput::new(photons.clone()),
                SavePresetMenuInput::new(&camera, menu.sub_menus.clone()), // TODO: Remove clone
                (),
            ));
            let outputs = menu.get_data();

            // All nodes
            show_octree = outputs.0.should_render_octree;
            current_octree_level = outputs.0.current_octree_level;
            octree_nodes_to_visualize = outputs.0.octree_nodes_to_visualize.clone();

            // Node search
            selected_debug_nodes = outputs.1.selected_items.clone();
            node_filter_text = outputs.1.filter_text.clone();
            should_show_neighbors = outputs.1.should_show_neighbors;
            selected_debug_nodes_updated = outputs.1.selected_items_updated;

            // Bricks
            bricks_to_show = outputs.2.bricks_to_show;
            brick_attribute = outputs.2.brick_attribute;
            should_show_normals = outputs.2.should_show_brick_normals;
            color_direction = outputs.2.color_direction;
            brick_padding = outputs.2.brick_padding;

            // Images
            cone_tracer.toggles = outputs.5.toggles.clone();

            // Camera
            camera.orthographic = outputs.8.orthographic;
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
                    // octree.run_colors_quad_shader(last_debug_node.index());
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
                &mut debug_cone.transform
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
            let projection = camera.get_projection_matrix();
            let view = camera.transform.get_view_matrix();
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
            model = model * Matrix4::from_scale(1.);

            if show_voxel_fragment_list {
                if should_move_light {
                    render_border_voxel_fragments_shader.run(&projection, &view, &model);
                } else {
                    render_voxel_fragments_shader.run(&projection, &view, &model);
                }
            }

            let node_data_to_visualize = match octree_nodes_to_visualize {
                OctreeDataType::Geometry => &octree.geometry_data.node_data,
                OctreeDataType::Border => &octree.border_data.node_data,
            };

            if show_octree {
                octree.render(
                    &model,
                    &view,
                    &projection,
                    current_octree_level,
                    color_direction,
                    should_show_normals,
                    brick_attribute,
                    brick_padding,
                    node_data_to_visualize,
                );
            }

            octree.set_node_indices(
                &selected_debug_nodes
                    .iter()
                    .map(|node| node.index())
                    .collect(),
            );
            octree.run_node_positions_shader(&projection, &view, &model);
            octree.set_bricks_to_show(bricks_to_show);

            if should_show_neighbors {
                octree.run_node_neighbors_shader(&projection, &view, &model);
            }

            if bricks_to_show.at_least_one() {
                octree.run_node_bricks_shader(
                    &projection,
                    &view,
                    &model,
                    color_direction,
                    brick_attribute,
                    brick_padding,
                );
            }

            if show_model {
                // Render model normally
                render_model_shader.use_program();
                render_model_shader.set_mat4(c_str!("projection"), &projection);
                render_model_shader.set_mat4(c_str!("view"), &view);
                render_model_shader.set_mat4(c_str!("model"), &model_normalization_matrix);
                our_model.draw(&render_model_shader);
            }

            cone_tracer.run(
                &light,
                debug_cone.cone_angle,
                &octree.textures,
                &geometry_buffers,
                light_maps,
                &quad,
                &camera,
            );

            // TODO: Add toggle to menu
            // debug_cone.run(
            //     &octree.textures,
            //     &projection,
            //     &view,
            //     &mut selected_debug_nodes,
            // );
            static_eye.draw_gizmo(&projection, &view);
            light.draw_gizmo(&projection, &view);
            // quad.render(light_maps.1);

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
