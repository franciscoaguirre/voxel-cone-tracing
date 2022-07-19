use cgmath::{vec3, Vector3};

// Axis Aligned Bounding Box
#[derive(Debug)]
pub struct AABB {
    pub min_vertex: Vector3<f32>,
    pub max_vertex: Vector3<f32>,
}

impl Default for AABB {
    fn default() -> AABB {
        AABB {
            min_vertex: vec3(std::f32::INFINITY,std::f32::INFINITY,std::f32::INFINITY),
            max_vertex: vec3(std::f32::NEG_INFINITY,std::f32::NEG_INFINITY,std::f32::NEG_INFINITY),
        }
    }
} 

impl AABB {
    pub fn new(new_min_vertex: Vector3<f32>, new_max_vertex: Vector3<f32>) -> AABB {
        AABB {
            min_vertex: new_min_vertex,
            max_vertex: new_max_vertex
        }
    }

    // Refreshes aabb whenever a vertex is added to the structure
    pub fn refresh_aabb(&mut self, pos_x: f32, pos_y: f32, pos_z: f32) {
        // Couldn't find max() or min() functions that worked with floats
        if self.max_vertex.x < pos_x {
          self.max_vertex.x = pos_x;
        }
        if self.max_vertex.y < pos_y {
          self.max_vertex.y = pos_y;
        }
        if self.max_vertex.z < pos_z {
          self.max_vertex.z = pos_z;
        }

        if self.min_vertex.x > pos_x {
          self.min_vertex.x = pos_x;
        }
        if self.min_vertex.y > pos_y {
          self.min_vertex.y = pos_y;
        }
        if self.min_vertex.z > pos_z {
          self.min_vertex.z = pos_z;
        }
    }

    pub fn middle_point(&self) -> Vector3<f32> {
        return (self.min_vertex + self.max_vertex) / 2f32;
    }

    pub fn longer_axis_length(&self) -> f32 {
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

        return z_axis_length;
    }
}
