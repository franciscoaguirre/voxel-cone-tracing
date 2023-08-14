use std::fs::File;

use once_cell::sync::Lazy;
use serde::Deserialize;
use structopt::StructOpt;

use crate::{cli_arguments::Options, rendering::light::SpotLight};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Scene {
    /// Light of the scene for both direct and indirect illumination
    pub light: SpotLight,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
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
