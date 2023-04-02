use cgmath::{point3, vec3, Deg, Euler, InnerSpace, Matrix4, Point3, Vector3, Zero};

/// Struct that handles `position`, `rotation` and `scale` for an entity
#[derive(Debug)]
pub struct Transform {
    pub position: Point3<f32>,
    pub scale: Vector3<f32>,
    rotation: Euler<f32>,
    forward: Vector3<f32>,
    up: Vector3<f32>,
    right: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        let mut this = Self {
            position: point3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
            rotation: Euler::new(0.0, 90.0, 0.0),
            forward: vec3(0.0, 0.0, 1.0),
            up: Vector3::zero(),    // Initialized later
            right: Vector3::zero(), // Initialized later
        };
        this.update_vectors();
        this
    }
}

impl Transform {
    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        let mut model = Matrix4::<f32>::from_angle_z(Deg(self.rotation.z))
            * Matrix4::<f32>::from_angle_y(Deg(self.rotation.y - 90.0))
            * Matrix4::<f32>::from_angle_x(Deg(-self.rotation.x));
        model =
            Matrix4::<f32>::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z) * model;
        model = Matrix4::<f32>::from_translation(vec3(
            self.position.x,
            self.position.y,
            self.position.z,
        )) * model;
        model
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.position + self.forward, self.up)
    }

    pub fn get_forward(&self) -> Vector3<f32> {
        self.forward
    }

    pub fn get_right(&self) -> Vector3<f32> {
        self.right
    }

    pub fn get_up(&self) -> Vector3<f32> {
        self.up
    }

    pub fn set_rotation_x(&mut self, x: f32) {
        self.rotation.x = x;
        self.update_vectors();
    }

    #[allow(dead_code)]
    pub fn set_rotation_y(&mut self, y: f32) {
        self.rotation.y = y;
        self.update_vectors();
    }

    #[allow(dead_code)]
    pub fn set_rotation_z(&mut self, z: f32) {
        self.rotation.z = z;
        self.update_vectors();
    }

    fn update_vectors(&mut self) {
        let forward = Vector3 {
            x: self.rotation.y.to_radians().cos() * self.rotation.x.to_radians().cos(),
            y: self.rotation.x.to_radians().sin(),
            z: self.rotation.y.to_radians().sin() * self.rotation.x.to_radians().cos(),
        };
        self.forward = forward.normalize();
        self.right = self.forward.cross(vec3(0.0, 1.0, 0.0)).normalize();
        self.up = self.right.cross(self.forward).normalize();
    }
}
