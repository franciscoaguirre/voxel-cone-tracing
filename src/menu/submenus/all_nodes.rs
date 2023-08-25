use egui_glfw_gl::egui;
use serde::Deserialize;

use super::super::get_button_text;
use super::SubMenu;
use crate::config::CONFIG;
use crate::menu::MenuInternals;

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(default)]
pub struct AllNodesMenu {
    is_showing: bool,
    pub output: AllNodesMenuOutput,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(default)]
pub struct AllNodesMenuOutput {
    pub should_render_octree: bool,
    pub current_octree_level: u32,
}

impl SubMenu for AllNodesMenu {
    type InputData = ();
    type OutputData = AllNodesMenuOutput;

    fn is_showing(&self) -> bool {
        self.is_showing
    }

    fn toggle_showing(&mut self) {
        self.is_showing = !self.is_showing;
    }

    fn get_data(&self) -> &Self::OutputData {
        &self.output
    }

    fn render(&mut self, internals: &MenuInternals, _: &Self::InputData) {
        if !self.is_showing() {
            return;
        }

        egui::Window::new("All Nodes").show(&internals.context, |ui| {
            if ui
                .button(get_button_text(
                    "Show octree",
                    self.output.should_render_octree,
                ))
                .clicked()
            {
                self.output.should_render_octree = !self.output.should_render_octree;
            }
            ui.add(
                egui::Slider::new(
                    &mut self.output.current_octree_level,
                    0..=CONFIG.last_octree_level,
                )
                .text("Octree level"),
            );
        });
    }
}
