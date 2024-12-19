use cgmath::{vec3, InnerSpace, Vector3};
use engine::ui::prelude::*;
use serde::{Serialize, Deserialize};

use super::SubMenu;
use crate::{
    menu::get_button_text,
    octree::{BrickAttribute, BricksToShow},
};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BricksMenu {
    is_showing: bool,
    output: BricksMenuOutput,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BricksMenuOutput {
    pub brick_attribute: BrickAttribute,
    pub brick_padding: f32,
    pub bricks_to_show: BricksToShow,
    pub color_direction: Vector3<f32>,
    pub should_show_brick_normals: bool,
}

impl Default for BricksMenuOutput {
    fn default() -> Self {
        Self {
            bricks_to_show: BricksToShow::default(),
            brick_attribute: BrickAttribute::default(),
            should_show_brick_normals: false,
            color_direction: vec3(1.0, 0.0, 0.0),
            brick_padding: 0.0,
        }
    }
}

impl<'a> SubMenu for BricksMenu {
    type InputData<'b> = ();
    type OutputData = BricksMenuOutput;

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

        egui::Window::new("Bricks").show(context, |ui| {
            ui.horizontal(|ui| {
                ui.label("Bricks: ");
                if ui
                    .button(get_button_text("Z0", self.output.bricks_to_show.z0()))
                    .clicked()
                {
                    self.output.bricks_to_show.toggle_z0();
                }
                if ui
                    .button(get_button_text("Z1", self.output.bricks_to_show.z1()))
                    .clicked()
                {
                    self.output.bricks_to_show.toggle_z1();
                }
                if ui
                    .button(get_button_text("Z2", self.output.bricks_to_show.z2()))
                    .clicked()
                {
                    self.output.bricks_to_show.toggle_z2();
                }
            });
            ui.horizontal(|ui| {
                ui.label("Brick attribute: ");
                let button_text = match self.output.brick_attribute {
                    BrickAttribute::None => "None",
                    BrickAttribute::Color => "Color",
                    BrickAttribute::Photons => "Photons",
                };
                if ui.button(button_text).clicked() {
                    self.output.brick_attribute = self.output.brick_attribute.next();
                }
            });
            ui.label("X");
            ui.add(
                egui::Slider::new(&mut self.output.color_direction.x, -1.0..=1.0)
            );
            ui.label("Y");
            ui.add(
                egui::Slider::new(&mut self.output.color_direction.y, -1.0..=1.0)
            );
            ui.label("Z");
            ui.add(
                egui::Slider::new(&mut self.output.color_direction.z, -1.0..=1.0)
            );
            if self.output.color_direction.magnitude2() == 0.0 {
                self.output.color_direction = vec3(1.0, 0.0, 0.0);
            } else {
                self.output.color_direction = self.output.color_direction.normalize();
            }
        });
    }
}
