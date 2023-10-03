use serde::Deserialize;
use cgmath::{Vector3};

#[derive(Debug, Deserialize)]
pub struct Material {
    pub name: String,
    pub properties: MaterialProperties,
}

#[derive(Debug, Deserialize)]
pub struct MaterialProperties {
    pub color: Vector3<f32>,
    pub diffuse: f32,
    pub specular: f32,
}
