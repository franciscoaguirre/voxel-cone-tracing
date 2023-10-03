use serde::Deserialize;

use crate::prelude::{
    Object, SpotLight, Material, MaterialProperties, Model, AssetRegistry,
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
    pub name: String,
    pub path: String,
}

pub fn process_scene(scene: Scene) {
    let mut assets = AssetRegistry::instance().lock().unwrap();
    for model in scene.models {
        let model_content = Model::new(&model.path);
        assets.register_model(model.name, model_content);
    }
    for material in scene.materials {
        assets.register_material(material.name.clone(), material);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{test_utils, Transform};

    use std::path::PathBuf;
    use std::env;
    use std::fs::File;

    use cgmath::vec3;

    fn get_test_scene() -> Scene {
        Scene {
            objects: vec![
                Object {
                    model: "cube".to_string(),
                    material: "red".to_string(),
                    transform: Transform::default(),
                },
            ],
            models: vec![
                ModelInfo {
                    name: "cube".to_string(),
                    path: "assets/models/cube.obj".to_string(),
                },
            ],
            materials: vec![
                Material {
                    name: "red".to_string(),
                    properties: MaterialProperties {
                        color: vec3(1.0, 0.0, 0.0),
                        diffuse: 1.0,
                        specular: 0.0,
                    },
                },
            ],
            light: SpotLight::default(),
        }
    }

    #[test]
    fn scene_deserialization_works() {
        let (_glfw, _window) = test_utils::init_opengl_context();

        // To go from the crate root to the workspace root
        let mut path = PathBuf::from(env::current_dir().unwrap());
        path.pop();
        path.pop();
        env::set_current_dir(path).unwrap();

        let scene_file = File::open("test_scene.ron").unwrap();
        let scene: Result<Scene, _> = ron::de::from_reader(scene_file);
        assert!(scene.is_ok());
    }

    #[test]
    fn process_scene_works() {
        let (_glfw, _window) = test_utils::init_opengl_context();

        // To go from the crate root to the workspace root
        let mut path = PathBuf::from(env::current_dir().unwrap());
        path.pop();
        path.pop();
        env::set_current_dir(path).unwrap();

        let scene = get_test_scene();

        {
            let assets = AssetRegistry::instance().lock().unwrap();

            // Models and materials are not loaded
            assert!(&assets.get_model("cube").is_none());
            assert!(&assets.get_material("red").is_none());
        }

        // Process the scene
        process_scene(scene);

        {
            let assets = AssetRegistry::instance().lock().unwrap();

            // Models and materials are now loaded
            assert!(&assets.get_model("cube").is_some());
            assert!(&assets.get_material("red").is_some());
        }
    }
}
