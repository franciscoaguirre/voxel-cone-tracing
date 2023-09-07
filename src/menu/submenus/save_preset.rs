use egui_glfw_gl::egui;
use serde::{Serialize, Deserialize};

use super::super::get_button_text;
use super::SubMenu;
use crate::Camera;
use crate::config::CONFIG;
use crate::menu::{MenuInternals, SubMenus};
use crate::octree::OctreeDataType;
use crate::preset::{Preset, save_preset};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct SavePresetMenu {
    name: String,
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
        true
    }

    fn toggle_showing(&mut self) {}

    fn get_data(&self) -> &Self::OutputData {
        &()
    }

    fn render<'b>(&mut self, internals: &MenuInternals, input: &Self::InputData<'b>) {
        egui::Window::new("Save Preset").show(&internals.context, |ui| {
            ui.text_edit_singleline(&mut self.name);
            if ui.button("Save").clicked() {
                let preset = Preset { camera: input.camera.clone(), submenus: input.submenus.clone() };
                save_preset(&self.name, preset);
            }
        });
    }
}
