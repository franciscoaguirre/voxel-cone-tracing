use egui_backend::egui;

use super::SubMenu;
use crate::menu::{get_button_text, MenuInternals};

pub struct ImagesMenu {
    is_showing: bool,
    output: ImagesMenuOutput,
}

pub struct ImagesMenuOutput {
    toggles: Toggles,
}

impl SubMenu for ImagesMenu {
    type InputData = ();
    type OutputData = ImagesMenuOutput;

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
        egui::Window::new("Images").show(&internals.context, |ui| {
            if ui
                .button(get_button_text(
                    "Color",
                    self.output.toggles.should_show_color(),
                ))
                .clicked()
            {
                self.output.toggles.toggle_color();
            }
            if ui
                .button(get_button_text(
                    "Direct light",
                    self.output.toggles.should_show_direct(),
                ))
                .clicked()
            {
                self.output.toggles.toggle_direct();
            }
            if ui
                .button(Self::get_button_text(
                    "Indirect diffuse",
                    self.output.toggles.should_show_indirect(),
                ))
                .clicked()
            {
                self.output.toggles.toggle_indirect();
            }
            if ui
                .button(Self::get_button_text(
                    "Indirect specular",
                    self.output.toggles.should_show_indirect_specular(),
                ))
                .clicked()
            {
                self.output.toggles.toggle_indirect_specular();
            }
            if ui
                .button(Self::get_button_text(
                    "Ambient occlusion",
                    self.output.toggles.should_show_ambient_occlusion(),
                ))
                .clicked()
            {
                self.output.toggles.toggle_ambient_occlusion();
            }
        });
    }
}
