use std::fs::File;

use engine::prelude::*;

pub fn load_scene(file_name: &str) -> Scene {
    let input_path = format!("scenes/{}.ron", file_name);
    let file = File::open(&input_path).expect("Missing scene file!");
    let mut scene: Scene = ron::de::from_reader(file).expect("Scene file malformed!");
    scene.light.transform_mut().update_vectors();
    scene
}
