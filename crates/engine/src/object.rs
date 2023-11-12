use std::fmt;

use serde::Deserialize;
use c_str_macro::c_str;
use cgmath::{vec3, Matrix4};

use crate::prelude::{Transform, AssetHandle, Shader, AssetRegistry, Material, Model};

static mut NEXT_OBJECT_ID: u32 = 0;

/// This is safe in this context because we are not multi-threaded
fn get_next_object_id() -> u32 {
    unsafe {
        let next_id = NEXT_OBJECT_ID;
        NEXT_OBJECT_ID += 1;
        next_id
    }
}

/// Object holds a handle to both a [`Model`] and a [`Material`]
/// These handles will be used to get the actual asset from the [`AssetRegistry`]
#[derive(Deserialize)]
pub struct Object {
    #[serde(skip_deserializing, default = "get_next_object_id")]
    id: u32,
    model: AssetHandle,
    material: AssetHandle,
    pub transform: Transform,
    #[serde(default)]
    is_dynamic: bool,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object {}, model {}, material {}", self.id, self.model, self.material)
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object {}", self.id)
    }
}

impl Object {
    pub fn new(
        model_handle: AssetHandle,
        material_handle: AssetHandle,
        transform: Transform,
        is_dynamic: bool,
    ) -> Self {
        Self {
            id: get_next_object_id(),
            model: model_handle,
            material: material_handle,
            transform,
            is_dynamic,
        }
    }

    pub fn draw(&self, shader: &Shader, model_normalization_matrix: &Matrix4<f32>) {
        unsafe {
            // Transform's model matrix
            shader.set_mat4(c_str!("model"), &self.transform.get_model_matrix());
            shader.set_mat4(c_str!("modelNormalizationMatrix"), model_normalization_matrix);
            shader.set_mat3(c_str!("normalMatrix"), &self.transform.get_normal_matrix());
            // Material properties
            self.material().set_uniforms(shader);
        };
        self.model().draw(&shader);
    }

    pub fn model(&self) -> &Model {
        let assets = AssetRegistry::instance();
        let model = assets.get_model(&self.model).unwrap();
        model
    }

    pub fn model_handle(&self) -> &AssetHandle {
        &self.model
    }

    pub fn material(&self) -> &Material {
        let assets = AssetRegistry::instance();
        let material = assets.get_material(&self.material).unwrap();
        material
    }

    pub fn material_handle(&self) -> &AssetHandle {
        &self.material
    }

    pub fn is_dynamic(&self) -> bool {
        self.is_dynamic
    }
}
