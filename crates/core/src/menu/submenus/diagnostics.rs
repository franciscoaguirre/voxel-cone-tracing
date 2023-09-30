use engine::ui::prelude::*;
use serde::{Serialize, Deserialize};

use super::SubMenu;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DiagnosticsMenu;

pub struct DiagnosticsMenuInput {
    fps: f64,
}

impl DiagnosticsMenuInput {
    pub fn new(fps: f64) -> Self {
        Self { fps }
    }
}

impl<'a> SubMenu for DiagnosticsMenu {
    type InputData<'b> = DiagnosticsMenuInput;
    type OutputData = ();

    fn is_showing(&self) -> bool {
        true
    }

    fn toggle_showing(&mut self) {}

    fn get_data(&self) -> &Self::OutputData {
        &()
    }

    fn render<'b>(&mut self, context: &egui::Context, input: &Self::InputData<'b>) {
        if !self.is_showing() {
            return;
        }

        egui::Window::new("Diagnostics").show(context, |ui| {
            let fps_text = format!("FPS: {:.2}", input.fps);
            ui.label(fps_text);
        });
    }
}
