use renderer::ui::prelude::*;
use serde::{Serialize, Deserialize};

use super::SubMenu;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PhotonsMenu {
    is_showing: bool,
}

pub struct PhotonsMenuInput<'a> {
    photons: &'a [u32],
}

impl<'a> PhotonsMenuInput<'a> {
    pub fn new(photons: &'a [u32]) -> Self {
        Self { photons }
    }
}

impl<'a> SubMenu for PhotonsMenu {
    type InputData<'b> = PhotonsMenuInput<'b>;
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

    fn render<'b>(&mut self, context: &egui::Context, input: &Self::InputData<'b>) {
        if !self.is_showing() {
            return;
        }

        egui::Window::new("Photons").show(context, |ui| {
            if input.photons.is_empty() {
                ui.label("No photon data. Pick a node!");
                return;
            }

            ui.vertical(|ui| {
                for (index, photon) in input.photons.iter().enumerate() {
                    let x = index % 3;
                    let y = (index / 3) % 3;
                    let z = index / (3 * 3);
                    let label_text = format!("({x}, {y}, {z}): {photon}");
                    ui.label(label_text);
                }
            });
        });
    }
}
