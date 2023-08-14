use std::fs::File;

use once_cell::sync::Lazy;
use serde::Deserialize;
use structopt::StructOpt;

use crate::cli_arguments::Options;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Preset {
    pub 
}

pub static PRESET: Lazy<Preset> = Lazy::new(load_preset);

fn load_preset() -> Preset {
    let options = Options::from_args();
    let input_path = format!("presets/{}.ron", options.preset);
    let file = File::open(&input_path).expect("Missing preset file!");
    let preset: Preset = ron::de::from_reader(file).expect("Preset file malformed!");
    log::info!("Preset used: {:#?}", preset);
    preset
}
