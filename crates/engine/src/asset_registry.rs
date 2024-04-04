use std::collections::HashMap;

use crate::prelude::{Material, Model, Scene};
use crate::types::*;

pub type AssetHandle = String;

pub struct AssetRegistry {
    models: HashMap<AssetHandle, Model>,
    materials: HashMap<AssetHandle, Material>,
    pub textures: HashMap<AssetHandle, Texture>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        AssetRegistry {
            models: HashMap::new(),
            materials: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn process_scene(&mut self, scene: &Scene) {
        for model in scene.models.iter() {
            let model_content = Model::new(&model.path);
            self.register_model(model.name.clone(), model_content);
        }
        for material in scene.materials.iter() {
            self.register_material(material.name.clone(), material.clone());
        }
    }

    pub fn register_texture(&mut self, id: AssetHandle, texture: Texture) {
        if self.textures.insert(id, texture).is_some() {
            // Handle overwriting an existing model if necessary.
        }
    }

    fn register_model(&mut self, id: AssetHandle, model: Model) {
        if self.models.insert(id, model).is_some() {
            // Handle overwriting an existing model if necessary.
        }
    }

    fn register_material(&mut self, id: AssetHandle, material: Material) {
        if self.materials.insert(id, material).is_some() {
            // Handle overwriting an existing material if necessary.
        }
    }

    pub fn get_model(&self, id: &str) -> Option<&Model> {
        self.models.get(id)
    }

    pub fn get_material(&self, id: &str) -> Option<&Material> {
        self.materials.get(id)
    }
}
