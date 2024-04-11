//! The entrypoint to the VCT application

use core::menu::submenus::{PickerMenu, SystemsMenu};
use core::simple_texture::{
    ConeTracer as SimpleConeTracer, DebugConeTracer as SimpleDebugConeTracer,
    Visualizer as VoxelVisualizer, Voxelizer,
};
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::Instant;
use std::{collections::HashMap, sync::mpsc::Receiver};

extern crate c_str_macro;

use c_str_macro::c_str;
extern crate gl;
use cgmath::{point3, vec2, vec3, Euler, Matrix4};
use core::{
    cone_tracing::{ConeTracer, DebugCone},
    config::Config as CoreConfig,
    menu::Preset,
    octree::{BrickAttribute, BricksToShow, Octree, OctreeDataType},
    voxelization,
    voxelization::visualize::RenderVoxelFragmentsShader,
};
use engine::ui::Ui;
use engine::ui::{egui, glfw};
use engine::{
    prelude::*,
    systems::{GeometryBuffers, MoveCamera, RenderObjects},
    ui::glfw::{Glfw, WindowEvent},
};
use structopt::StructOpt;

mod cli_arguments;
use cli_arguments::Options;
mod preset;
mod scene;

fn main() {
    simple_logger::init().unwrap();
    let options = Options::from_args();
    // NOTE: This is true if the binary was compiled in debug mode
    let debug = cfg!(debug_assertions);
    let file = File::open(&format!("{}.ron", &options.config)).expect("Missing config file!");
    let config: CoreConfig = ron::de::from_reader(file).expect("Config file malformed!");
    log::info!("Configuration used: {:#?}", config);

    let (viewport_width, viewport_height) = config.viewport_dimensions();
    let (glfw, events) = unsafe {
        common::setup_glfw(
            viewport_width,
            viewport_height,
            debug,
            options.screenshot
                || options.record_octree_build_time
                || options.seconds_for_fps.is_some(), // Don't run headless
        )
    };

    let scene = scene::load_scene(&options.scene);
    let preset = preset::load_preset(&options.preset);

    let parameters = ApplicationParameters {
        config,
        scene,
        preset,
        events,
        options,
    };

    run_application(parameters, glfw);
}

struct ApplicationParameters {
    config: CoreConfig,
    scene: Scene,
    preset: Preset,
    events: Receiver<(f64, WindowEvent)>,
    options: Options,
}

fn run_application(parameters: ApplicationParameters, mut glfw: Glfw) {
    // Load configuration file and set it up in core
    unsafe {
        CoreConfig::initialize(parameters.config);
    }
    let config = CoreConfig::instance();
    let mut scene = parameters.scene;
    let preset = parameters.preset;

    let (viewport_width, viewport_height) = config.viewport_dimensions();

    // Camera setup
    let mut camera = preset.camera.clone();

    // TODO: Remove once we handle cameras properly.
    scene.cameras.push(RefCell::new(camera));

    unsafe {
        common::log_device_information();
    };

    // let mut menu = Menu::new(preset.clone());

    let mut cone_tracer = ConeTracer::init();
    // let mut cone_parameters = HashMap::new();
    let mut debug_cone = unsafe { DebugCone::new() };

    // let (voxel_positions, number_of_voxel_fragments, voxel_colors, voxel_normals) =
    //     unsafe { voxelization::build_voxel_fragment_list(&mut objects[..], &scene_aabb) };
    // log::info!("Number of voxel fragments: {}", number_of_voxel_fragments);

    // let _instant_before_octree = Instant::now();
    // let mut octree = unsafe {
    //     Octree::new(
    //         voxel_positions.clone(),
    //         number_of_voxel_fragments,
    //         voxel_colors,
    //         voxel_normals,
    //     )
    // };
    // if parameters.options.record_octree_build_time {
    //     let octree_build_time = _instant_before_octree.elapsed().as_millis().to_string();
    //     let mut in_bytes = octree_build_time.as_bytes().to_vec();
    //     let newline = b'\n';
    //     in_bytes.push(newline);
    //     let folder = parameters.options.get_name();
    //     let file_name = format!("{folder}/octree_build_time.txt");
    //     OpenOptions::new()
    //         .append(true)
    //         .create(true)
    //         .open(&file_name)
    //         .expect("Couldn't open record octree build time file")
    //         .write(&in_bytes[..])
    //         .expect("Couldn't append to record octree build time file");
    // }

    // let node_positions = unsafe {
    //     helpers::get_values_from_texture_buffer(
    //         octree.textures.node_positions.1,
    //         octree.number_of_nodes(),
    //         0_u32,
    //     )
    // };

    // let debug_nodes: Vec<DebugNode> = node_positions
    //     .iter()
    //     .enumerate()
    //     .map(|(index, &node_position)| {
    //         let position = helpers::r32ui_to_rgb10_a2ui(node_position);
    //         let text = format!("({}, {}, {})", position.0, position.1, position.2);
    //         DebugNode::new(index as u32, text)
    //     })
    //     .collect();
    // let mut selected_debug_nodes: Vec<DebugNode> = Vec::new();
    // let mut selected_debug_nodes_updated = false;
    let mut color_direction = vec3(1.0, 0.0, 0.0);
    let mut brick_attribute = BrickAttribute::None;
    let mut brick_padding = 0.0;
    let mut should_show_normals = false;

    let mut photons: Vec<u32> = Vec::new();
    let mut children: Vec<u32> = Vec::new();

    // let mut light_maps = unsafe { octree.inject_light(&mut objects[..], &light, &scene_aabb) };
    let quad = unsafe { Quad::new() };

    let mut current_voxel_fragment_count: u32 = 0;
    let mut current_octree_level: u32 = 0;
    let mut show_model = false;
    let mut show_voxel_fragment_list = false;
    let mut show_octree = false;
    let mut octree_nodes_to_visualize = OctreeDataType::Geometry;
    let mut geometry_buffer_coordinates = vec2(0.0, 0.0);

    let mut node_filter_text = String::new();
    let mut should_show_final_image_quad = false;

    let mut should_move_light = false;

    let mut should_show_neighbors = false;
    let mut bricks_to_show = BricksToShow::default();

    let mut should_show_debug_cone = false;
    let mut should_move_debug_cone = false;

    // let render_voxel_fragments_shader = RenderVoxelFragmentsShader::init(
    //     voxel_positions.texture(),
    //     voxel_colors.0,
    //     number_of_voxel_fragments,
    // );
    // let render_border_voxel_fragments_shader = RenderVoxelFragmentsShader::init(
    //     octree.border_data.voxel_data.voxel_positions.texture(),
    //     octree.border_data.voxel_data.voxel_colors.0,
    //     octree.border_data.voxel_data.number_of_voxel_fragments,
    // );
    let render_depth_buffer_shader = compile_shaders!("assets/shaders/renderDepthQuad.glsl");

    // let photon_power = scene.light.intensity() / (viewport_width * viewport_height) as f32;

    // let mut fps_values = Vec::new();

    let mut mipmap_level = 0;

    #[derive(SubMenu, Showable)]
    enum AggregatedSubMenus {
        Systems(SystemsMenu),
        Picker(PickerMenu),
    }

    #[derive(System, Pausable)]
    enum AggregatedSystem {
        RenderObjects(RenderObjects),
        GeometryBuffers(GeometryBuffers),
        Voxelizer(Voxelizer),
        VoxelVisualizer(VoxelVisualizer),
        SimpleConeTracer(SimpleConeTracer),
        SimpleDebugConeTracer(SimpleDebugConeTracer),
        MoveCamera(MoveCamera),
    }

    let mut render_loop = RenderLoop::<AggregatedSystem, AggregatedSubMenus>::new(
        glfw,
        parameters.events,
        (viewport_width, viewport_height),
    );
    render_loop.set_scene(scene);

    // Render loop.
    unsafe {
        // Register systems.
        render_loop.register_system(AggregatedSystem::RenderObjects(RenderObjects::new()));
        render_loop.register_system(AggregatedSystem::GeometryBuffers(GeometryBuffers::new()));
        render_loop.register_system(AggregatedSystem::Voxelizer(Voxelizer::new()));
        render_loop.register_system(AggregatedSystem::VoxelVisualizer(VoxelVisualizer::new()));
        render_loop.register_system(AggregatedSystem::SimpleConeTracer(SimpleConeTracer::new()));
        render_loop.register_system(AggregatedSystem::SimpleDebugConeTracer(
            SimpleDebugConeTracer::new(),
        ));
        render_loop.register_system(AggregatedSystem::MoveCamera(MoveCamera::new()));

        render_loop.register_submenu("Systems", AggregatedSubMenus::Systems(SystemsMenu::new()));
        render_loop.register_submenu("Picker", AggregatedSubMenus::Picker(PickerMenu::new()));

        render_loop.run();
    };

    //     if elapsed_time > 1.0 {
    //         fps = frame_count as f64 / elapsed_time;
    //         if let Some(seconds_for_fps) = parameters.options.seconds_for_fps {
    //             // Push fps value to vec
    //             fps_values.push(fps);
    //             if fps_values.len() > seconds_for_fps as usize {
    //                 let average: f64 = fps_values.iter().sum::<f64>() / fps_values.len() as f64;
    //                 let folder = parameters.options.get_name();
    //                 let file_name = format!("{folder}/fps.txt");
    //                 OpenOptions::new()
    //                     .write(true)
    //                     .truncate(true)
    //                     .create(true)
    //                     .open(&file_name)
    //                     .expect("Couldn't open file")
    //                     .write_all(average.to_string().as_bytes())
    //                     .expect("Couldn't write to file");
    //             }
    //         }
    //         frame_count = 0;
    //         starting_time = current_frame;
    //     }

    //     // egui render
    //     // if ui.is_showing() {

    //     //     // All nodes
    //     //     show_octree = outputs.0.should_render_octree;
    //     //     current_octree_level = outputs.0.current_octree_level;
    //     //     octree_nodes_to_visualize = outputs.0.octree_nodes_to_visualize.clone();

    //     //     // Node search
    //     //     selected_debug_nodes = selected_debug_nodes
    //     //         .into_iter()
    //     //         .chain(outputs.1.selected_items.clone())
    //     //         .collect();
    //     //     node_filter_text = outputs.1.filter_text.clone();
    //     //     should_show_neighbors = outputs.1.should_show_neighbors;
    //     //     selected_debug_nodes_updated = outputs.1.selected_items_updated;

    //     //     // Bricks
    //     //     bricks_to_show = outputs.2.bricks_to_show;
    //     //     brick_attribute = outputs.2.brick_attribute;
    //     //     should_show_normals = outputs.2.should_show_brick_normals;
    //     //     color_direction = outputs.2.color_direction;
    //     //     brick_padding = outputs.2.brick_padding;

    //     //     // Images
    //     //     cone_tracer.toggles = outputs.5.toggles.clone();

    //     //     // Camera
    //     //     active_camera.orthographic = outputs.8.orthographic;

    //     //     // Cone tracing
    //     //     cone_parameters.insert(
    //     //         "shadowConeParameters",
    //     //         outputs.9.shadow_cone_parameters.clone(),
    //     //     );
    //     //     cone_parameters.insert(
    //     //         "ambientOcclusionConeParameters",
    //     //         outputs.9.ambient_occlusion_cone_parameters.clone(),
    //     //     );
    //     //     cone_parameters.insert(
    //     //         "diffuseConeParameters",
    //     //         outputs.9.diffuse_cone_parameters.clone(),
    //     //     );
    //     //     cone_parameters.insert(
    //     //         "specularConeParameters",
    //     //         outputs.9.specular_cone_parameters.clone(),
    //     //     );
    //     //     // This one doesn't come from `get_data()` but is still relevant to `debug_cone`
    //     //     geometry_buffer_coordinates = menu.get_quad_coordinates();

    //     //     should_show_debug_cone = outputs.10.show_debug_cone;
    //     //     should_move_debug_cone = outputs.10.move_debug_cone;
    //     //     // TODO: there is quite a bit of cloning here
    //     //     debug_cone.parameters = outputs.10.cone_parameters.clone();
    //     //     debug_cone.point_to_light = outputs.10.point_to_light;
    //     // }

    //     // This is for debugging
    //     // if selected_debug_nodes_updated {
    //     //     selected_debug_nodes_updated = false;
    //     //     let last_debug_node = selected_debug_nodes.last();
    //     //     if let Some(last_debug_node) = last_debug_node {
    //     //         unsafe {
    //     //             octree.run_get_photons_shader(last_debug_node.index());
    //     //             photons = helpers::get_values_from_texture_buffer(
    //     //                 octree.textures.photons_buffer.1,
    //     //                 27, // Voxels in a brick
    //     //                 42_u32,
    //     //             );
    //     //             octree.run_get_children_shader(last_debug_node.index());
    //     //             children = helpers::get_values_from_texture_buffer(
    //     //                 octree.textures.children_buffer.1,
    //     //                 8, // Children in a node
    //     //                 42_u32,
    //     //             );
    //     //             // octree.run_colors_quad_shader(last_debug_node.index());
    //     //         };
    //     //     }
    //     // }

    //     // Render
    //     unsafe {
    //         // if show_octree {
    //         //     octree.render(
    //         //         &model,
    //         //         &view,
    //         //         &projection,
    //         //         current_octree_level,
    //         //         color_direction,
    //         //         should_show_normals,
    //         //         brick_attribute,
    //         //         brick_padding,
    //         //         node_data_to_visualize,
    //         //     );
    //         // }

    //         // octree.set_node_indices(
    //         //     &selected_debug_nodes
    //         //         .iter()
    //         //         .map(|node| node.index())
    //         //         .collect(),
    //         // );
    //         // octree.run_node_positions_shader(&projection, &view, &model);
    //         // octree.set_bricks_to_show(bricks_to_show);

    //         // if should_show_neighbors {
    //         //     octree.run_node_neighbors_shader(&projection, &view, &model);
    //         // }

    //         // if bricks_to_show.at_least_one() {
    //         //     octree.run_node_bricks_shader(
    //         //         &projection,
    //         //         &view,
    //         //         &model,
    //         //         color_direction,
    //         //         brick_attribute,
    //         //         brick_padding,
    //         //     );
    //         // }

    //         // cone_tracer.run(
    //         //     &light,
    //         //     &octree.textures,
    //         //     &geometry_buffers,
    //         //     light_maps,
    //         //     &quad,
    //         //     &active_camera,
    //         //     &cone_parameters,
    //         //     if parameters.options.screenshot {
    //         //         Some(parameters.options.get_name())
    //         //     } else {
    //         //         None
    //         //     },
    //         // );

    //         // if should_show_debug_cone {
    //         //     debug_cone.run(
    //         //         &octree.textures,
    //         //         &projection,
    //         //         &view,
    //         //         &mut selected_debug_nodes,
    //         //         &geometry_buffers,
    //         //         &geometry_buffer_coordinates,
    //         //         &light,
    //         //     );
    //         // }
    //         light.draw_gizmo(&projection, &view);
    //         // quad.render(light_maps.1);

    //         // let quad_vao = quad.get_vao();
    //         // render_depth_buffer_shader.use_program();

    //         // gl::ActiveTexture(gl::TEXTURE0);
    //         // gl::BindTexture(gl::TEXTURE_2D, shadow_map);
    //         // render_depth_buffer_shader.set_int(c_str!("depthMap"), 0);
    //         // gl::BindVertexArray(quad_vao);
    //         // gl::DrawElements(
    //         //     gl::TRIANGLES,
    //         //     quad.get_num_indices() as i32,
    //         //     gl::UNSIGNED_INT,
    //         //     std::ptr::null(),
    //         // );
    //         // gl::BindVertexArray(0);
    //     }

    //     // current_voxel_fragment_count =
    //     //     (current_voxel_fragment_count + 10000).min(number_of_voxel_fragments);

    //     // Swap buffers and poll I/O events was here.

    //     if parameters.options.screenshot
    //         || parameters.options.record_octree_build_time
    //         || parameters
    //             .options
    //             .seconds_for_fps
    //             .is_some_and(|seconds| fps_values.len() as f64 > seconds as f64)
    //     {
    //         break;
    //     }
}
