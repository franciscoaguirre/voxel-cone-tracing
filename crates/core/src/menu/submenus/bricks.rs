use cgmath::{vec3, InnerSpace, Vector3};
use engine::prelude::*;
use engine::ui::get_button_text;
use engine::ui::prelude::*;
use serde::{Deserialize, Serialize};

use crate::octree::{BrickAttribute, BricksToShow};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Showable)]
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

impl SubMenu for BricksMenu {
    fn show(&mut self, context: &egui::Context, scene: &Scene, assets: &mut AssetRegistry) {
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
            if ui
                .button(get_button_text(
                    "Show normals",
                    self.output.should_show_brick_normals,
                ))
                .clicked()
            {
                self.output.should_show_brick_normals = !self.output.should_show_brick_normals;
            }
            ui.add(
                egui::Slider::new(&mut self.output.color_direction.x, -1.0..=1.0)
                    .text("Color direction X"),
            );
            ui.add(
                egui::Slider::new(&mut self.output.color_direction.y, -1.0..=1.0)
                    .text("Color direction Y"),
            );
            ui.add(
                egui::Slider::new(&mut self.output.color_direction.z, -1.0..=1.0)
                    .text("Color direction Z"),
            );
            if self.output.color_direction.magnitude2() == 0.0 {
                self.output.color_direction = vec3(1.0, 0.0, 0.0);
            } else {
                self.output.color_direction = self.output.color_direction.normalize();
            }
            ui.add(
                egui::Slider::new(&mut self.output.brick_padding, 0.0..=1.0).text("Brick padding"),
            );
        });
    }
}
