use std::{ffi::c_void, mem::size_of};

use c_str_macro::c_str;
use cgmath::{point3, vec3, Deg, Euler, InnerSpace, Matrix4, Point3, Vector3, Zero};
use gl::types::GLuint;

use crate::config::CONFIG;

use super::{gizmo::RenderGizmo, model::Model, shader::Shader};

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
    pub unsafe fn take_photo(
        &self,
        models: &[&Model],
        projection: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) -> (GLuint, GLuint, GLuint) {
        // TODO: Compile beforehand once we have dynamic lights/objects
        let shader = Shader::new_single("assets/shaders/octree/viewMap.glsl");

        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        shader.use_program();
        shader.set_mat4(c_str!("projection"), &projection);
        shader.set_mat4(c_str!("view"), &self.get_view_matrix());
        shader.set_mat4(c_str!("model"), &model);
        shader.set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        let mut fbo = 0;
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut textures = [0; 3]; // First one is rgb10_a2ui, second rgba8 for viewing, third is normals
        gl::GenTextures(3, textures.as_mut_ptr());

        gl::BindTexture(gl::TEXTURE_2D, textures[0]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB10_A2UI as i32,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
            0,
            gl::RGBA_INTEGER,
            gl::UNSIGNED_INT_2_10_10_10_REV,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::BindTexture(gl::TEXTURE_2D, textures[1]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::BindTexture(gl::TEXTURE_2D, textures[2]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB32F as i32,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
            0,
            gl::RGB,
            gl::FLOAT,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        let mut rbo = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
        );
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo,
        );
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            textures[0],
            0,
        );
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT1,
            gl::TEXTURE_2D,
            textures[1],
            0,
        );
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT2,
            gl::TEXTURE_2D,
            textures[2],
            0,
        );

        gl::DrawBuffers(
            3,
            [
                gl::COLOR_ATTACHMENT0,
                gl::COLOR_ATTACHMENT1,
                gl::COLOR_ATTACHMENT2,
            ]
            .as_ptr(),
        );

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER: Framebuffer is not complete!");
        }

        gl::Enable(gl::DEPTH_TEST);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        for model in models {
            model.draw(&shader);
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        (textures[0], textures[1], textures[2])
    }
}
