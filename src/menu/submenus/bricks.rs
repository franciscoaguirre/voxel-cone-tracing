use egui_backend::egui;

use super::SubMenu;
use crate::menu::{get_button_text, MenuInternals};

pub struct BricksMenu {
    is_showing: bool,
    output: BricksMenuOutput,
}

pub struct BricksMenuOutput {
    bricks_to_show: BricksToShow,
    brick_attribute: BrickAttribute,
    should_show_brick_normals: bool,
    color_direction: Vector3<f32>,
    brick_padding: f32,
}

impl SubMenu for BricksMenu {
    type InputData = ();
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

    fn render(&self, internals: MenuInternals, _: &Self::InputData) {
        egui::Window::new("Bricks").show(&internals.context, |ui| {
            ui.horizontal(|ui| {
                ui.label("Bricks: ");
                if ui
                    .button(Self::get_button_text("Z0", self.output.bricks_to_show.z0()))
                    .clicked()
                {
                    self.output.bricks_to_show.toggle_z0();
                }
                if ui
                    .button(Self::get_button_text("Z1", self.output.bricks_to_show.z1()))
                    .clicked()
                {
                    self.output.bricks_to_show.toggle_z1();
                }
                if ui
                    .button(Self::get_button_text("Z2", self.output.bricks_to_show.z2()))
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
                .button(Self::get_button_text(
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
            ui.add(egui::Slider::new(self.output.brick_padding, 0.0..=1.0).text("Brick padding"));
        });
    }
}
