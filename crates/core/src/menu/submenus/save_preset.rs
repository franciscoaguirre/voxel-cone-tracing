use serde::{Serialize, Deserialize};
use engine::prelude::*;
use engine::ui::prelude::*;

use super::SubMenu;
use crate::menu::{SubMenus, Preset, save_preset};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct SavePresetMenu {
    name: String,
    is_showing: bool,
}

#[derive(Debug, Clone)]
pub struct SavePresetMenuInput<'a> {
    camera: &'a Camera,
    submenus: SubMenus,
}

impl<'a> SavePresetMenuInput<'a> {
    pub fn new(camera: &'a Camera, submenus: SubMenus) -> Self {
        Self { camera, submenus }
    }
}

impl<'a> SubMenu for SavePresetMenu {
    type InputData<'b> = SavePresetMenuInput<'b>;
    type OutputData = ();

    fn is_showing(&self) -> bool {
        self.is_showing
    }

    fn toggle_showing(&mut self) {
        self.is_showing = !self.is_showing;
    }

    fn get_data(&self) -> &Self::OutputData {
        &()
    }

    fn render<'b>(&mut self, context: &egui::Context, input: &Self::InputData<'b>) {
        if !self.is_showing() {
            return;
        }

        egui::Window::new("Save Preset").show(context, |ui| {
            ui.text_edit_singleline(&mut self.name);
            if ui.button("Save").clicked() {
                let preset = Preset { camera: input.camera.clone(), submenus: input.submenus.clone() };
                save_preset(&self.name, preset);
            }
        });
    }
}
