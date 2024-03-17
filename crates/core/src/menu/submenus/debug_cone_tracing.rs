use engine::ui::prelude::*;
use serde::{Deserialize, Serialize};

use crate::cone_tracing::ConeParameters;

use super::super::get_button_text;
use super::{cone_parameters_inputs, SubMenu};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct DebugConeMenu {
    is_showing: bool,
    output: DebugConeMenuOutput,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct DebugConeMenuOutput {
    pub is_picking: bool,
    pub show_debug_cone: bool,
    pub move_debug_cone: bool,
    pub point_to_light: bool,
    pub cone_parameters: ConeParameters,
}

impl<'a> SubMenu for DebugConeMenu {
    type InputData<'b> = ();
    type OutputData = DebugConeMenuOutput;

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

        egui::Window::new("Debug Cone").show(context, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .button(get_button_text("Picker", self.output.is_picking))
                    .clicked()
                {
                    self.output.is_picking = !self.output.is_picking;
                }
                if ui
                    .button(get_button_text("Show", self.output.show_debug_cone))
                    .clicked()
                {
                    self.output.show_debug_cone = !self.output.show_debug_cone;
                }
                if ui
                    .button(get_button_text("Move", self.output.move_debug_cone))
                    .clicked()
                {
                    self.output.move_debug_cone = !self.output.move_debug_cone;
                }
                if ui
                    .button(get_button_text(
                        "Point to light",
                        self.output.point_to_light,
                    ))
                    .clicked()
                {
                    self.output.point_to_light = !self.output.point_to_light;
                }
            });
            cone_parameters_inputs!(
                self,
                ui,
                "Cone parameters": cone_parameters,
            );
        });
    }
}
