use engine::ui::prelude::*;
use serde::{Serialize, Deserialize};

use super::super::get_button_text;
use super::SubMenu;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct CameraMenu {
    is_showing: bool,
    pub output: CameraMenuOutput,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct CameraMenuOutput {
    pub orthographic: bool,
}

impl<'a> SubMenu for CameraMenu {
    type InputData<'b> = ();
    type OutputData = CameraMenuOutput;

    fn is_showing(&self) -> bool {
        self.is_showing
    }

    fn toggle_showing(&mut self) {
        self.is_showing = !self.is_showing;
    }

    fn get_data(&self) -> &Self::OutputData {
        &self.output
    }

    fn render<'b>(&mut self, context: &egui::Context, _: &Self::InputData<'b>) {
        if !self.is_showing() {
            return;
        }

        egui::Window::new("Camera").show(context, |ui| {
            if ui.button(get_button_text("Orthographic", self.output.orthographic)).clicked() {
                self.output.orthographic = !self.output.orthographic;
            }
        });
    }
}
