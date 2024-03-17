use engine::ui::prelude::*;
use serde::{Deserialize, Serialize};

use super::super::get_button_text;
use super::{cone_parameters_inputs, SubMenu};
use crate::cone_tracing::ConeParameters;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ConeTracingMenu {
    is_showing: bool,
    output: ConeTracingMenuOutput,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ConeTracingMenuOutput {
    pub shadow_cone_parameters: ConeParameters,
    pub ambient_occlusion_cone_parameters: ConeParameters,
    pub diffuse_cone_parameters: ConeParameters,
    pub specular_cone_parameters: ConeParameters,
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
                "Ambient Occlusion": ambient_occlusion_cone_parameters,
                "Diffuse Cones": diffuse_cone_parameters,
                "Specular Cones": specular_cone_parameters,
            );
        });
    }
}
