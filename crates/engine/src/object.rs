use serde::Deserialize;
use c_str_macro::c_str;

use crate::prelude::{Transform, AssetHandle, Shader, AssetRegistry};

/// Object holds a handle to both a [`Model`] and a [`Material`]
/// These handles will be used to get the actual asset from the [`AssetRegistry`]
#[derive(Debug, Deserialize)]
pub struct Object {
    pub model: AssetHandle,
    pub material: AssetHandle,
    pub transform: Transform,
}

impl Object {
    pub fn draw(&self, shader: &Shader) {
        let assets = AssetRegistry::instance();
        let material = assets.get_material(&self.material).unwrap();
        let model = assets.get_model(&self.model).unwrap();
        unsafe {
            // Transform's model matrix
            shader.set_mat4(c_str!("model"), &self.transform.get_model_matrix());
            // Material properties
            material.set_uniforms(shader);
        };
        model.draw(&shader);
    }
}
