use std::fmt;

use egui_backend::{
    egui::{self, vec2, Color32, Pos2, Rect},
    glfw::{Action, CursorMode, Key, Window, WindowEvent},
};
use egui_glfw_gl as egui_backend;
use serde::{Serialize, Deserialize};

use crate::{config::CONFIG, preset::PRESET};

pub mod submenus;
use submenus::*;

pub struct Menu {
    is_showing: bool,
    internals: MenuInternals,
    pub sub_menus: SubMenus,
}

pub struct MenuInternals {
    painter: egui_backend::Painter,
    context: egui::Context,
    input_state: egui_backend::EguiInputState,
    modifier_keys: egui::Modifiers,
    native_pixels_per_point: f32,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct SubMenus {
    all_nodes: AllNodesMenu,
    node_search: NodeSearchMenu,
    bricks: BricksMenu,
    children: ChildrenMenu,
    diagnostics: DiagnosticsMenu,
    images: ImagesMenu,
    photons: PhotonsMenu,
    save_preset: SavePresetMenu,
    camera: CameraMenu,
    cone_tracing: ConeTracingMenu,
}

type SubMenuInputs<'a> = (
    <AllNodesMenu as SubMenu>::InputData<'a>,
    <NodeSearchMenu as SubMenu>::InputData<'a>,
    <BricksMenu as SubMenu>::InputData<'a>,
    <ChildrenMenu as SubMenu>::InputData<'a>,
    <DiagnosticsMenu as SubMenu>::InputData<'a>,
    <ImagesMenu as SubMenu>::InputData<'a>,
    <PhotonsMenu as SubMenu>::InputData<'a>,
    <SavePresetMenu as SubMenu>::InputData<'a>,
    <CameraMenu as SubMenu>::InputData<'a>,
    <ConeTracingMenu as SubMenu>::InputData<'a>,
);

type SubMenuOutputs<'a> = (
    &'a <AllNodesMenu as SubMenu>::OutputData,
    &'a <NodeSearchMenu as SubMenu>::OutputData,
    &'a <BricksMenu as SubMenu>::OutputData,
    &'a <ChildrenMenu as SubMenu>::OutputData,
    &'a <DiagnosticsMenu as SubMenu>::OutputData,
    &'a <ImagesMenu as SubMenu>::OutputData,
    &'a <PhotonsMenu as SubMenu>::OutputData,
    &'a <SavePresetMenu as SubMenu>::OutputData,
    &'a <CameraMenu as SubMenu>::OutputData,
    &'a <ConeTracingMenu as SubMenu>::OutputData,
);

impl Menu {
    pub fn new(window: &mut Window) -> Self {
        let mut menu = Self::setup_egui(window);
        menu.process_preset();
        menu
    }

    fn process_preset(&mut self) {
        self.sub_menus = PRESET.submenus.clone();
    }

    pub fn toggle_showing(&mut self, window: &mut Window, last_x: &mut f32, last_y: &mut f32) {
        self.is_showing = !self.is_showing;

        if self.is_showing {
            window.set_cursor_mode(CursorMode::Normal);
        } else {
            window.set_cursor_mode(CursorMode::Disabled);

            // So that we don't take into account mouse movements while using the menu
            let cursor_position = window.get_cursor_pos();
            *last_x = cursor_position.0 as f32;
            *last_y = cursor_position.1 as f32;
        };
    }

    pub fn is_showing(&self) -> bool {
        self.is_showing
    }

    pub fn handle_event(&mut self, event: WindowEvent) {
        if !self.is_showing {
            return;
        }

        if let WindowEvent::Key(Key::LeftShift, _, Action::Press, _) = event {
            self.internals.modifier_keys.shift = true;
        } else if let WindowEvent::Key(Key::LeftShift, _, Action::Release, _) = event {
            self.internals.modifier_keys.shift = false;
        }

        egui_backend::handle_event(event, &mut self.internals.input_state);
    }

    pub fn begin_frame(&mut self, current_frame: f64) {
        if !self.is_showing {
            return;
        }

        self.internals.input_state.input.time = Some(current_frame);
        self.internals.input_state.input.modifiers = self.internals.modifier_keys;
        self.internals
            .context
            .begin_frame(self.internals.input_state.input.take());
        self.internals.input_state.input.pixels_per_point =
            Some(self.internals.native_pixels_per_point);
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
        } = self.internals.context.end_frame();
        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(
                &mut self.internals.input_state,
                platform_output.copied_text,
            );
        }
        let clipped_shapes = self.internals.context.tessellate(shapes);
        self.internals
            .painter
            .paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);
    }

    fn setup_egui(window: &mut Window) -> Menu {
        let painter = egui_backend::Painter::new(window);
        let context = egui::Context::default();
        let native_pixels_per_point = window.get_content_scale().0;
        let input_state = egui_backend::EguiInputState::new(egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0_f32, 0_f32),
                vec2(CONFIG.viewport_width as f32, CONFIG.viewport_height as f32)
                    / native_pixels_per_point,
            )),
            pixels_per_point: Some(native_pixels_per_point),
            ..Default::default()
        });
        let modifier_keys = egui::Modifiers::default();
        let internals = MenuInternals {
            painter,
            context,
            input_state,
            modifier_keys,
            native_pixels_per_point,
        };
        let sub_menus = SubMenus::default();

        Self {
            is_showing: false,
            internals,
            sub_menus,
        }
    }

    pub fn render(&mut self, inputs: SubMenuInputs) {
        self.sub_menus.all_nodes.render(&self.internals, &inputs.0);
        self.sub_menus
            .node_search
            .render(&self.internals, &inputs.1);
        self.sub_menus.bricks.render(&self.internals, &inputs.2);
        self.sub_menus.children.render(&self.internals, &inputs.3);
        self.sub_menus
            .diagnostics
            .render(&self.internals, &inputs.4);
        self.sub_menus.images.render(&self.internals, &inputs.5);
        self.sub_menus.photons.render(&self.internals, &inputs.6);
        self.sub_menus.save_preset.render(&self.internals, &inputs.7);
        self.sub_menus.camera.render(&self.internals, &inputs.8);
        self.sub_menus.cone_tracing.render(&self.internals, &inputs.9);
    }

    pub fn get_data(&self) -> SubMenuOutputs {
        (
            self.sub_menus.all_nodes.get_data(),
            self.sub_menus.node_search.get_data(),
            self.sub_menus.bricks.get_data(),
            self.sub_menus.children.get_data(),
            self.sub_menus.diagnostics.get_data(),
            self.sub_menus.images.get_data(),
            self.sub_menus.photons.get_data(),
            self.sub_menus.save_preset.get_data(),
            self.sub_menus.camera.get_data(),
            self.sub_menus.cone_tracing.get_data(),
        )
    }

    pub fn show_main_window(&mut self) {
        egui::Window::new("Menu").show(&self.internals.context, |ui| {
            if ui
                .button(get_button_text(
                    "All nodes",
                    self.sub_menus.all_nodes.is_showing(),
                ))
                .clicked()
            {
                self.sub_menus.all_nodes.toggle_showing();
            }
            if ui
                .button(get_button_text(
                    "Node search",
                    self.sub_menus.node_search.is_showing(),
                ))
                .clicked()
            {
                self.sub_menus.node_search.toggle_showing();
            }
            if ui
                .button(get_button_text(
                    "Bricks",
                    self.sub_menus.bricks.is_showing(),
                ))
                .clicked()
            {
                self.sub_menus.bricks.toggle_showing();
            }
            if ui
                .button(get_button_text(
                    "Photons",
                    self.sub_menus.photons.is_showing(),
                ))
                .clicked()
            {
                self.sub_menus.photons.toggle_showing();
            }
            if ui
                .button(get_button_text(
                    "Children",
                    self.sub_menus.children.is_showing(),
                ))
                .clicked()
            {
                self.sub_menus.children.toggle_showing();
            }
            if ui
                .button(get_button_text(
                    "Images",
                    self.sub_menus.images.is_showing(),
                ))
                .clicked()
            {
                self.sub_menus.images.toggle_showing();
            }
            if ui
                .button(get_button_text(
                    "Camera",
                    self.sub_menus.camera.is_showing(),
                ))
                .clicked()
            {
                self.sub_menus.camera.toggle_showing();
            }
            if ui
                .button(get_button_text(
                    "Cone Tracing",
                    self.sub_menus.cone_tracing.is_showing(),
                ))
                .clicked()
            {
                self.sub_menus.cone_tracing.toggle_showing();
            }
        });
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DebugNode {
    index: u32,
    text: String,
}

impl DebugNode {
    pub fn new(index: u32, text: String) -> Self {
        Self { index, text }
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

impl fmt::Display for DebugNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.index, self.text)
    }
}

pub fn get_button_text(text: &str, clicked: bool) -> egui::RichText {
    let mut button_text = egui::RichText::new(text);
    if clicked {
        button_text = button_text.color(Color32::RED);
    }
    button_text
}
