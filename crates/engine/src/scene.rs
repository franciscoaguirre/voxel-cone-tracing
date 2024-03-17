use serde::Deserialize;

use crate::prelude::{AssetRegistry, Light, Material, Object};

#[derive(Deserialize)]
pub struct Scene {
    /// Objects in the scene
    pub objects: Vec<Object>,
    /// Models to load in the `AssetRegistry`
    pub models: Vec<ModelInfo>,
    /// Materials to load in the `AssetsRegistry`
    pub materials: Vec<Material>,
    /// Light of the scene for both direct and indirect illumination
    pub light: Light,
}

#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub path: String,
}

pub fn process_scene(scene: Scene) -> (Vec<Object>, Light) {
    unsafe { AssetRegistry::initialize(&scene) };
    (scene.objects, scene.light)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{test_utils, MaterialProperties, Transform};

    use std::env;
    use std::fs::File;

    use cgmath::vec3;

    fn get_test_scene() -> Scene {
        Scene {
            objects: vec![Object::new(
                "cube".to_string(),
                "red".to_string(),
                Transform::default(),
            )],
            models: vec![ModelInfo {
                name: "cube".to_string(),
                path: "assets/models/cube.obj".to_string(),
            }],
            materials: vec![Material {
                name: "red".to_string(),
                properties: MaterialProperties {
                    color: vec3(1.0, 0.0, 0.0),
                    specular: 0.0,
                },
            }],
            light: Light::default(),
        }
    }

    #[test]
    fn scene_deserialization_works() {
        let (_glfw, _window) = test_utils::init_opengl_context();

        let previous_path = env::current_dir().unwrap();
        // To go from the crate root to the workspace root
        let mut path = previous_path.clone();
        path.pop();
        path.pop();
        env::set_current_dir(path).unwrap();

        let scene_file = File::open("test_scene.ron").unwrap();
        let scene: Result<Scene, _> = ron::de::from_reader(scene_file);
        assert!(scene.is_ok());

        // Reset dir in the end
        env::set_current_dir(previous_path).unwrap();
    }

    #[test]
    fn process_scene_works() {
        let (_glfw, _window) = test_utils::init_opengl_context();

        let previous_path = env::current_dir().unwrap();
        // To go from the crate root to the workspace root
        let mut path = previous_path.clone();
        path.pop();
        path.pop();
        env::set_current_dir(path).unwrap();

        let scene = get_test_scene();

        // Process the scene
        let (objects, _light) = process_scene(scene);

        {
            // Models and materials are now loaded
            let assets = AssetRegistry::instance();
            assert!(&assets.get_model("cube").is_some());
            assert!(&assets.get_material("red").is_some());
        }

        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].model_handle(), "cube");
        assert_eq!(objects[0].material_handle(), "red");

        // Reset dir in the end
        env::set_current_dir(previous_path).unwrap();
    }
}
