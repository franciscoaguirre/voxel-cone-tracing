use std::{ffi::c_void, mem::size_of};

use c_str_macro::c_str;
use cgmath::{point3, Deg, Matrix4, Point3};
use gl::types::GLuint;
use serde::Deserialize;

use crate::{
    aabb::Aabb,
    common,
    framebuffer::{LightFramebuffer, LIGHT_MAP_BUFFERS},
    gizmo::RenderGizmo,
    object::Object,
    shader::{compile_shaders, Shader},
    transform::Transform,
    types::Textures,
};

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct SpotLight {
    pub transform: Transform,
    pub intensity: f32,
    width: f32,
    height: f32,
    color: Point3<f32>,
    #[serde(skip_deserializing, default = "gizmo_shader")]
    shader: Shader,
    #[serde(skip_deserializing)]
    vao: GLuint,
    #[serde(skip_deserializing)]
    light_map_shader: Shader,
    #[serde(skip_deserializing)]
    framebuffer: LightFramebuffer,
}

impl Default for SpotLight {
    fn default() -> Self {
        let mut light = unsafe {
            Self {
                width: 2.0,
                height: 2.0,
                transform: Transform::default(),
                intensity: 1_000_000.0,
                color: point3(1.0, 1.0, 1.0),
                vao: 0,
                shader: gizmo_shader(),
                light_map_shader: light_map_shader(),
                framebuffer: LightFramebuffer::new_directional(),
            }
        };
        unsafe {
            light.setup_vao();
        }
        light
    }
}

impl SpotLight {
    pub unsafe fn new(width: f32, height: f32, color: Point3<f32>, intensity: f32) -> Self {
        let light = Self {
            width,
            height,
            color,
            intensity,
            ..Default::default()
        };
        light
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        cgmath::ortho(
            -self.width / 2.0,
            self.width / 2.0,
            -self.height / 2.0,
            self.height / 2.0,
            0.0,
            2.0,
        )
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

    pub unsafe fn take_photo(
        &self,
        objects: &mut [Object],
        scene_aabb: &Aabb,
        voxel_dimension: u32, // TODO: Find another way. This breaks separation of concerns
    ) -> Textures<LIGHT_MAP_BUFFERS> {
        let projection = self.get_projection_matrix();
        self.light_map_shader.use_program();
        self.light_map_shader
            .set_mat4(c_str!("projection"), &projection);
        self.light_map_shader
            .set_mat4(c_str!("view"), &self.transform.get_view_matrix());
        self.light_map_shader
            .set_uint(c_str!("voxelDimension"), voxel_dimension);
        self.light_map_shader.set_vec3(
            c_str!("lightPosition"),
            self.transform.position.x,
            self.transform.position.y,
            self.transform.position.z,
        );

        gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.fbo());
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        for object in objects.iter_mut() {
            object.draw(&self.light_map_shader, &scene_aabb.normalization_matrix());
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        self.framebuffer.textures()
    }
}

fn gizmo_shader() -> Shader {
    compile_shaders!(
        "assets/shaders/debug/gizmo.vert.glsl",
        "assets/shaders/debug/gizmo.frag.glsl",
        "assets/shaders/debug/gizmo.geom.glsl",
    )
}

fn light_map_shader() -> Shader {
    compile_shaders!("assets/shaders/octree/lightViewMapDirectional.glsl",)
}

impl RenderGizmo for SpotLight {
    unsafe fn draw_gizmo(&self, projection: &Matrix4<f32>, view: &Matrix4<f32>) {
        self.shader.use_program();

        self.shader.set_mat4(c_str!("projection"), projection);
        self.shader.set_mat4(c_str!("view"), view);
        self.shader
            .set_mat4(c_str!("model"), &self.transform.get_model_matrix());

        self.shader
            .set_vec3(c_str!("color"), self.color.x, self.color.y, self.color.z);
        self.shader.set_vec3(
            c_str!("dimensions"),
            self.width / 2.0,
            self.height / 2.0,
            0.01,
        );

        gl::BindVertexArray(self.vao);
        gl::DrawArrays(gl::POINTS, 0, 1);
        gl::BindVertexArray(0);
    }
}
