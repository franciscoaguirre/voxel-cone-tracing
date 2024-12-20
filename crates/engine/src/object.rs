use serde::Deserialize;
use c_str_macro::c_str;
use cgmath::{vec3, Matrix4};

use crate::prelude::{Transform, AssetHandle, Shader, AssetRegistry, Material, Model};

/// Object holds a handle to both a [`Model`] and a [`Material`]
/// These handles will be used to get the actual asset from the [`AssetRegistry`]
#[derive(Deserialize)]
pub struct Object {
    model: AssetHandle,
    material: AssetHandle,
    pub transform: Transform,
    #[serde(skip_deserializing)]
    actual_model: Option<&'static Model>,
    #[serde(skip_deserializing)]
    actual_material: Option<&'static Material>,
}

impl Object {
    pub fn new(model_handle: AssetHandle, material_handle: AssetHandle, transform: Transform) -> Self {
        Self {
            model: model_handle,
            material: material_handle,
            transform,
            actual_model: None,
            actual_material: None,
        }
    }

    // TODO: Shouldn't need mut, but does for optimization purposes
    pub fn draw(&mut self, shader: &Shader, model_normalization_matrix: &Matrix4<f32>) {
        unsafe {
            // Transform's model matrix
            shader.set_mat4(c_str!("model"), &self.transform.get_model_matrix());
            shader.set_mat4(c_str!("modelNormalizationMatrix"), model_normalization_matrix);
            shader.set_mat3(c_str!("normalMatrix"), &self.transform.get_normal_matrix());
            // Material properties
            if let Some(material) = self.material() {
                material.set_uniforms(shader);
            }
        };
        self.model().draw(&shader);
    }

    pub fn model(&mut self) -> &Model {
        if let Some(model) = self.actual_model {
            model
        } else {
            let assets = AssetRegistry::instance();
            let model = assets.get_model(&self.model).unwrap();
            self.actual_model = Some(model);
            model
        }
    }

    pub fn model_handle(&self) -> &AssetHandle {
        &self.model
    }

    pub fn material(&mut self) -> Option<&Material> {
        if let Some(material) = self.actual_material {
            Some(material)
        } else {
            let assets = AssetRegistry::instance();
            let material = assets.get_material(&self.material);
            self.actual_material = material;
            material
        }
    }

    pub fn material_handle(&self) -> &AssetHandle {
        &self.material
    }
}
