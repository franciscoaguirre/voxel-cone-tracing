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
                    ui.collapsing(info.name, |ui| {
                        if let Some(system_uniforms) = inputs.assets.get_system_uniforms(info.name)
                        {
                            ui.horizontal(|ui| {
                                for (key, value) in system_uniforms.iter_mut() {
                                    ui.label(format!("{key}:"));
                                    match value {
                                        Uniform::Uint(inner) => {
                                            ui.label(inner.to_string());
                                        }
                                        Uniform::Vec2(x, y) => {
                                            ui.label(format!("({x}, {y})"));
                                        }
                                        Uniform::Vec3(x, y, z) => {
                                            ui.label(format!("({x}, {y}, {z})"));
                                        }
                                        Uniform::Bool(inner) => {
                                            ui.checkbox(inner, "");
                                        }
                                    }
                                }
                            });
                        }

                        for subsystem in system.subsystems().iter_mut() {
                            ui.horizontal(|ui| {
                                let mut is_running = !subsystem.is_paused();
                                ui.checkbox(&mut is_running, subsystem.get_info().name);
                                if is_running {
                                    subsystem.unpause();
                                } else {
                                    subsystem.pause();
                                }
                            });
                        }
                    });
                });
            }
        });
    }
}
