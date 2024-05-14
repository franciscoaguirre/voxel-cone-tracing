use std::{ffi::c_void, mem::size_of};

use c_str_macro::c_str;
use cgmath::{point3, vec3, Deg, Matrix4, Point3};
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
pub struct PointLight {
    pub transform: Transform,
    pub intensity: f32,
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

const FAR_PLANE: f32 = 1_000_000.0;

impl Default for PointLight {
    fn default() -> Self {
        let mut light = unsafe {
            Self {
                transform: Transform::default(),
                intensity: 1_000_000.0,
                color: point3(1.0, 1.0, 1.0),
                vao: 0,
                shader: gizmo_shader(),
                light_map_shader: light_map_shader(),
                framebuffer: LightFramebuffer::new(),
            }
        };
        unsafe {
            light.setup_vao();
        }
        light
    }
}

fn gizmo_shader() -> Shader {
    compile_shaders!(
        "assets/shaders/debug/cubicGizmo.vert.glsl",
        "assets/shaders/debug/cubicGizmo.frag.glsl",
        "assets/shaders/debug/cubicGizmo.geom.glsl",
    )
}

fn light_map_shader() -> Shader {
    compile_shaders!("assets/shaders/octree/lightViewMapPoint.glsl",)
}

impl PointLight {
    pub unsafe fn new(color: Point3<f32>, intensity: f32) -> Self {
        let light = Self {
            intensity,
            color,
            ..Default::default()
        };
        light
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        let (width, height) = unsafe { common::get_framebuffer_size() };

        cgmath::perspective(Deg(90.0), width as f32 / height as f32, 0.0001, FAR_PLANE)
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
        let shadow_matrices = vec![
            projection
                * Matrix4::look_at_rh(
                    self.transform.position,
                    self.transform.position + vec3(1.0, 0.0, 0.0),
                    vec3(0.0, 1.0, 0.0),
                ),
            projection
                * Matrix4::look_at_rh(
                    self.transform.position,
                    self.transform.position + vec3(-1.0, 0.0, 0.0),
                    vec3(0.0, 1.0, 0.0),
                ),
            projection
                * Matrix4::look_at_rh(
                    self.transform.position,
                    self.transform.position + vec3(0.0, 1.0, 0.0),
                    vec3(0.0, 0.0, 1.0),
                ),
            projection
                * Matrix4::look_at_rh(
                    self.transform.position,
                    self.transform.position + vec3(0.0, -1.0, 0.0),
                    vec3(0.0, 0.0, 1.0),
                ),
            projection
                * Matrix4::look_at_rh(
                    self.transform.position,
                    self.transform.position + vec3(0.0, 0.0, 1.0),
                    vec3(0.0, 1.0, 0.0),
                ),
            projection
                * Matrix4::look_at_rh(
                    self.transform.position,
                    self.transform.position + vec3(0.0, 0.0, -1.0),
                    vec3(0.0, 1.0, 0.0),
                ),
        ];
        self.light_map_shader.set_mat4_array(
            c_str!("shadowMatrices"),
            shadow_matrices
                .iter()
                .collect::<Vec<&Matrix4<f32>>>()
                .as_slice(),
        );
        self.light_map_shader
            .set_uint(c_str!("voxelDimension"), voxel_dimension);
        self.light_map_shader.set_vec3(
            c_str!("lightPosition"),
            self.transform.position.x,
            self.transform.position.y,
            self.transform.position.z,
        );
        self.light_map_shader
            .set_float(c_str!("farPlane"), FAR_PLANE);

        gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.fbo());
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        // TODO: Move to Kernel.
        // for object in objects.iter_mut() {
        //     object.draw(&self.light_map_shader, &scene_aabb.normalization_matrix());
        // }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        [
            self.framebuffer.textures()[0].1,
            self.framebuffer.textures()[1].1,
            self.framebuffer.textures()[2].1,
        ]
    }
}

impl RenderGizmo for PointLight {
    unsafe fn draw_gizmo(&self, projection: &Matrix4<f32>, view: &Matrix4<f32>) {
        self.shader.use_program();

        self.shader.set_mat4(c_str!("projection"), projection);
        self.shader.set_mat4(c_str!("view"), view);
        self.shader
            .set_mat4(c_str!("model"), &self.transform.get_model_matrix());

        self.shader
            .set_vec3(c_str!("color"), self.color.x, self.color.y, self.color.z);

        gl::BindVertexArray(self.vao);
        gl::DrawArrays(gl::POINTS, 0, 1);
        gl::BindVertexArray(0);
    }
}
