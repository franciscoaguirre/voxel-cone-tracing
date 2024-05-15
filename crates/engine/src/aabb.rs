use std::ffi::c_void;
use std::mem::size_of;

use cgmath::{vec3, vec4, Deg, Euler, Matrix4, Vector3};

use crate::prelude::{compile_shaders, Shader};

// Axis Aligned Bounding Box
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Aabb {
    pub min_vertex: Vector3<f32>,
    pub max_vertex: Vector3<f32>,
}

impl Default for Aabb {
    fn default() -> Aabb {
        Aabb {
            min_vertex: vec3(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY),
            max_vertex: vec3(
                std::f32::NEG_INFINITY,
                std::f32::NEG_INFINITY,
                std::f32::NEG_INFINITY,
            ),
        }
    }
}

impl Aabb {
    /// Offset the entire Aabb by a given vector
    pub fn offsetted(self, offset: Vector3<f32>) -> Self {
        Self {
            min_vertex: self.min_vertex + offset,
            max_vertex: self.max_vertex + offset,
        }
    }

    pub fn rotated(self, rotation: Euler<f32>) -> Self {
        let rotation_matrix = Matrix4::<f32>::from_angle_z(Deg(rotation.z))
            * Matrix4::<f32>::from_angle_y(Deg(90.0 - rotation.y))
            * Matrix4::<f32>::from_angle_x(Deg(-rotation.x));
        Self {
            min_vertex: (rotation_matrix
                * vec4(self.min_vertex.x, self.min_vertex.y, self.min_vertex.z, 1.0))
            .xyz(),
            max_vertex: (rotation_matrix
                * vec4(self.max_vertex.x, self.max_vertex.y, self.max_vertex.z, 1.0))
            .xyz(),
        }
    }

    /// Refreshes Aabb whenever a vertex is added to the structure
    pub fn refresh_aabb(&mut self, pos_x: f32, pos_y: f32, pos_z: f32) {
        self.max_vertex.x = pos_x.max(self.max_vertex.x);
        self.max_vertex.y = pos_y.max(self.max_vertex.y);
        self.max_vertex.z = pos_z.max(self.max_vertex.z);

        self.min_vertex.x = pos_x.min(self.min_vertex.x);
        self.min_vertex.y = pos_y.min(self.min_vertex.y);
        self.min_vertex.z = pos_z.min(self.min_vertex.z);
    }

    pub fn join(&mut self, other: &Aabb) {
        self.max_vertex.x = self.max_vertex.x.max(other.max_vertex.x);
        self.max_vertex.y = self.max_vertex.y.max(other.max_vertex.y);
        self.max_vertex.z = self.max_vertex.z.max(other.max_vertex.z);

        self.min_vertex.x = self.min_vertex.x.min(other.min_vertex.x);
        self.min_vertex.y = self.min_vertex.y.min(other.min_vertex.y);
        self.min_vertex.z = self.min_vertex.z.min(other.min_vertex.z);
    }

    pub fn normalization_matrix(&self) -> Matrix4<f32> {
        let center_matrix = Matrix4::from_translation(-self.middle_point());
        let normalize_size_matrix = Matrix4::from_scale(2_f32 / self.longer_axis_length());
        normalize_size_matrix * center_matrix
    }

    fn middle_point(&self) -> Vector3<f32> {
        (self.min_vertex + self.max_vertex) / 2_f32
    }

    fn longer_axis_length(&self) -> f32 {
        let diff_vector = self.max_vertex - self.min_vertex;
        let x_axis_length = diff_vector.x;
        let y_axis_length = diff_vector.y;
        let z_axis_length = diff_vector.z;

        if x_axis_length > y_axis_length && x_axis_length > z_axis_length {
            return x_axis_length;
        }

        if y_axis_length > z_axis_length {
            return y_axis_length;
        }

        z_axis_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_with_default_gives_same() {
        let mut aabb = Aabb {
            min_vertex: vec3(0.0, -1.0, -1.0),
            max_vertex: vec3(1.0, 1.0, 1.0),
        };
        let aabb_before = aabb.clone();
        let default_aabb = Aabb::default();
        aabb.join(&default_aabb);
        assert_eq!(aabb, aabb_before);
    }

    #[test]
    fn join_works() {
        let mut aabb_1 = Aabb {
            min_vertex: vec3(0.0, -1.0, -1.0),
            max_vertex: vec3(1.0, 1.0, 1.0),
        };
        let aabb_2 = Aabb {
            min_vertex: vec3(-1.0, -1.0, -1.0),
            max_vertex: vec3(0.0, 1.0, 1.0),
        };
        let expected = Aabb {
            min_vertex: vec3(-1.0, -1.0, -1.0),
            max_vertex: vec3(1.0, 1.0, 1.0),
        };
        aabb_1.join(&aabb_2);
        assert_eq!(aabb_1, expected);
    }
}
