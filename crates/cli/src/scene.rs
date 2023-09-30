use std::fs::File;

use once_cell::sync::Lazy;
use serde::Deserialize;
use structopt::StructOpt;
use renderer::prelude::*;

use crate::cli_arguments::Options;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Scene {
    /// Model for the scene
    #[serde(default = "default_model")]
    pub model: String,
    /// Light of the scene for both direct and indirect illumination
    pub light: SpotLight,
}

fn default_model() -> String {
    "triangle".to_string()
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            model: "triangle".to_string(),
            light: SpotLight::default(),
        }
    }
}

pub static SCENE: Lazy<Scene> = Lazy::new(load_scene);

fn load_scene() -> Scene {
    let options = Options::from_args();
    let input_path = format!("scenes/{}.ron", options.scene);
    let file = File::open(&input_path).expect("Missing config file!");
    let mut scene: Scene = ron::de::from_reader(file).expect("Scene file malformed!");
    unsafe {
        scene.light.transform.update_vectors();
    }
    log::info!("Scene used: {:#?}", scene);
    scene
}
