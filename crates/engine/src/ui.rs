pub use egui_glfw_gl as egui_backend;
pub use egui_backend::egui;
pub use egui_backend::glfw;

pub mod prelude {
    pub use super::{
        egui,
        glfw,
        Ui,
        egui_backend,
    };
}

use crate::common::{self, WINDOW};
use once_cell::sync::OnceCell;

static mut INSTANCE: OnceCell<Ui> = OnceCell::new();

pub struct Ui {
    painter: egui_backend::Painter,
    context: egui::Context,
    input_state: egui_backend::EguiInputState,
    modifier_keys: egui::Modifiers,
    native_pixels_per_point: f32,
    is_showing: bool,
}

impl Ui {
    pub fn instance() -> &'static mut Ui {
        unsafe {
            INSTANCE.get_mut().expect("UI should have been initialized")
        }
    }

    pub fn context(&self) -> &egui::Context {
        &self.context
    }

    pub fn input_state(&self) -> &egui_backend::EguiInputState {
        &self.input_state
    }

    pub fn input_state_mut(&mut self) -> &mut egui_backend::EguiInputState {
        &mut self.input_state
    }

    pub fn toggle_shift(&mut self) {
        self.modifier_keys.shift = !self.modifier_keys.shift;
    }

    pub fn setup(window: &mut egui_backend::glfw::Window) {
        let painter = egui_backend::Painter::new(window);
        let context = egui::Context::default();
        let native_pixels_per_point = window.get_content_scale().0;
        let (viewport_width, viewport_height) = window.get_framebuffer_size();
        let input_state = egui_backend::EguiInputState::new(egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::new(0_f32, 0_f32),
                egui::vec2(viewport_width as f32, viewport_height as f32)
                    / native_pixels_per_point,
            )),
            pixels_per_point: Some(native_pixels_per_point),
            ..Default::default()
        });
        let modifier_keys = egui::Modifiers::default();
        let ui = Ui {
            painter,
            context,
            input_state,
            modifier_keys,
            native_pixels_per_point,
            is_showing: false,
        };
        unsafe { let _ = INSTANCE.set(ui); }
    }

    pub fn toggle_showing(&mut self) {
        self.is_showing = !self.is_showing;
    }

    pub fn is_showing(&self) -> bool {
        self.is_showing
    }

    pub fn begin_frame(&mut self, current_frame: f64) {
        if !self.is_showing {
            return;
        }

        self.input_state.input.time = Some(current_frame);
        self.input_state.input.modifiers = self.modifier_keys;
        self.context
            .begin_frame(self.input_state.input.take());
        self.input_state.input.pixels_per_point =
            Some(self.native_pixels_per_point);
    }

    pub fn end_frame(&mut self) {
        if !self.is_showing {
            return;
        }

        let egui::FullOutput {
            platform_output,
            repaint_after: _,
            textures_delta,
            shapes,
        } = self.context.end_frame();
        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(
                &mut self.input_state,
                platform_output.copied_text,
            );
        }
        let clipped_shapes = self.context.tessellate(shapes);
        self.painter
            .paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);
    }

    pub fn set_cursor_mode(mode: glfw::CursorMode) {
        unsafe {
            let mut binding = WINDOW.borrow_mut();
            let window = binding.as_mut().unwrap();
            window.set_cursor_mode(mode);
        }
    }

    pub fn get_cursor_pos() -> (f64, f64) {
        unsafe {
            let binding = WINDOW.borrow();
            let window = binding.as_ref().unwrap();
            window.get_cursor_pos()
        }
    }

    pub fn get_window_size() -> (i32, i32) {
        unsafe {
            common::get_framebuffer_size()
        }
    }
}
