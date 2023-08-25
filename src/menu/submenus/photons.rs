use egui_glfw_gl::egui;

use super::SubMenu;
use crate::menu::MenuInternals;

#[derive(Default)]
pub struct PhotonsMenu {
    is_showing: bool,
}

pub struct PhotonsMenuInput {
    photons: Vec<u32>,
}

impl PhotonsMenuInput {
    pub fn new(photons: Vec<u32>) -> Self {
        Self { photons }
    }
}

impl SubMenu for PhotonsMenu {
    type InputData = PhotonsMenuInput;
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

    fn render(&mut self, internals: &MenuInternals, input: &Self::InputData) {
        if !self.is_showing() {
            return;
        }

        egui::Window::new("Photons").show(&internals.context, |ui| {
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
