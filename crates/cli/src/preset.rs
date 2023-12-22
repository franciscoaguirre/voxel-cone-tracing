use std::fs::File;

use core::menu::Preset;
use once_cell::sync::Lazy;
use structopt::StructOpt;

use crate::cli_arguments::Options;

pub static PRESET: Lazy<Preset> = Lazy::new(load_options_preset);

pub fn load_preset(file_name: &str) -> Preset {
    let input_path = format!("presets/{}.ron", file_name);
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

fn load_options_preset() -> Preset {
    let options = Options::from_args();
    let preset = load_preset(&options.preset);
    preset
}
