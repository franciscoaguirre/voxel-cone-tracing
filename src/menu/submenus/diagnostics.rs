use egui_backend::egui;

use super::SubMenu;
use crate::menu::{get_button_text, MenuInternals};

pub struct DiagnosticsMenu {
    is_showing: bool,
}

pub struct DiagnosticsMenuInput {
    fps: f64,
}

impl SubMenu for DiagnosticsMenu {
    type InputData = DiagnosticsMenuInput;
    type OutputData = ();

    fn is_showing(&self) -> bool {
        self.is_showing
    }

    fn toggle_showing(&mut self) {
        self.is_showing = !self.is_showing;
    }

    fn get_data(&self) -> &Self::OutputData {
        &()
    }

    fn render(&self, internals: MenuInternals, input: &Self::InputData) {
        egui::Window::new("Diagnostics").show(&internals.context, |ui| {
            let fps_text = format!("FPS: {fps:.2}");
            ui.label(fps_text);
        });
    }
}
