use egui_backend::egui;

use super::SubMenu;
use crate::menu::{get_button_text, MenuInternals};

pub struct ChildrenMenu {
    is_showing: bool,
}

pub struct ChildrenMenuInput {
    children: Vec<u32>,
}

impl SubMenu for ChildrenMenu {
    type InputData = ChildrenMenuInput;
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

    fn render(&self, internals: MenuInternals, _: &Self::InputData) {
        egui::Window::new("Children").show(&internals.context, |ui| {
            if children.is_empty() {
                ui.label("No children data. Pick a node!");
                return;
            }

            ui.vertical(|ui| {
                for child in children.iter() {
                    ui.label(child.to_string());
                }
            });
        });
    }
}
