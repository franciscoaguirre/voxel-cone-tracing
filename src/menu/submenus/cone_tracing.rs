use egui_glfw_gl::egui;
use serde::{Serialize, Deserialize};

use super::super::get_button_text;
use super::SubMenu;
use crate::config::CONFIG;
use crate::menu::MenuInternals;
use crate::octree::OctreeDataType;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ConeTracingMenu {
    is_showing: bool,
    output: ConeTracingMenuOutput,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ConeTracingMenuOutput {
    pub show_debug_cone: bool,
    pub move_debug_cone: bool,
    pub cone_angle_in_degrees: f32,
    number_of_cones: u32,
}

impl<'a> SubMenu for ConeTracingMenu {
    type InputData<'b> = ();
    type OutputData = ConeTracingMenuOutput;

    fn is_showing(&self) -> bool {
        self.is_showing
    }

    fn toggle_showing(&mut self) {
        self.is_showing = !self.is_showing;
    }

    fn get_data(&self) -> &Self::OutputData {
        &self.output
    }

    fn render<'b>(&mut self, internals: &MenuInternals, _: &Self::InputData<'b>) {
        if !self.is_showing() {
            return;
        }

        egui::Window::new("Cone Tracing").show(&internals.context, |ui| {
            ui.horizontal(|ui| {
                ui.label("Debug cone:");
                if ui.button(get_button_text("Show", self.output.show_debug_cone)).clicked() {
                    self.output.show_debug_cone = !self.output.show_debug_cone;
                }
                if ui.button(get_button_text("Move", self.output.move_debug_cone)).clicked() {
                    self.output.move_debug_cone = !self.output.move_debug_cone;
                }
            });
            ui.label("Cone Angle (degrees):");
            ui.add(
                egui::Slider::new(&mut self.output.cone_angle_in_degrees, 1.0..=90.0),
            );
        });
    }
}
