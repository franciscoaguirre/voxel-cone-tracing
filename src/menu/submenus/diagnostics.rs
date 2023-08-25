use egui_glfw_gl::egui;
use serde::Deserialize;

use super::SubMenu;
use crate::menu::MenuInternals;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct DiagnosticsMenu;

pub struct DiagnosticsMenuInput {
    fps: f64,
}

impl DiagnosticsMenuInput {
    pub fn new(fps: f64) -> Self {
        Self { fps }
    }
}

impl SubMenu for DiagnosticsMenu {
    type InputData = DiagnosticsMenuInput;
    type OutputData = ();

    fn is_showing(&self) -> bool {
        true
    }

    fn toggle_showing(&mut self) {}

    fn get_data(&self) -> &Self::OutputData {
        &()
    }

    fn render(&mut self, internals: &MenuInternals, input: &Self::InputData) {
        if !self.is_showing() {
            return;
        }

        egui::Window::new("Diagnostics").show(&internals.context, |ui| {
            let fps_text = format!("FPS: {:.2}", input.fps);
            ui.label(fps_text);
        });
    }
}
