use std::{ffi::c_void, mem::size_of};

use c_str_macro::c_str;
use cgmath::{point3, vec3, Deg, Euler, InnerSpace, Matrix4, Point3, Vector3, Zero};
use gl::types::GLuint;
use serde::{Serialize, Deserialize};

use super::prelude::{
    Framebuffer, RenderGizmo, Model, Shader, compile_shaders, Object,
    Aabb,
};
use super::types::*;

/// Struct that handles `position`, `rotation` and `scale` for an entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transform {
    pub position: Point3<f32>,
    #[serde(default = "default_scale")]
    pub scale: Vector3<f32>,
    #[serde(default = "default_rotation")]
    rotation: Euler<f32>,
    #[serde(skip, default = "Vector3::unit_z")]
    forward: Vector3<f32>,
    #[serde(skip, default = "Vector3::unit_y")]
    up: Vector3<f32>,
    #[serde(skip, default = "Vector3::unit_x")]
    right: Vector3<f32>,
    #[serde(default = "default_movement_speed")]
    pub movement_speed: f32,
    #[serde(skip)]
    pub vao: GLuint,
    #[serde(skip, default = "gizmo_shader")]
    shader: Shader,
    #[serde(skip, default = "default_view_map_shader")]
    view_map_shader: Shader, // TODO: It's kind of ugly to store this here
}

const fn default_scale() -> Vector3<f32> {
    vec3(1.0, 1.0, 1.0)
}

const fn default_rotation() -> Euler<f32> {
    Euler::new(0.0, 90.0, 0.0)
}

const fn default_movement_speed() -> f32 {
    1.0
}

// Defines several possible options for movement. Used as abstraction to stay away from window-system specific input methods
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}
use self::Direction::*;

impl Default for Transform {
    fn default() -> Self {
        let mut this = Self {
            position: point3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
            rotation: default_rotation(),
            forward: vec3(0.0, 0.0, 1.0),
            up: Vector3::zero(),    // Initialized later
            right: Vector3::zero(), // Initialized later
            vao: 0,                 // Initialized later
            movement_speed: 1.0,
            shader: gizmo_shader(),
            view_map_shader: default_view_map_shader(),
        };
        unsafe {
            this.setup_vao();
        }
        this.update_vectors();
        this
    }
}

fn gizmo_shader() -> Shader {
    compile_shaders!(
        "assets/shaders/debug/cubicGizmo.vert.glsl",
        "assets/shaders/debug/cubicGizmo.frag.glsl",
        "assets/shaders/debug/cubicGizmo.geom.glsl",
    )
}

// TODO: Weird to have this here
fn default_view_map_shader() -> Shader {
    compile_shaders!("assets/shaders/octree/viewMap.glsl")
}

impl RenderGizmo for Transform {
    unsafe fn draw_gizmo(&self, projection: &Matrix4<f32>, view: &Matrix4<f32>) {
        self.shader.use_program();

        self.shader.set_mat4(c_str!("projection"), projection);
        self.shader.set_mat4(c_str!("view"), view);
        self.shader
            .set_mat4(c_str!("model"), &self.get_model_matrix());

        gl::BindVertexArray(self.vao);
        gl::DrawArrays(gl::POINTS, 0, 1);
        gl::BindVertexArray(0);
    }
}

impl Transform {
    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        let mut model = Matrix4::<f32>::from_angle_z(Deg(self.rotation.z))
            * Matrix4::<f32>::from_angle_y(Deg(90.0 - self.rotation.y))
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

    pub fn rotation_x(&self) -> f32 {
        self.rotation.x
    }

    pub fn set_rotation_x(&mut self, x: f32) {
        self.rotation.x = x;
        self.update_vectors();
    }

    pub fn rotation_y(&self) -> f32 {
        self.rotation.y
    }

    pub fn set_rotation_y(&mut self, y: f32) {
        self.rotation.y = y;
        self.update_vectors();
    }

    pub fn rotation_z(&self) -> f32 {
        self.rotation.z
    }

    pub fn set_rotation_z(&mut self, z: f32) {
        self.rotation.z = z;
        self.update_vectors();
    }

    pub fn update_vectors(&mut self) {
        let forward = Vector3 {
            x: self.rotation.y.to_radians().cos() * self.rotation.x.to_radians().cos(),
            y: self.rotation.x.to_radians().sin(),
            z: self.rotation.y.to_radians().sin() * self.rotation.x.to_radians().cos(),
        };
        self.forward = forward.normalize();
        self.right = self.forward.cross(vec3(0.0, 1.0, 0.0)).normalize();
        self.up = self.right.cross(self.forward).normalize();
    }

    pub unsafe fn setup_vao(&mut self) {
        gl::GenVertexArrays(1, &mut self.vao);
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let size = size_of::<Point3<f32>>() as isize;
        let local_position = point3(0_f32, 0_f32, 0_f32);
        let data = &[local_position][0] as *const Point3<f32> as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Point3<f32>>() as i32,
            0 as *const c_void,
        );
        gl::BindVertexArray(0);
        // TODO: We wanna draw a cube and we already have a GLSL helper for drawing cubes.
        // However, we should start doing that on the CPU once instead of on the GPU every frame
        // in the geometry shader. Will speed things up a lot.
        // Not that relevant since it's debugging code.
    }

    /// Writes to a framebuffer from the transform's POV of `objects`.
    /// Used to get geometry buffers
    pub unsafe fn take_photo<const N: usize>(
        &self,
        objects: &mut [Object],
        projection: &Matrix4<f32>,
        scene_aabb: &Aabb,
        framebuffer: &Framebuffer<N>,
        voxel_dimension: u32, // TODO: Find another way. This breaks separation of concerns
    ) -> Textures<N> {
        self.view_map_shader.use_program();
        self.view_map_shader.set_mat4(c_str!("projection"), &projection);
        self.view_map_shader.set_mat4(c_str!("view"), &self.get_view_matrix());
        self.view_map_shader.set_uint(c_str!("voxelDimension"), voxel_dimension);

        gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer.fbo());
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        for object in objects.iter_mut() {
            object.draw(&self.view_map_shader, &scene_aabb.normalization_matrix());
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        framebuffer.textures()
    }

    /// Processes input received from any keyboard-like input system. Accepts input parameter in the form of camera defined ENUM (to abstract it from windowing systems)
    pub fn process_keyboard(&mut self, direction: Direction, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        if direction == Forward {
            self.position += self.get_forward() * velocity;
        }
        if direction == Backward {
            self.position += -(self.get_forward() * velocity);
        }
        if direction == Left {
            self.position += -(self.get_right() * velocity);
        }
        if direction == Right {
            self.position += self.get_right() * velocity;
        }
        if direction == Up {
            self.position += self.get_up() * velocity;
        }
        if direction == Down {
            self.position += -(self.get_up() * velocity);
        }
    }
}
