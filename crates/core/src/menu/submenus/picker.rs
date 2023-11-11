use engine::ui::prelude::*;
use serde::{Serialize, Deserialize};

use super::super::get_button_text;
use super::SubMenu;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct PickerMenu {
    is_showing: bool,
    output: PickerMenuOutput,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct PickerMenuOutput {
    pub is_picking: bool,
}

impl<'a> SubMenu for PickerMenu {
    type InputData<'b> = ();
    type OutputData = PickerMenuOutput;

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

        egui::Window::new("Picker").show(context, |ui| {
            if ui.button(get_button_text("Toggle", self.output.is_picking)).clicked() {
                self.output.is_picking = !self.output.is_picking;
            }
        });
    }
}
