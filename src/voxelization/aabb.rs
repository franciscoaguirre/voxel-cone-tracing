use cgmath::{vec3, Vector3};

// Axis Aligned Bounding Box
#[derive(Debug)]
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
    /// Refreshes Aabb whenever a vertex is added to the structure
    pub fn refresh_aabb(&mut self, pos_x: f32, pos_y: f32, pos_z: f32) {
        self.max_vertex.x = pos_x.max(self.max_vertex.x);
        self.max_vertex.y = pos_y.max(self.max_vertex.y);
        self.max_vertex.z = pos_z.max(self.max_vertex.z);

        self.min_vertex.x = pos_x.min(self.min_vertex.x);
        self.min_vertex.y = pos_y.min(self.min_vertex.y);
        self.min_vertex.z = pos_z.min(self.min_vertex.z);
    }

    pub fn middle_point(&self) -> Vector3<f32> {
        //(self.min_vertex + self.max_vertex) / 2_f32
        vec3(0f32, 0f32, 0f32)
    }

    pub fn longer_axis_length(&self) -> f32 {
        let diff_vector = self.max_vertex - self.min_vertex;
        let x_axis_length = diff_vector.x;
        let y_axis_length = diff_vector.y;
        let z_axis_length = diff_vector.z;

        return 2f32;

        if x_axis_length > y_axis_length && x_axis_length > z_axis_length {
            return x_axis_length;
        }

        if y_axis_length > z_axis_length {
            return y_axis_length;
        }

        z_axis_length
    }
}
