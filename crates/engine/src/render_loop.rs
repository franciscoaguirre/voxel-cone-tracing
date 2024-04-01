use std::{cell::RefCell, sync::mpsc::Receiver};

use egui_glfw_gl::glfw::{self, Glfw, WindowEvent};

use crate::{
    camera::Camera,
    common,
    prelude::{Kernel, Scene},
};

pub struct RenderLoop<T> {
    glfw: Glfw, // TODO: Remove from here.
    events: Events,
    mouse_info: MouseInfo,
    scene: Option<Scene>,
    kernels: Vec<T>,
}

impl<T: Kernel> RenderLoop<T> {
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
        }
    }

    pub unsafe fn register_kernel(&mut self, kernel: T) {
        self.kernels.push(kernel);
    }

    pub unsafe fn run(&mut self) {
        let scene = self.scene.as_ref().expect("Scene should've been set");

        for kernel in &mut self.kernels {
            kernel.setup();
        }

        // Time setup.
        // TODO: Create TimeManager.
        let mut delta_time: f64;
        let mut last_frame: f64 = 0.0;

        // Main rendering loop.
        while !common::should_close_window() {
            // Time update.
            let current_frame = self.glfw.get_time();
            delta_time = current_frame - last_frame;
            last_frame = current_frame;

            // GL settings.
            Self::gl_start_settings();

            // Process events.
            Self::process_events(
                &self.events,
                &mut self.mouse_info,
                &mut scene.cameras[scene.active_camera.unwrap_or(0)].borrow_mut(),
            );

            common::process_movement_input(
                delta_time as f32,
                &mut scene.cameras[scene.active_camera.unwrap_or(0)]
                    .borrow_mut()
                    .transform,
            );

            // Run all updates.
            for kernel in &mut self.kernels {
                kernel.update(&scene);
            }

            // Probably rendering a full-screen quad.

            // Swap buffers and poll I/O events.
            common::swap_buffers();
            self.glfw.poll_events();
        }
    }

    pub fn set_scene(&mut self, scene: Scene) {
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

    fn process_events(events: &Events, mouse_info: &mut MouseInfo, camera: &mut Camera) {
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
            // common::handle_show_model(&event, &mut show_model);
            // common::handle_show_voxel_fragment_list(&event, &mut show_voxel_fragment_list);
            // common::handle_light_movement(&event, &mut should_move_light);
            // common::handle_mipmap_level(&event, &mut mipmap_level);
            // } else {
            //     menu.handle_event(event);
            // }
        }
    }
}

type Events = Receiver<(f64, WindowEvent)>;

pub struct MouseInfo {
    pub first_mouse: bool,
    pub last_x: f32,
    pub last_y: f32,
}
