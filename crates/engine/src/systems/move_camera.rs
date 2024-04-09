use crate::{
    input::InputManager,
    prelude::{AssetRegistry, Pausable, Scene, System},
    time::TimeManager,
    transform::Direction,
};

#[derive(Pausable)]
pub struct MoveCamera {
    paused: bool,
}

impl MoveCamera {
    pub fn new() -> Self {
        Self { paused: false }
    }
}

impl System for MoveCamera {
    unsafe fn setup(&mut self, _assets: &mut AssetRegistry) {}
    unsafe fn update(&mut self, scene: &Scene, _assets: &AssetRegistry, time: &TimeManager) {
        let camera = &mut scene.active_camera_mut();
        let input = InputManager::new();

        use egui_glfw_gl::glfw::Key;

        let delta_time = time.delta_time() as f32;

        if input.get_key(Key::W) {
            camera
                .transform
                .process_keyboard(Direction::Forward, delta_time);
        }
        if input.get_key(Key::S) {
            camera
                .transform
                .process_keyboard(Direction::Backward, delta_time);
        }
        if input.get_key(Key::A) {
            camera
                .transform
                .process_keyboard(Direction::Left, delta_time);
        }
        if input.get_key(Key::D) {
            camera
                .transform
                .process_keyboard(Direction::Right, delta_time);
        }
        if input.get_key(Key::Space) {
            camera.transform.process_keyboard(Direction::Up, delta_time);
        }
        if input.get_key(Key::LeftShift) {
            camera
                .transform
                .process_keyboard(Direction::Down, delta_time);
        }
    }
}
