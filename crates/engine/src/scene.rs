use serde::Deserialize;

use crate::prelude::{
    Object, SpotLight, Material,
};

#[derive(Debug, Deserialize)]
pub struct Scene {
    /// Objects in the scene
    pub objects: Vec<Object>,
    /// Models to load in the `AssetRegistry`
    pub models: Vec<ModelInfo>,
    /// Materials to load in the `AssetsRegistry`
    pub materials: Vec<Material>,
    /// Light of the scene for both direct and indirect illumination
    pub light: SpotLight,
}

#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    name: String,
    path: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    use std::path::PathBuf;
    use std::env;
    use std::fs::File;

    #[test]
    fn scene_deserialization_works() {
        let (_glfw, _window) = test_utils::init_opengl_context();

        // To go from the crate root to the workspace root
        let mut path = PathBuf::from(env::current_dir().unwrap());
        path.pop();
        path.pop();
        env::set_current_dir(path).unwrap();

        let scene_file = File::open("test_scene.ron").unwrap();
        let scene: Scene = ron::de::from_reader(scene_file).unwrap();
        dbg!(&scene);
    }
}
