use egui_glfw_gl::glfw::{Action, Key};

use crate::common::WINDOW;

pub struct InputManager;

impl InputManager {
    pub fn new() -> Self {
        Self
    }

    pub fn get_key(&self, key: Key) -> bool {
        let binding = unsafe { WINDOW.borrow() };
        let window = binding.as_ref().unwrap();
        window.get_key(key) == Action::Press
    }
}
