use std::sync::mpsc::Receiver;

use crate::pause_kernels_with_number_keys;

use egui_glfw_gl::glfw::{self, Glfw, WindowEvent};

use crate::{
    camera::Camera,
    common::{self, WINDOW},
    prelude::{AssetRegistry, Kernel, Pausable, Scene},
};

pub struct RenderLoop<T> {
    glfw: Glfw, // TODO: Remove from here.
    events: Events,
    mouse_info: MouseInfo,
    scene: Option<Scene>,
    kernels: Vec<(String, T)>,
    asset_registry: AssetRegistry,
}

impl<T: Kernel + Pausable> RenderLoop<T> {
    pub fn new(glfw: Glfw, events: Events, viewport_dimensions: (i32, i32)) -> Self {
        Self {
            glfw,
            events,
            mouse_info: MouseInfo {
                first_mouse: true,
                last_x: viewport_dimensions.0 as f32 / 2.0,
                last_y: viewport_dimensions.1 as f32 / 2.0,
            },
            scene: None,
            kernels: Vec::new(),
            asset_registry: AssetRegistry::new(),
        }
    }

    pub unsafe fn register_kernel(&mut self, name: &str, kernel: T) {
        self.kernels.push((name.to_string(), kernel));
    }

    pub unsafe fn run(&mut self) {
        println!(
            "Kernels: {:?}",
            self.kernels
                .iter()
                .map(|(name, _)| name)
                .collect::<Vec<_>>()
        );

        let scene = self.scene.as_ref().expect("Scene should've been set");

        for (_, kernel) in &mut self.kernels {
            kernel.setup(&mut self.asset_registry);
        }

        println!("Textures: {:?}", self.asset_registry.textures);

        // FPS variables.
        let mut frame_count = 0;
        let mut starting_time: f64 = self.glfw.get_time();
        let mut elapsed_time: f64;
        let mut fps: f64 = 0.0;

        // Time setup.
        // TODO: Create TimeManager.
        let mut delta_time: f64;
        let mut last_frame: f64 = 0.0;

        // Main rendering loop.
        while !common::should_close_window() {
            // Time update.
            let current_frame = self.glfw.get_time();
            frame_count += 1;
            delta_time = current_frame - last_frame;
            last_frame = current_frame;
            elapsed_time = current_frame - starting_time;

            // GL settings.
            Self::gl_start_settings();

            // Process events.
            Self::process_events(
                &self.events,
                &mut self.mouse_info,
                &mut scene.cameras[scene.active_camera.unwrap_or(0)].borrow_mut(),
                &mut self.kernels,
            );

            // Camera movement.
            common::process_movement_input(
                delta_time as f32,
                &mut scene.cameras[scene.active_camera.unwrap_or(0)]
                    .borrow_mut()
                    .transform,
            );

            // Run all updates.
            for (_, kernel) in &mut self.kernels {
                kernel.update(&scene, &mut self.asset_registry);
            }

            // Probably rendering a full-screen quad.

            // Swap buffers and poll I/O events.
            common::swap_buffers();
            self.glfw.poll_events();
        }
    }

    pub fn set_scene(&mut self, mut scene: Scene) {
        self.process_scene(&scene);
        scene.calculate_aabb(&self.asset_registry);
        self.scene = Some(scene);
    }

    unsafe fn gl_start_settings() {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::Enable(gl::DEPTH_TEST);
        gl::Disable(gl::BLEND);
        // TODO: Could add transparency.
        // gl::Enable(gl::BLEND);
        // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    fn process_scene(&mut self, scene: &Scene) {
        self.asset_registry.process_scene(scene);
    }

    fn process_events(
        events: &Events,
        mouse_info: &mut MouseInfo,
        camera: &mut Camera,
        kernels: &mut [(String, T)],
    ) {
        for (_, event) in glfw::flush_messages(events) {
            // events
            // if let glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) = event {
            //     menu.toggle_showing(&mut last_x, &mut last_y);
            // };
            // if !ui.is_showing() {
            common::process_events(
                &event, mouse_info, camera,
                // &mut debug_cone, // todo: bring back
            );
            Self::process_pausing_kernels(kernels, &event);
            // common::handle_show_model(&event, &mut show_model);
            // common::handle_show_voxel_fragment_list(&event, &mut show_voxel_fragment_list);
            // common::handle_light_movement(&event, &mut should_move_light);
            // common::handle_mipmap_level(&event, &mut mipmap_level);
            // } else {
            //     menu.handle_event(event);
            // }
        }
    }

    fn process_pausing_kernels(kernels: &mut [(String, T)], event: &glfw::WindowEvent) {
        pause_kernels_with_number_keys!(kernels, event, 0, 1, 2);
    }
}

type Events = Receiver<(f64, WindowEvent)>;

pub struct MouseInfo {
    pub first_mouse: bool,
    pub last_x: f32,
    pub last_y: f32,
}
