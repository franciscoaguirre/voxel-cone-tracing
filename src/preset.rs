use std::collections::HashMap;
use std::fs::File;

use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use structopt::StructOpt;

use crate::cli_arguments::Options;
use crate::menu::SubMenus;
use crate::rendering::camera::Camera;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Preset {
    pub submenus: SubMenus,
    pub camera: Camera,
}

pub static PRESET: Lazy<Preset> = Lazy::new(load_preset);

fn load_preset() -> Preset {
    let options = Options::from_args();
    let input_path = format!("presets/{}.ron", options.preset);
    let file = File::open(&input_path);
    let mut preset: Preset;
    if let Ok(file) = file {
        preset = ron::de::from_reader(file).expect("Preset file malformed!");
    } else {
        log::info!("Using default preset");
        preset = Default::default();
    }
    unsafe {
        preset.camera.transform.update_vectors();
    }
    log::info!("Preset used: {:#?}", preset);
    preset
}

pub fn save_preset(name: &str, preset: Preset) {
    let path = format!("presets/{}.ron", name);
    let mut file = File::create(&path).expect("Could not save preset file");
    let pretty_config = ron::ser::PrettyConfig::new();
    ron::ser::to_writer_pretty(&mut file, &preset, pretty_config).expect("Preset file malformed!");
}
