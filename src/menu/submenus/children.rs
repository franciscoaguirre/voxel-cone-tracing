use egui_glfw_gl::egui;
use serde::{Serialize, Deserialize};

use super::SubMenu;
use crate::menu::MenuInternals;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ChildrenMenu {
    is_showing: bool,
}

pub struct ChildrenMenuInput<'a> {
    children: &'a [u32],
}

impl<'a> ChildrenMenuInput<'a> {
    pub fn new(children: &'a [u32]) -> Self {
        Self { children }
    }
}

impl<'a> SubMenu for ChildrenMenu {
    type InputData<'b> = ChildrenMenuInput<'b>;
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

    fn render<'b>(&mut self, internals: &MenuInternals, input: &Self::InputData<'b>) {
        if !self.is_showing() {
            return;
        }

        egui::Window::new("Children").show(&internals.context, |ui| {
            if input.children.is_empty() {
                ui.label("No children data. Pick a node!");
                return;
            }

            ui.vertical(|ui| {
                for child in input.children.iter() {
                    ui.label(child.to_string());
                }
            });
        });
    }
}
