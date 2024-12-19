use engine::ui::prelude::*;
use serde::{Serialize, Deserialize};

use super::super::get_button_text;
use super::SubMenu;
use crate::config::Config;
use crate::octree::OctreeDataType;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AllNodesMenu {
    is_showing: bool,
    pub output: AllNodesMenuOutput,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AllNodesMenuOutput {
    pub should_render_octree: bool,
    pub current_octree_level: u32,
    pub octree_nodes_to_visualize: OctreeDataType,
}

impl<'a> SubMenu for AllNodesMenu {
    type InputData<'b> = ();
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

    fn render<'b>(&mut self, context: &egui::Context, _: &Self::InputData<'b>) {
        if !self.is_showing() {
            return;
        }

        let config = Config::instance();

        egui::Window::new("All Nodes").show(context, |ui| {
            if ui
                .button(get_button_text(
                    "Show octree",
                    self.output.should_render_octree,
                ))
                .clicked()
            {
                self.output.should_render_octree = !self.output.should_render_octree;
            }
            ui.horizontal(|ui| {
                ui.label("Node type:");
                let button_text = match self.output.octree_nodes_to_visualize {
                    OctreeDataType::Geometry => "Geometry",
                    OctreeDataType::Border => "Border",
                };
                if ui.button(button_text).clicked() {
                    self.output.octree_nodes_to_visualize =
                        self.output.octree_nodes_to_visualize.next();
                }
            });
            ui.label("Octree level");
            ui.add(
                egui::Slider::new(
                    &mut self.output.current_octree_level,
                    0..=config.last_octree_level(),
                )
            );
        });
    }
}
