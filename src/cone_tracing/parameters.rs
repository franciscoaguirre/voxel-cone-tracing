use c_str_macro::c_str;
use serde::{Serialize, Deserialize};

use crate::rendering::shader::Shader;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ConeTracingParameters {
    pub cone_angle_in_degrees: f32,
    pub number_of_cones: u32,
    pub max_distance: f32, // Only affects AO
}

impl ConeTracingParameters {
    /// Helper to always set the uniforms correctly in cone tracing shaders
    pub unsafe fn set_uniforms(&self, shader: Shader) {
        dbg!(self);
        
        shader.set_float(c_str!("halfConeAngle"), self.cone_angle_in_degrees.to_radians() / 2.0);
        shader.set_float(c_str!("maxDistance"), self.max_distance);
    }
}
