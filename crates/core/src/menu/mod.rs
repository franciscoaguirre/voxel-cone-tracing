use std::fmt;
use std::fs::File;

use cgmath::{vec2, Vector2};
use engine::ui::prelude::*;
use serde::{Deserialize, Serialize};

pub mod submenus;
// use submenus::*;

pub struct Menu {
    // pub sub_menus: SubMenus,
}

// #[derive(Debug, Default, Serialize, Deserialize, Clone)]
// #[serde(default)]
// pub struct SubMenus {
//     all_nodes: AllNodesMenu,
//     node_search: NodeSearchMenu,
//     bricks: BricksMenu,
//     children: ChildrenMenu,
//     diagnostics: DiagnosticsMenu,
//     images: ImagesMenu,
//     photons: PhotonsMenu,
//     save_preset: SavePresetMenu,
//     camera: CameraMenu,
//     cone_tracing: ConeTracingMenu,
//     debug_cone: DebugConeMenu,
// }

// impl Menu {
//     pub fn new(preset: Preset) -> Self {
//         let sub_menus = SubMenus::default();
//         let mut menu = Menu {
//             sub_menus,
//             quad_coordinates: vec2(0.0, 0.0),
//         };
//         menu.process_preset(preset);
//         menu
//     }

//     fn process_preset(&mut self, preset: Preset) {
//         self.sub_menus = preset.submenus.clone();
//         if self.sub_menus.any_showing() {
//             self.toggle_showing(&mut 0.0, &mut 0.0);
//         }
//     }

//     pub fn handle_event(&mut self, event: glfw::WindowEvent) {
//         let mut ui = Ui::instance();

//         if let glfw::WindowEvent::Key(glfw::Key::LeftShift, _, glfw::Action::Press, _) = event {
//             ui.toggle_shift();
//         } else if let glfw::WindowEvent::Key(glfw::Key::LeftShift, _, glfw::Action::Release, _) =
//             event
//         {
//             ui.toggle_shift();
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Preset {
    // pub submenus: SubMenus,
    pub camera: engine::camera::Camera,
}

pub fn save_preset(name: &str, preset: Preset) {
    let path = format!("presets/{}.ron", name);
    let mut file = File::create(&path).expect("Could not save preset file");
    let pretty_config = ron::ser::PrettyConfig::new();
    ron::ser::to_writer_pretty(&mut file, &preset, pretty_config).expect("Preset file malformed!");
}
