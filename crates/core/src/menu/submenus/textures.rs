use engine::{prelude::*, ui::prelude::*};

#[derive(Showable)]
pub struct TexturesMenu {
    should_show: bool,
}

impl TexturesMenu {
    pub fn new() -> Self {
        Self { should_show: false }
    }
}

impl<S> SubMenu<S> for TexturesMenu {
    fn show(&mut self, context: &egui::Context, _inputs: &mut SubMenuInputs<S>) {
        egui::Window::new("Textures").show(context, |ui| {
            // TODO: Make a `Texture` enum similar to `Uniform` that lets me know
            // what type of texture is being registered in the asset registry.
            // With that information, I can choose to only show buffer textures here,
            // with a button to fetch the first X bytes of the texture from the GPU.
            ui.label("TODO");
        });
    }
}
