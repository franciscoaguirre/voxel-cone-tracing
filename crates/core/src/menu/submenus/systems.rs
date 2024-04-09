use engine::{prelude::*, ui::prelude::*};

#[derive(Showable)]
pub struct SystemsMenu {
    should_show: bool,
}

impl SystemsMenu {
    pub fn new() -> Self {
        Self { should_show: false }
    }
}

impl SubMenu for SystemsMenu {
    fn show(&mut self, context: &egui::Context, _scene: &Scene, _assets: &mut AssetRegistry) {
        egui::Window::new("Systems").show(context, |ui| {
            ui.label("Hello");
        });
    }
}
