use std::{ffi::c_void, mem::size_of};

use c_str_macro::c_str;
use cgmath::{point3, vec3, Deg, Euler, InnerSpace, Matrix4, Point3, Vector3, Zero};
use gl::types::GLuint;

use crate::config::CONFIG;

use super::{framebuffer::Framebuffer, gizmo::RenderGizmo, model::Model, shader::Shader};

/// Struct that handles `position`, `rotation` and `scale` for an entity
#[derive(Debug)]
pub struct Transform {
    pub position: Point3<f32>,
    pub scale: Vector3<f32>,
    rotation: Euler<f32>,
    forward: Vector3<f32>,
    up: Vector3<f32>,
    right: Vector3<f32>,
    pub vao: GLuint,
    shader: Shader,
    // TODO: This is kind of ugly
    view_map_shader: Shader,
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
            vao: 0,                 // Initialized later
            shader: Shader::with_geometry_shader(
                "assets/shaders/debug/cubicGizmo.vert.glsl",
                "assets/shaders/debug/cubicGizmo.frag.glsl",
                "assets/shaders/debug/cubicGizmo.geom.glsl",
            ),
            view_map_shader: Shader::new_single("assets/shaders/octree/viewMap.glsl"),
        };
        this.update_vectors();
        unsafe {
            this.setup_vao();
        };
        this
    }
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

    pub fn set_rotation_x(&mut self, x: f32) {
        self.rotation.x = x;
        self.update_vectors();
    }

    pub fn set_rotation_y(&mut self, y: f32) {
        self.rotation.y = y;
        self.update_vectors();
    }

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

    unsafe fn setup_vao(&mut self) {
        gl::GenVertexArrays(1, &mut self.vao);
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let size = size_of::<Point3<f32>>() as isize;
        let data = &[self.position][0] as *const Point3<f32> as *const c_void;
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

    /// Creates textures from its POV of `models`.
    /// First two hold global positions, unnormalized and normalized respectively.
    /// The third holds normals.
    /// The fourth holds colors.
    pub unsafe fn take_photo(
        &self,
        models: &[&Model],
        projection: &Matrix4<f32>,
        model: &Matrix4<f32>,
        framebuffer: &Framebuffer,
        shader: Option<Shader>,
    ) -> (GLuint, GLuint, GLuint, GLuint) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        let shader = if let Some(shader) = shader {
            shader
        } else {
            self.view_map_shader
        };

        shader.use_program();
        shader.set_mat4(c_str!("projection"), &projection);
        shader.set_mat4(c_str!("view"), &self.get_view_matrix());
        shader.set_mat4(c_str!("model"), &model);
        shader.set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer.fbo());
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        for model in models {
            model.draw(&shader);
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        framebuffer.textures()
    }
}
