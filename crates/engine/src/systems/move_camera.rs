use crate::{
    input::InputManager,
    prelude::{AssetRegistry, Pausable, System},
    system::{SystemInfo, SystemInputs},
    transform::Direction,
};

#[derive(Pausable)]
pub struct MoveCamera {
    paused: bool,
    pause_next_frame: bool,
}

impl MoveCamera {
    pub fn new() -> Self {
        Self {
            paused: false,
            pause_next_frame: false,
        }
    }
}

impl System for MoveCamera {
    unsafe fn setup(&mut self, _assets: &mut AssetRegistry) {}
    unsafe fn update(&mut self, inputs: SystemInputs) {
        let input = InputManager::new();
        use egui_glfw_gl::glfw::Key;

        let camera = &mut inputs.scene.active_camera_mut();
        let light = &mut inputs.scene.light.borrow_mut();

        let transform = if input.get_key(Key::C) {
            light.transform_mut()
        } else {
            &mut camera.transform
        };

        let delta_time = inputs.time.delta_time() as f32;

        if input.get_key(Key::W) {
            transform.process_keyboard(Direction::Forward, delta_time);
        }
        if input.get_key(Key::S) {
            transform.process_keyboard(Direction::Backward, delta_time);
        }
        if input.get_key(Key::A) {
            transform.process_keyboard(Direction::Left, delta_time);
        }
        if input.get_key(Key::D) {
            transform.process_keyboard(Direction::Right, delta_time);
        }
        if input.get_key(Key::Space) {
            transform.process_keyboard(Direction::Up, delta_time);
        }
        if input.get_key(Key::LeftShift) {
            transform.process_keyboard(Direction::Down, delta_time);
        }
    }
    fn get_info(&self) -> SystemInfo {
        SystemInfo { name: "MoveCamera" }
    }
}
