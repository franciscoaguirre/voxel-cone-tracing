use std::fs::File;

use once_cell::sync::Lazy;
use structopt::StructOpt;

use crate::cli_arguments::Options;

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
