//! The entrypoint to the VCT application

use core::simple_texture::{
    ConeTracer as SimpleConeTracer, ConeTracerRunInputs, GpuKernel, Visualizer,
    VisualizerRunInputs, Voxelizer, VoxelizerRunInputs,
};
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
    menu::{
        submenus::{
            ChildrenMenuInput, DiagnosticsMenuInput, NodeSearchMenuInput, PhotonsMenuInput,
            SavePresetMenuInput,
        },
        DebugNode, Menu, Preset,
    },
    octree::{BrickAttribute, BricksToShow, Octree, OctreeDataType},
    voxelization,
    voxelization::visualize::RenderVoxelFragmentsShader,
};
use engine::ui::glfw;
use engine::ui::Ui;
use engine::{
    prelude::*,
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
    let scene = parameters.scene;
    let preset = parameters.preset;

    // Timing setup
    let mut delta_time: f64;
    let mut last_frame: f64 = 0.0;

    let (viewport_width, viewport_height) = config.viewport_dimensions();

    // Camera setup
    let mut camera = preset.camera.clone();
    let mut first_mouse = true;
    let mut last_x: f32 = viewport_width as f32 / 2.0;
    let mut last_y: f32 = viewport_height as f32 / 2.0;

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

    let mut menu = Menu::new(preset.clone());

    let render_model_shader = compile_shaders!(
        "assets/shaders/model/modelLoading.vert.glsl",
        "assets/shaders/model/modelLoading.frag.glsl",
        "assets/shaders/model/modelLoading.geom.glsl",
    );
    let mut cone_tracer = ConeTracer::init();
    // let mut cone_parameters = HashMap::new();
    let mut debug_cone = unsafe { DebugCone::new() };

    // Process scene
    let (mut objects, mut light) = process_scene(scene);

    let mut scene_aabb = Aabb::default();
    for object in objects.iter_mut() {
        let offset = vec3(
            object.transform.position.x,
            object.transform.position.y,
            object.transform.position.z,
        );
        let rotation = Euler {
            x: object.transform.rotation_x(),
            y: object.transform.rotation_y(),
            z: object.transform.rotation_z(),
        };
        scene_aabb.join(&object.model().aabb.rotated(rotation).offsetted(offset));
    }
    let model_normalization_matrix = scene_aabb.normalization_matrix();

    // Here I'm trying to simplify and use the simple 3D texture approach.
    let voxelizer = unsafe { Voxelizer::init(()) };
    let voxels_texture = unsafe {
        voxelizer.run(VoxelizerRunInputs {
            objects: &mut objects[..],
            scene_aabb: &scene_aabb,
            camera: &camera,
            light: &light,
        });
        voxelizer.voxels_texture
    };
    let voxels_visualizer = unsafe { Visualizer::init(()) };
    let mut mipmap_level = 0;
    let simple_cone_tracer = unsafe { SimpleConeTracer::init(()) };

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
    // let cube = unsafe { Cube::new() };
    // let very_simple_shader = compile_shaders!("assets/shaders/model/very_simple.glsl");
    let camera_framebuffer = unsafe { GeometryFramebuffer::new() };

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

    let photon_power = light.intensity() / (viewport_width * viewport_height) as f32;

    // TODO: Theory. See if EGUI breaks the rendering in some way.
    // I remember it did some weird things with opacity, but maybe that was
    // just how we rendered the UI.
    let ui = Ui::instance();

    let mut fps_values = Vec::new();

    // We create a camera from the view of the light.
    let mut light_camera = Camera::default();
    light_camera.transform = light.transform().clone();

    // The active camera is a reference to a camera.
    // All calculations are done with the active camera.
    // It can be switched at runtime. TODO: Not yet.
    let active_camera = &mut camera;

    // Render loop
    while !common::should_close_window() {
        let current_frame = glfw.get_time();

        frame_count += 1;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        elapsed_time = current_frame - starting_time;

        if elapsed_time > 1.0 {
            fps = frame_count as f64 / elapsed_time;
            if let Some(seconds_for_fps) = parameters.options.seconds_for_fps {
                // Push fps value to vec
                fps_values.push(fps);
                if fps_values.len() > seconds_for_fps as usize {
                    let average: f64 = fps_values.iter().sum::<f64>() / fps_values.len() as f64;
                    let folder = parameters.options.get_name();
                    let file_name = format!("{folder}/fps.txt");
                    OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .create(true)
                        .open(&file_name)
                        .expect("Couldn't open file")
                        .write_all(average.to_string().as_bytes())
                        .expect("Couldn't write to file");
                }
            }
            frame_count = 0;
            starting_time = current_frame;
        }

        let geometry_buffers = unsafe {
            active_camera.transform.take_photo(
                &mut objects[..],
                &active_camera.get_projection_matrix(),
                &scene_aabb,
                &camera_framebuffer,
                0,
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

        for (_, event) in glfw::flush_messages(&parameters.events) {
            // Events
            if let glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) = event {
                menu.toggle_showing(&mut last_x, &mut last_y);
            };
            if !ui.is_showing() {
                common::process_events(
                    &event,
                    &mut first_mouse,
                    &mut last_x,
                    &mut last_y,
                    active_camera,
                    // &mut debug_cone, // TODO: Bring back
                );
                common::handle_show_model(&event, &mut show_model);
                common::handle_show_voxel_fragment_list(&event, &mut show_voxel_fragment_list);
                common::handle_light_movement(&event, &mut should_move_light);
                common::handle_mipmap_level(&event, &mut mipmap_level);
            } else {
                menu.handle_event(event);
            }
        }

        ui.begin_frame(current_frame);

        // egui render
        // if ui.is_showing() {
        //     menu.show_main_window();
        //     menu.render((
        //         (),
        //         NodeSearchMenuInput::new(&debug_nodes),
        //         (),
        //         ChildrenMenuInput::new(&children),
        //         DiagnosticsMenuInput::new(fps),
        //         (),
        //         PhotonsMenuInput::new(&photons),
        //         SavePresetMenuInput::new(&active_camera, menu.sub_menus.clone()), // TODO: Remove clone
        //         (),
        //         (),
        //         (),
        //     ));
        //     let outputs = menu.get_data();

        //     // All nodes
        //     show_octree = outputs.0.should_render_octree;
        //     current_octree_level = outputs.0.current_octree_level;
        //     octree_nodes_to_visualize = outputs.0.octree_nodes_to_visualize.clone();

        //     // Node search
        //     selected_debug_nodes = selected_debug_nodes
        //         .into_iter()
        //         .chain(outputs.1.selected_items.clone())
        //         .collect();
        //     node_filter_text = outputs.1.filter_text.clone();
        //     should_show_neighbors = outputs.1.should_show_neighbors;
        //     selected_debug_nodes_updated = outputs.1.selected_items_updated;

        //     // Bricks
        //     bricks_to_show = outputs.2.bricks_to_show;
        //     brick_attribute = outputs.2.brick_attribute;
        //     should_show_normals = outputs.2.should_show_brick_normals;
        //     color_direction = outputs.2.color_direction;
        //     brick_padding = outputs.2.brick_padding;

        //     // Images
        //     cone_tracer.toggles = outputs.5.toggles.clone();

        //     // Camera
        //     active_camera.orthographic = outputs.8.orthographic;

        //     // Cone tracing
        //     cone_parameters.insert(
        //         "shadowConeParameters",
        //         outputs.9.shadow_cone_parameters.clone(),
        //     );
        //     cone_parameters.insert(
        //         "ambientOcclusionConeParameters",
        //         outputs.9.ambient_occlusion_cone_parameters.clone(),
        //     );
        //     cone_parameters.insert(
        //         "diffuseConeParameters",
        //         outputs.9.diffuse_cone_parameters.clone(),
        //     );
        //     cone_parameters.insert(
        //         "specularConeParameters",
        //         outputs.9.specular_cone_parameters.clone(),
        //     );
        //     // This one doesn't come from `get_data()` but is still relevant to `debug_cone`
        //     geometry_buffer_coordinates = menu.get_quad_coordinates();

        //     should_show_debug_cone = outputs.10.show_debug_cone;
        //     should_move_debug_cone = outputs.10.move_debug_cone;
        //     // TODO: there is quite a bit of cloning here
        //     debug_cone.parameters = outputs.10.cone_parameters.clone();
        //     debug_cone.point_to_light = outputs.10.point_to_light;
        //     menu.is_picking = outputs.10.is_picking;
        // }

        // This is for debugging
        // if selected_debug_nodes_updated {
        //     selected_debug_nodes_updated = false;
        //     let last_debug_node = selected_debug_nodes.last();
        //     if let Some(last_debug_node) = last_debug_node {
        //         unsafe {
        //             octree.run_get_photons_shader(last_debug_node.index());
        //             photons = helpers::get_values_from_texture_buffer(
        //                 octree.textures.photons_buffer.1,
        //                 27, // Voxels in a brick
        //                 42_u32,
        //             );
        //             octree.run_get_children_shader(last_debug_node.index());
        //             children = helpers::get_values_from_texture_buffer(
        //                 octree.textures.children_buffer.1,
        //                 8, // Children in a node
        //                 42_u32,
        //             );
        //             // octree.run_colors_quad_shader(last_debug_node.index());
        //         };
        //     }
        // }

        // Input
        if !ui.is_showing() {
            let transform = if should_move_light {
                // unsafe {
                // light_maps = // TODO: This takes too long, optimize
                //     octree.inject_light(
                //         &mut objects[..],
                //         &light,
                //         &scene_aabb,
                //     );
                light.transform_mut()
                // }
            } else if should_move_debug_cone {
                &mut debug_cone.transform
            } else {
                &mut active_camera.transform
            };
            unsafe {
                common::process_movement_input(delta_time as f32, transform);
            }
        }

        // Render
        unsafe {
            // let projection: Matrix4<f32> = perspective(
            //     Deg(active_camera.zoom),
            //     viewport_width as f32 / viewport_height as f32,
            //     0.0001,
            //     10000.0,
            // );
            let projection = active_camera.get_projection_matrix();
            let view = active_camera.transform.get_view_matrix();
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
            model = model * Matrix4::from_scale(1.);

            // if show_voxel_fragment_list {
            //     if should_move_light {
            //         render_border_voxel_fragments_shader.run(&projection, &view, &model);
            //     } else {
            //         render_voxel_fragments_shader.run(&projection, &view, &model);
            //     }
            // }

            // let node_data_to_visualize = match octree_nodes_to_visualize {
            //     OctreeDataType::Geometry => &octree.geometry_data.node_data,
            //     OctreeDataType::Border => &octree.border_data.node_data,
            // };

            // if show_octree {
            //     octree.render(
            //         &model,
            //         &view,
            //         &projection,
            //         current_octree_level,
            //         color_direction,
            //         should_show_normals,
            //         brick_attribute,
            //         brick_padding,
            //         node_data_to_visualize,
            //     );
            // }

            // octree.set_node_indices(
            //     &selected_debug_nodes
            //         .iter()
            //         .map(|node| node.index())
            //         .collect(),
            // );
            // octree.run_node_positions_shader(&projection, &view, &model);
            // octree.set_bricks_to_show(bricks_to_show);

            // if should_show_neighbors {
            //     octree.run_node_neighbors_shader(&projection, &view, &model);
            // }

            // if bricks_to_show.at_least_one() {
            //     octree.run_node_bricks_shader(
            //         &projection,
            //         &view,
            //         &model,
            //         color_direction,
            //         brick_attribute,
            //         brick_padding,
            //     );
            // }

            if show_model {
                // Render model normally
                // render_model_shader.use_program();
                // render_model_shader.set_mat4(c_str!("projection"), &projection);
                // render_model_shader.set_mat4(c_str!("view"), &view);
                // Model and model normalization matrix get set in the draw call
                // for object in objects.iter_mut() {
                //     object.draw(&render_model_shader, &model_normalization_matrix);
                // }
                simple_cone_tracer.run(ConeTracerRunInputs {
                    camera: active_camera,
                    voxels_texture,
                    light: &light,
                    scene_aabb: &scene_aabb,
                    geometry_buffers: &geometry_buffers,
                });
            } else {
                voxels_visualizer.run(VisualizerRunInputs {
                    camera: active_camera,
                    voxels_texture,
                    mipmap_level,
                });
            }

            voxelizer.run(VoxelizerRunInputs {
                objects: &mut objects[..],
                scene_aabb: &scene_aabb,
                camera: active_camera,
                light: &light,
            });

            // cube.render(&very_simple_shader, active_camera);

            // cone_tracer.run(
            //     &light,
            //     &octree.textures,
            //     &geometry_buffers,
            //     light_maps,
            //     &quad,
            //     &active_camera,
            //     &cone_parameters,
            //     if parameters.options.screenshot {
            //         Some(parameters.options.get_name())
            //     } else {
            //         None
            //     },
            // );

            // if should_show_debug_cone {
            //     debug_cone.run(
            //         &octree.textures,
            //         &projection,
            //         &view,
            //         &mut selected_debug_nodes,
            //         &geometry_buffers,
            //         &geometry_buffer_coordinates,
            //         &light,
            //     );
            // }
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

        ui.end_frame();

        // current_voxel_fragment_count =
        //     (current_voxel_fragment_count + 10000).min(number_of_voxel_fragments);

        // Swap buffers and poll I/O events
        common::swap_buffers();
        glfw.poll_events();

        if parameters.options.screenshot
            || parameters.options.record_octree_build_time
            || parameters
                .options
                .seconds_for_fps
                .is_some_and(|seconds| fps_values.len() as f64 > seconds as f64)
        {
            break;
        }
    }
}
