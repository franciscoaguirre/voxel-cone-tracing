use engine::prelude::*;
use engine::ui::get_button_text;
use engine::ui::prelude::*;

#[derive(Showable)]
pub struct PickerMenu {
    is_picking: bool,
    should_show: bool,
}

impl PickerMenu {
    pub fn new() -> Self {
        Self {
            is_picking: false,
            should_show: false,
        }
    }
}

impl SubMenu for PickerMenu {
    fn show(&mut self, context: &egui::Context, _scene: &Scene, _assets: &mut AssetRegistry) {
        egui::Window::new("Picker").show(context, |ui| {
            if ui
                .button(get_button_text("Picker", self.is_picking))
                .clicked()
            {
                self.is_picking = !self.is_picking;
            }
        });
    }

    fn handle_event(
        &mut self,
        event: &glfw::WindowEvent,
        context: &egui::Context,
        assets: &mut AssetRegistry,
    ) {
        if self.is_picking && !context.wants_pointer_input() {
            if let glfw::WindowEvent::MouseButton(_, glfw::Action::Press, _) = event {
                let cursor_position = Ui::get_cursor_pos();
                let viewport_dimensions = Ui::get_window_size();
                let quad_coordinates = (
                    cursor_position.0 / viewport_dimensions.0 as f64,
                    1.0 - (cursor_position.1 / viewport_dimensions.1 as f64),
                );
                *assets
                    .get_uniform_mut("SimpleDebugConeTracer.gBufferQueryCoordinates")
                    .unwrap() = Uniform::Vec2(quad_coordinates.0 as f32, quad_coordinates.1 as f32);
            }
        }
    }
}
