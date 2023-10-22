use std::fmt;
use std::fs::File;

use engine::ui::prelude::*;
use serde::{Serialize, Deserialize};

pub mod submenus;
use submenus::*;

pub struct Menu {
    pub sub_menus: SubMenus,
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

impl SubMenus {
    /// Returns whether any submenu is showing right now.
    /// Used to determine whether or not the menu itself should be visible
    /// after loading a preset.
    /// We're purposely leaving out `diagnostics` since it's always open and
    /// `save_preset` since it'll always be open on a preset, unless manually edited.
    pub fn any_showing(&self) -> bool {
        self.all_nodes.is_showing()
            || self.node_search.is_showing()
            || self.bricks.is_showing()
            || self.children.is_showing()
            || self.images.is_showing()
            || self.photons.is_showing()
            || self.camera.is_showing()
            || self.cone_tracing.is_showing()
    }
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
    pub fn new(preset: Preset) -> Self {
        let sub_menus = SubMenus::default();
        let mut menu = Menu {
            sub_menus,
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

        if !ui.is_showing() {
            return;
        }

        if let glfw::WindowEvent::Key(glfw::Key::LeftShift, _, glfw::Action::Press, _) = event {
            ui.toggle_shift();
        } else if let glfw::WindowEvent::Key(glfw::Key::LeftShift, _, glfw::Action::Release, _) = event {
            ui.toggle_shift();
        }

        egui_backend::handle_event(event, ui.input_state_mut());
    }

    pub fn render(&mut self, inputs: SubMenuInputs) {
        let ui = Ui::instance();
        self.sub_menus.all_nodes.render(ui.context(), &inputs.0);
        self.sub_menus
            .node_search
            .render(ui.context(), &inputs.1);
        self.sub_menus.bricks.render(ui.context(), &inputs.2);
        self.sub_menus.children.render(ui.context(), &inputs.3);
        self.sub_menus
            .diagnostics
            .render(ui.context(), &inputs.4);
        self.sub_menus.images.render(ui.context(), &inputs.5);
        self.sub_menus.photons.render(ui.context(), &inputs.6);
        self.sub_menus.save_preset.render(ui.context(), &inputs.7);
        self.sub_menus.camera.render(ui.context(), &inputs.8);
        self.sub_menus.cone_tracing.render(ui.context(), &inputs.9);
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
        let ui = Ui::instance();
        egui::Window::new("Menu").show(ui.context(), |ui| {
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
                    "Save Preset",
                    self.sub_menus.save_preset.is_showing(),
                ))
                .clicked()
            {
                self.sub_menus.save_preset.toggle_showing();
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
