use std::ffi::CString;

use serde::{Serialize, Deserialize};
use engine::prelude::*;
use c_str_macro::c_str;

mod debug_cone;
pub use debug_cone::DebugCone;

mod voxel_cone_trace;
pub use voxel_cone_trace::{ConeTracer, Toggles, VisualTestsParameters};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ConeParameters {
    pub cone_angle_in_degrees: f32,
    pub max_distance: f32,
}

impl ConeParameters {
    pub unsafe fn set_uniforms(&self, parameters_key: &str, shader: &Shader) {
        let half_cone_angle_uniform_key = CString::new(format!("{parameters_key}.halfConeAngle").as_bytes()).unwrap();
        let max_distance_uniform_key = CString::new(format!("{parameters_key}.maxDistance").as_bytes()).unwrap();
        shader.set_float(&half_cone_angle_uniform_key, self.cone_angle_in_degrees.to_radians() / 2.0);
        shader.set_float(&max_distance_uniform_key, self.max_distance);
    }
}
