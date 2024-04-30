use engine::{
    prelude::*,
    ui::{get_button_text, prelude::*},
};

#[derive(Showable)]
pub struct SystemsMenu {
    should_show: bool,
}

impl SystemsMenu {
    pub fn new() -> Self {
        Self { should_show: false }
    }
}

impl<SystemType: System + Pausable> SubMenu<SystemType> for SystemsMenu {
    fn show(&mut self, context: &egui::Context, inputs: &mut SubMenuInputs<SystemType>) {
        egui::Window::new("Systems").show(context, |ui| {
            for (info, system) in inputs.system_info.iter().zip(inputs.systems.iter_mut()) {
                ui.horizontal(|ui| {
                    let mut is_running = !system.is_paused();
                    ui.checkbox(&mut is_running, "");
                    if is_running {
                        system.unpause();
                    } else {
                        system.pause();
                    }
                    ui.label(info.name);
                    if ui.button(get_button_text("Details", false)).clicked() {
                        egui::Window::new(info.name).show(context, |ui| {
                            ui.label("Hola Flo");
                        });
                    }
                });
            }
        });
    }
}
