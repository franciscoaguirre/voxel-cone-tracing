use std::collections::HashMap;

use crate::prelude::{Material, Model, Scene};
use crate::types::*;

pub type AssetHandle = String;
pub type SystemName = String; // TODO: Could make an enum with all system names.

pub struct AssetRegistry {
    models: HashMap<AssetHandle, Model>,
    materials: HashMap<AssetHandle, Material>,
    pub textures: HashMap<AssetHandle, Texture>,
    pub uniforms: HashMap<SystemName, HashMap<AssetHandle, Uniform>>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        AssetRegistry {
            models: HashMap::new(),
            materials: HashMap::new(),
            textures: HashMap::new(),
            uniforms: HashMap::new(),
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

    pub fn register_texture(&mut self, id: &str, texture: Texture) {
        if self.textures.insert(id.to_string(), texture).is_some() {
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

    pub fn register_uniform(&mut self, system_name: &str, id: &str, uniform: Uniform) {
        if self
            .uniforms
            .entry(system_name.to_string())
            .or_insert(HashMap::new())
            .insert(id.to_string(), uniform)
            .is_some()
        {
            // Handle overwriting an existing material if necessary.
        }
    }

    pub fn get_model(&self, id: &str) -> Option<&Model> {
        self.models.get(id)
    }

    pub fn get_material(&self, id: &str) -> Option<&Material> {
        self.materials.get(id)
    }

    pub fn get_texture(&self, id: &str) -> Option<&Texture> {
        self.textures.get(id)
    }

    pub fn get_uniform(&self, system_name: &str, id: &str) -> Option<&Uniform> {
        let error_message = format!("No registered uniforms for system: {:?}", system_name);
        let system_uniforms = self.uniforms.get(system_name).expect(&error_message);
        system_uniforms.get(id)
    }

    pub fn get_uniform_mut(&mut self, system_name: &str, id: &str) -> Option<&mut Uniform> {
        let error_message = format!("No registered uniforms for system: {:?}", system_name);
        let system_uniforms = self.uniforms.get_mut(system_name).expect(&error_message);
        system_uniforms.get_mut(id)
    }

    pub fn get_system_uniforms(
        &mut self,
        system_name: &str,
    ) -> Option<&mut HashMap<String, Uniform>> {
        self.uniforms.get_mut(system_name)
    }
}
