use std::collections::HashMap;
use std::fs::File;

use once_cell::sync::Lazy;
use serde::Deserialize;
use structopt::StructOpt;

use crate::cli_arguments::Options;
use crate::menu::SubMenus;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Preset {
    pub submenus: SubMenus,
}

pub static PRESET: Lazy<Preset> = Lazy::new(load_preset);

fn load_preset() -> Preset {
    let options = Options::from_args();
    let input_path = format!("presets/{}.ron", options.preset);
    let file = File::open(&input_path);
    let preset: Preset;
    if let Ok(file) = file {
        preset = ron::de::from_reader(file).expect("Preset file malformed!");
    } else {
        log::info!("Using default preset");
        preset = Default::default();
    }
    log::info!("Preset used: {:#?}", preset);
    preset
}
