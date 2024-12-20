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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl Default for ConeTracingMenuOutput {
    fn default() -> Self {
        Self {
            show_debug_cone: false,
            move_debug_cone: false,
            shadow_cone_parameters: ConeParameters::default(),
            ambient_occlusion_cone_parameters: ConeParameters::default(),
            diffuse_cone_parameters: ConeParameters::default(),
            specular_cone_parameters: ConeParameters::default(),
            debug_cone_parameters: ConeParameters::default(),
            point_to_light: false,
            exposure: 1.0,
        }
    }
}

macro_rules! cone_parameters_inputs {
    ( $self:expr, $ui:expr, $( $menu_name:literal: $cone_parameters:ident ),*$(,)? ) => {
        $(
            $ui.label($menu_name);
            $ui.add(
                egui::Slider::new(&mut $self.output.$cone_parameters.cone_angle_in_degrees, 1.0..=90.0),
            );
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

            ui.label("Exposure:");
            ui.add(
                egui::Slider::new(&mut self.output.exposure, 0.0..=6.0),
            );
        });
    }
}
