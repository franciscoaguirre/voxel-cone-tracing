use engine::ui::prelude::*;
use serde::{Serialize, Deserialize};

use crate::cone_tracing::ConeParameters;
use super::super::get_button_text;
use super::SubMenu;

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
    pub shadow_cone_parameters: ConeParameters,
    pub ambient_occlusion_cone_parameters: ConeParameters,
    pub diffuse_cone_parameters: ConeParameters,
    pub specular_cone_parameters: ConeParameters,
    pub debug_cone_parameters: ConeParameters,
    pub point_to_light: bool,
    pub exposure: f32,
}

macro_rules! cone_parameters_inputs {
    ( $self:expr, $ui:expr, $( $menu_name:literal: $cone_parameters:ident ),*$(,)? ) => {
        $(
            $ui.label($menu_name);
            $ui.horizontal(|ui| {
                ui.label("Aperture (degrees):");
                ui.add(
                    egui::Slider::new(&mut $self.output.$cone_parameters.cone_angle_in_degrees, 1.0..=90.0),
                );
                ui.label("Max distance:");
                ui.add(
                    egui::Slider::new(&mut $self.output.$cone_parameters.max_distance, 0.1..=1.0),
                );
            });
        )*
    };
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

    fn render<'b>(&mut self, context: &egui::Context, _: &Self::InputData<'b>) {
        if !self.is_showing() {
            return;
        }

        egui::Window::new("Cone Tracing").show(context, |ui| {
            cone_parameters_inputs!(
                self,
                ui,
                "Shadow Cones": shadow_cone_parameters,
                "Diffuse Cones": diffuse_cone_parameters,
                "Specular Cones": specular_cone_parameters,
                "Debug Cones": debug_cone_parameters,
            );

            ui.horizontal(|ui| {
                ui.label("Exposure:");
                ui.add(
                    egui::Slider::new(&mut self.output.exposure, 0.0..=6.0),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Debug cone:");
                if ui.button(get_button_text("Show", self.output.show_debug_cone)).clicked() {
                    self.output.show_debug_cone = !self.output.show_debug_cone;
                }
                if ui.button(get_button_text("Move", self.output.move_debug_cone)).clicked() {
                    self.output.move_debug_cone = !self.output.move_debug_cone;
                }
                if ui.button(get_button_text("Point to light", self.output.point_to_light)).clicked() {
                    self.output.point_to_light = !self.output.point_to_light;
                };
            });
        });
    }
}
