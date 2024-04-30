use std::sync::mpsc::Receiver;

use crate::{
    common::WINDOW,
    pause_systems_with_number_keys,
    submenu::{Showable, SubMenu, SubMenuInputs},
    system::{SystemInfo, SystemInputs},
    time::TimeManager,
    ui::Ui,
};

use egui_glfw_gl::glfw::{self, Glfw, WindowEvent};

use crate::{
    common,
    prelude::{AssetRegistry, Pausable, Scene, System},
};

pub struct RenderLoop<SystemType, SubMenuType> {
    glfw: Glfw, // TODO: Remove from here.
    events: Events,
    mouse_info: MouseInfo,
    scene: Option<Scene>,
    systems: Vec<SystemType>,
    asset_registry: AssetRegistry,
    ui: Ui<SubMenuType, SystemType>,
    should_move_light: bool,
}

impl<SystemType: System + Pausable, SubMenuType: SubMenu<SystemType> + Showable>
    RenderLoop<SystemType, SubMenuType>
{
    pub fn new(glfw: Glfw, events: Events, viewport_dimensions: (i32, i32)) -> Self {
        let mut binding = unsafe { WINDOW.borrow_mut() };
        let mut window = binding.as_mut().unwrap();

        Self {
            glfw,
            events,
            mouse_info: MouseInfo {
                first_mouse: true,
                last_x: viewport_dimensions.0 as f32 / 2.0,
                last_y: viewport_dimensions.1 as f32 / 2.0,
            },
            scene: None,
            systems: Vec::new(),
            asset_registry: AssetRegistry::new(),
            ui: Ui::<SubMenuType, SystemType>::new(&mut window),
            should_move_light: false,
        }
    }

    pub fn register_system(&mut self, system: SystemType, running: bool) {
        let mut system = system;
        if !running {
            system.pause();
        }
        self.systems.push(system);
    }

    pub fn register_submenu(&mut self, name: &str, submenu: SubMenuType) {
        self.ui.register_submenu(name, submenu);
    }

    pub unsafe fn run(&mut self) {
        let system_info: Vec<SystemInfo> = self
            .systems
            .iter()
            .map(|system| system.get_info())
            .collect();
        println!("Systems: {:?}", system_info);

        for system in &mut self.systems {
            system.setup(&mut self.asset_registry);
        }

        println!("Textures: {:?}", self.asset_registry.textures);

        // FPS variables.
        let mut frame_count = 0;
        let mut starting_time: f64 = self.glfw.get_time();
        let mut elapsed_time: f64;
        let mut fps: f64 = 0.0;

        let mut time = TimeManager::new();

        // Main rendering loop.
        while !common::should_close_window() {
            let current_frame = time.update(&self.glfw);

            frame_count += 1;
            elapsed_time = current_frame - starting_time;

            // GL settings.
            Self::gl_start_settings();

            // Process events.
            self.process_events();

            // UI.
            self.ui.begin_frame(current_frame);

            let mut submenu_inputs = SubMenuInputs::<SystemType> {
                scene: &self.scene.as_ref().expect("Scene should've been set."),
                assets: &mut self.asset_registry,
                system_info: &system_info,
                systems: &mut self.systems,
            };

            // Menu.
            if self.ui.should_show() {
                self.ui.show(&mut submenu_inputs);
            }

            let system_inputs = SystemInputs {
                scene: &self.scene.as_ref().expect("Scene should've been set."),
                assets: &self.asset_registry,
                time: &time,
            };

            // Run all updates.
            for system in &mut self.systems {
                system.update(system_inputs);
            }

            // Probably rendering a full-screen quad.

            // Needed for the menu to render.
            gl::Disable(gl::DEPTH_TEST);

            // UI end.
            self.ui.end_frame();

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

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            // events
            if let glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) = event {
                self.ui.toggle_showing();
                if self.ui.should_show() {
                    Ui::set_cursor_mode(glfw::CursorMode::Normal);
                } else {
                    Ui::set_cursor_mode(glfw::CursorMode::Disabled);

                    // So that we don't take into account mouse movements while using the menu
                    let cursor_position = Ui::get_cursor_pos();
                    self.mouse_info.last_x = cursor_position.0 as f32;
                    self.mouse_info.last_y = cursor_position.1 as f32;
                }
            };
            if !self.ui.should_show() {
                common::process_events(
                    &event,
                    &mut self.mouse_info,
                    &mut self
                        .scene
                        .as_ref()
                        .expect("Scene should've been set.")
                        .active_camera_mut(),
                    // &mut debug_cone, // todo: bring back
                );
                Self::process_pausing_systems(&mut self.systems, &event);
                // common::handle_show_model(&event, &mut show_model);
                // common::handle_show_voxel_fragment_list(&event, &mut show_voxel_fragment_list);
                common::handle_light_movement(&event, &mut self.should_move_light);
            // common::handle_mipmap_level(&event, &mut mipmap_level);
            } else {
                let system_info: Vec<SystemInfo> = self
                    .systems
                    .iter()
                    .map(|system| system.get_info())
                    .collect();
                let mut submenu_inputs = SubMenuInputs::<SystemType> {
                    scene: &self.scene.as_ref().expect("Scene should've been set."),
                    assets: &mut self.asset_registry,
                    system_info: &system_info,
                    systems: &mut self.systems,
                };
                self.ui.handle_event(event, &mut submenu_inputs);
            }
        }
    }

    fn process_pausing_systems(systems: &mut [SystemType], event: &glfw::WindowEvent) {
        pause_systems_with_number_keys!(systems, event, 0, 1, 2, 3, 4, 5);
    }
}

type Events = Receiver<(f64, WindowEvent)>;

pub struct MouseInfo {
    pub first_mouse: bool,
    pub last_x: f32,
    pub last_y: f32,
}
