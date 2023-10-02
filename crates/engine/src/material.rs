use serde::Deserialize;
use cgmath::{Vector3};

#[derive(Debug, Deserialize)]
pub struct Material {
    name: String,
    properties: MaterialProperties,
}

#[derive(Debug, Deserialize)]
struct MaterialProperties {
    color: Vector3<f32>,
    diffuse: f32,
    specular: f32,
}
