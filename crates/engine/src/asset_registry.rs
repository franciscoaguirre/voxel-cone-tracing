use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

use crate::prelude::{Model, Material};

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

    // Other methods...
}

impl AssetRegistry {
    pub fn instance() -> &'static Arc<Mutex<AssetRegistry>> {
        static INSTANCE: OnceCell<Arc<Mutex<AssetRegistry>>> = OnceCell::new();
        INSTANCE.get_or_init(|| Arc::new(Mutex::new(AssetRegistry::new())))
    }
}
