use std::cell::RefCell;
use std::marker::PhantomData;

pub use egui_backend::egui;
pub use egui_backend::glfw;
pub use egui_glfw_gl as egui_backend;

pub mod prelude {
    pub use super::{egui, egui_backend, glfw, Ui};
}

use crate::common::{self, WINDOW};
use crate::prelude::AssetRegistry;
use crate::submenu::Showable;
use crate::submenu::SubMenu;
use crate::submenu::SubMenuInputs;
use crate::system::Pausable;
use crate::system::System;

/// UI manager.
/// Consumer can register a custom menu that uses the available UI system.
/// TODO: Theory. See if EGUI breaks the rendering in some way.
/// I remember it did some weird things with opacity, but maybe that was
/// just how we rendered the UI.
pub struct Ui<S, SystemType> {
    painter: egui_backend::Painter,
    context: egui::Context,
    input_state: egui_backend::EguiInputState,
    modifier_keys: egui::Modifiers,
    native_pixels_per_point: f32,
    should_show: bool,
    submenus: Vec<RefCell<(String, S)>>,
    _phantom: PhantomData<SystemType>,
}

impl<S: SubMenu<SystemType> + Showable, SystemType: System + Pausable> Ui<S, SystemType> {
    pub fn new(window: &mut egui_backend::glfw::Window) -> Self {
        let painter = egui_backend::Painter::new(window);
        let context = egui::Context::default();
        let native_pixels_per_point = window.get_content_scale().0;
        let (viewport_width, viewport_height) = window.get_framebuffer_size();
        let input_state = egui_backend::EguiInputState::new(egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::new(0_f32, 0_f32),
                egui::vec2(viewport_width as f32, viewport_height as f32) / native_pixels_per_point,
            )),
            pixels_per_point: Some(native_pixels_per_point),
            ..Default::default()
        });
        let modifier_keys = egui::Modifiers::default();
        Self {
            painter,
            context,
            input_state,
            modifier_keys,
            native_pixels_per_point,
            should_show: false,
            submenus: Vec::new(),
            _phantom: PhantomData,
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

    pub fn toggle_showing(&mut self) {
        self.should_show = !self.should_show;
    }

    pub fn should_show(&self) -> bool {
        self.should_show
    }

    pub fn begin_frame(&mut self, current_frame: f64) {
        if !self.should_show {
            return;
        }

        self.input_state.input.time = Some(current_frame);
        self.input_state.input.modifiers = self.modifier_keys;
        self.context.begin_frame(self.input_state.input.take());
        self.input_state.input.pixels_per_point = Some(self.native_pixels_per_point);
    }

    pub fn end_frame(&mut self) {
        if !self.should_show {
            return;
        }

        let egui::FullOutput {
            platform_output,
            repaint_after: _,
            textures_delta,
            shapes,
        } = self.context.end_frame();
        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut self.input_state, platform_output.copied_text);
        }
        let clipped_shapes = self.context.tessellate(shapes);
        self.painter
            .paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);
    }

    pub fn register_submenu(&mut self, name: &str, submenu: S) {
        self.submenus
            .push(RefCell::new((name.to_string(), submenu)));
    }

    pub fn show(&mut self, inputs: &mut SubMenuInputs<SystemType>) {
        // Main menu to toggle submenus.
        egui::Window::new("Menu").show(self.context(), |ui| {
            for submenu in self.submenus.iter() {
                let mut submenu = submenu.borrow_mut();
                if ui
                    .button(get_button_text(&submenu.0, submenu.1.should_show()))
                    .clicked()
                {
                    submenu.1.toggle_showing();
                }
            }
        });

        // Actual submenus.
        for submenu in self.submenus.iter() {
            let mut submenu = submenu.borrow_mut();
            submenu.1.show(self.context(), inputs);
        }
    }

    pub fn handle_event(
        &mut self,
        event: glfw::WindowEvent,
        inputs: &mut SubMenuInputs<SystemType>,
    ) {
        for submenu in self.submenus.iter() {
            let mut submenu = submenu.borrow_mut();
            submenu.1.handle_event(&event, self.context(), inputs);
        }
        egui_backend::handle_event(event, self.input_state_mut());
    }
}

impl Ui<(), ()> {
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
        unsafe { common::get_framebuffer_size() }
    }
}

pub fn get_button_text(text: &str, clicked: bool) -> egui::RichText {
    let mut button_text = egui::RichText::new(text);
    if clicked {
        button_text = button_text.color(egui::Color32::RED);
    }
    button_text
}
