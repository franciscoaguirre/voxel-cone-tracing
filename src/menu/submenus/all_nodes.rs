use egui_backend::egui;

use super::super::get_button_text;
use super::SubMenu;

pub struct AllNodesMenu {
    is_showing: bool,
    output: AllNodesMenuOutput,
}

pub struct AllNodesMenuOutput {
    should_render_octree: bool,
    current_octree_level: u32,
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

    fn get_data(&self) -> Self::OutputData {
        &self.output
    }

    fn render(&self, context: egui::Context, _: &Self::InputData) {
        egui::Window::new("All Nodes").show(&context, |ui| {
            if ui
                .button(get_button_text(
                    "Show octree",
                    *self.output.should_render_octree,
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
