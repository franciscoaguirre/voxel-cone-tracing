use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

use crate::prelude::{Model, Material, Scene};

pub type AssetHandle = String;

pub struct AssetRegistry {
    models: HashMap<AssetHandle, Model>,
    materials: HashMap<AssetHandle, Material>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        AssetRegistry {
            models: HashMap::new(),
            materials: HashMap::new(),
        }
    }

    pub fn register_model(&mut self, id: AssetHandle, model: Model) {
        if self.models.insert(id, model).is_some() {
            // Handle overwriting an existing model if necessary.
        }
    }

    pub fn register_material(&mut self, id: AssetHandle, material: Material) {
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

static INSTANCE: OnceCell<AssetRegistry> = OnceCell::new();

impl AssetRegistry {
    pub fn instance() -> &'static AssetRegistry {
        if let Some(assets) = INSTANCE.get() {
            &assets
        } else { panic!("Must initialize asset registry"); }
    }

    pub unsafe fn initialize(scene: &Scene) {
        if INSTANCE.get().is_some() { panic!("Can only initialize asset registry once"); }
        let mut assets = Self::new();
        for model in scene.models.iter() {
            let model_content = Model::new(&model.path);
            assets.register_model(model.name.clone(), model_content);
        }
        for material in scene.materials.iter() {
            assets.register_material(material.name.clone(), material.clone());
        }
        let _ = INSTANCE.set(assets);
    }
}
