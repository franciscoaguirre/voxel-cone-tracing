use std::fs::File;

use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use engine::prelude::*;
use structopt::StructOpt;
use core::menu::Preset;

use crate::cli_arguments::Options;

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
