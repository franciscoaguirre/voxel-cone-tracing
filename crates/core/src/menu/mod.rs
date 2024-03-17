use std::fmt;
use std::fs::File;

use cgmath::{vec2, Vector2};
use engine::ui::prelude::*;
use macros::SimplifySubMenus;
use serde::{Deserialize, Serialize};

pub mod submenus;
use submenus::*;

pub struct Menu {
    pub sub_menus: SubMenus,
    quad_coordinates: Vector2<f32>, // These are just to return for debugging
    pub is_picking: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, SimplifySubMenus)]
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
    debug_cone: DebugConeMenu,
}

impl Menu {
    pub fn new(preset: Preset) -> Self {
        let sub_menus = SubMenus::default();
        let mut menu = Menu {
            sub_menus,
            quad_coordinates: vec2(0.0, 0.0),
            is_picking: false,
        };
        menu.process_preset(preset);
        menu
    }

    fn process_preset(&mut self, preset: Preset) {
        self.sub_menus = preset.submenus.clone();
        if self.sub_menus.any_showing() {
            self.toggle_showing(&mut 0.0, &mut 0.0);
        }
    }

    pub fn toggle_showing(&mut self, last_x: &mut f32, last_y: &mut f32) {
        let mut ui = Ui::instance();
        ui.toggle_showing();

        if ui.is_showing() {
            Ui::set_cursor_mode(glfw::CursorMode::Normal);
        } else {
            Ui::set_cursor_mode(glfw::CursorMode::Disabled);

            // So that we don't take into account mouse movements while using the menu
            let cursor_position = Ui::get_cursor_pos();
            *last_x = cursor_position.0 as f32;
            *last_y = cursor_position.1 as f32;
        };
    }

    pub fn handle_event(&mut self, event: glfw::WindowEvent) {
        let mut ui = Ui::instance();

        if let glfw::WindowEvent::Key(glfw::Key::LeftShift, _, glfw::Action::Press, _) = event {
            ui.toggle_shift();
        } else if let glfw::WindowEvent::Key(glfw::Key::LeftShift, _, glfw::Action::Release, _) =
            event
        {
            ui.toggle_shift();
        }

        if self.is_picking && !ui.context().wants_pointer_input() {
            if let glfw::WindowEvent::MouseButton(_, glfw::Action::Press, _) = event {
                let cursor_position = Ui::get_cursor_pos();
                let viewport_dimensions = Ui::get_window_size();
                let quad_coordinates = (
                    cursor_position.0 / viewport_dimensions.0 as f64,
                    1.0 - (cursor_position.1 / viewport_dimensions.1 as f64),
                );
                self.quad_coordinates = vec2(quad_coordinates.0 as f32, quad_coordinates.1 as f32);
            }
        }

        egui_backend::handle_event(event, ui.input_state_mut());
    }

    pub fn get_quad_coordinates(&self) -> Vector2<f32> {
        self.quad_coordinates
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
        button_text = button_text.color(egui::Color32::RED);
    }
    button_text
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Preset {
    pub submenus: SubMenus,
    pub camera: engine::camera::Camera,
}

pub fn save_preset(name: &str, preset: Preset) {
    let path = format!("presets/{}.ron", name);
    let mut file = File::create(&path).expect("Could not save preset file");
    let pretty_config = ron::ser::PrettyConfig::new();
    ron::ser::to_writer_pretty(&mut file, &preset, pretty_config).expect("Preset file malformed!");
}
