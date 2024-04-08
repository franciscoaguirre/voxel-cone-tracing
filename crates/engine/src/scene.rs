use std::cell::{RefCell, RefMut};

use cgmath::{vec3, Euler};
use serde::Deserialize;

use crate::{
    aabb::Aabb,
    prelude::{AssetRegistry, Camera, Light, Material, Object},
};

#[derive(Deserialize)]
pub struct Scene {
    /// Objects in the scene
    pub objects: Vec<RefCell<Object>>,
    /// Models to load in the `AssetRegistry`
    pub models: Vec<ModelInfo>,
    /// Materials to load in the `AssetsRegistry`
    pub materials: Vec<Material>,
    /// Light of the scene for both direct and indirect illumination
    pub light: Light,
    /// Cameras.
    #[serde(skip)]
    pub cameras: Vec<RefCell<Camera>>,
    /// Scenes can have many cameras, which could be switched at runtime.
    /// This index references the active camera.
    #[serde(skip)]
    pub active_camera: Option<usize>,
    /// For getting the model normalization matrix.
    #[serde(skip)]
    pub aabb: Aabb,
}

#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub path: String,
}

impl Scene {
    pub fn calculate_aabb(&mut self, assets: &AssetRegistry) {
        for object in self.objects.iter() {
            let object = object.borrow();
            let offset = vec3(
                object.transform.position.x,
                object.transform.position.y,
                object.transform.position.z,
            );
            let rotation = Euler {
                x: object.transform.rotation_x(),
                y: object.transform.rotation_y(),
                z: object.transform.rotation_z(),
            };
            self.aabb.join(
                &object
                    .model(assets)
                    .aabb
                    .rotated(rotation)
                    .offsetted(offset),
            );
        }
    }

    pub fn active_camera_mut(&self) -> RefMut<Camera> {
        self.cameras[self.active_camera.unwrap_or(0)].borrow_mut()
    }
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
            objects: vec![RefCell::new(Object::new(
                "cube".to_string(),
                "red".to_string(),
                Transform::default(),
            ))],
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
            cameras: Vec::new(),
            active_camera: None,
            aabb: Aabb::default(),
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

        {
            // Models and materials are now loaded
            let mut assets = AssetRegistry::new();
            assets.process_scene(&scene);
            assert!(&assets.get_model("cube").is_some());
            assert!(&assets.get_material("red").is_some());
        }

        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.objects[0].borrow().model_handle(), "cube");
        assert_eq!(scene.objects[0].borrow().material_handle(), "red");

        // Reset dir in the end
        env::set_current_dir(previous_path).unwrap();
    }
}
