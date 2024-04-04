use c_str_macro::c_str;
use cgmath::Matrix4;
use serde::Deserialize;

use crate::prelude::{AssetHandle, AssetRegistry, Material, Model, Shader, Transform};

/// Object holds a handle to both a [`Model`] and a [`Material`]
/// These handles will be used to get the actual asset from the [`AssetRegistry`]
#[derive(Deserialize)]
pub struct Object {
    model: AssetHandle,
    material: AssetHandle,
    pub transform: Transform,
}

impl Object {
    pub fn new(
        model_handle: AssetHandle,
        material_handle: AssetHandle,
        transform: Transform,
    ) -> Self {
        Self {
            model: model_handle,
            material: material_handle,
            transform,
        }
    }

    pub fn draw(
        &self,
        shader: &Shader,
        model_normalization_matrix: &Matrix4<f32>,
        assets: &AssetRegistry,
    ) {
        unsafe {
            // Transform's model matrix
            shader.set_mat4(c_str!("model"), &self.transform.get_model_matrix());
            shader.set_mat4(
                c_str!("modelNormalizationMatrix"),
                model_normalization_matrix,
            );
            shader.set_mat3(c_str!("normalMatrix"), &self.transform.get_normal_matrix());
            // Material properties
            self.material(assets).set_uniforms(shader);
        };
        self.model(assets).draw(&shader);
    }

    pub fn model<'a>(&self, assets: &'a AssetRegistry) -> &'a Model {
        let model = assets
            .get_model(&self.model)
            .expect("Model should exist at this point.");
        model
    }

    pub fn model_handle(&self) -> &AssetHandle {
        &self.model
    }

    pub fn material<'a>(&self, assets: &'a AssetRegistry) -> &'a Material {
        let material = assets
            .get_material(&self.material)
            .expect("Material should exist at this point.");
        material
    }

    pub fn material_handle(&self) -> &AssetHandle {
        &self.material
    }
}
