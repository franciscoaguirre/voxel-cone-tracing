use c_str_macro::c_str;
use cgmath::Vector3;
use serde::Deserialize;

use super::prelude::Shader;

#[derive(Debug, Deserialize, Clone)]
pub struct Material {
    pub name: String,
    pub properties: MaterialProperties,
}

impl Material {
    pub unsafe fn set_uniforms(&self, shader: &Shader) {
        self.properties.set_uniforms(shader);
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct MaterialProperties {
    pub color: Vector3<f32>,
    pub specular: f32,
}

impl MaterialProperties {
    pub unsafe fn set_uniforms(&self, shader: &Shader) {
        shader.set_bool(c_str!("hasMaterial"), true);
        shader.set_vec3(
            c_str!("material.color"),
            self.color.x,
            self.color.y,
            self.color.z,
        );
        shader.set_float(c_str!("material.specular"), self.specular);
    }
}
