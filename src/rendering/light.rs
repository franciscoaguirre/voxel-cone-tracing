use std::{ffi::c_void, mem::size_of};

use c_str_macro::c_str;
use cgmath::{Matrix4, Vector3};
use gl::types::GLuint;

use super::{gizmo::RenderGizmo, shader::Shader};

#[derive(Debug)]
pub struct PointLight {
    position: Vector3<f32>,
    color: Vector3<f32>,
    shader: Shader,
    vao: GLuint,
}

impl PointLight {
    pub unsafe fn new(position: Vector3<f32>, color: Vector3<f32>) -> Self {
        let mut light = Self {
            position,
            color,
            vao: 0,
            shader: Shader::with_geometry_shader(
                "assets/shaders/debug/gizmo.vert.glsl",
                "assets/shaders/debug/gizmo.frag.glsl",
                "assets/shaders/debug/gizmo.geom.glsl",
            ),
        };
        light.setup_vao();
        light
    }

    unsafe fn setup_vao(&mut self) {
        gl::GenVertexArrays(1, &mut self.vao);
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let size = (2 * size_of::<Vector3<f32>>()) as isize;
        let data = &[self.position, self.color][0] as *const Vector3<f32> as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vector3<f32>>() as i32,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vector3<f32>>() as i32,
            (size_of::<Vector3<f32>>() as isize) as *const c_void,
        );
        gl::BindVertexArray(0);
        // TODO: We wanna draw a cube and we already have a GLSL helper for drawing cubes.
        // However, we should start doing that on the CPU once instead of on the GPU every frame
        // in the geometry shader. Will speed things up a lot.
        // Not that relevant since it's debugging code.
    }
}

impl RenderGizmo for PointLight {
    unsafe fn draw_gizmo(
        &self,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        self.shader.use_program();

        self.shader.set_mat4(c_str!("projection"), projection);
        self.shader.set_mat4(c_str!("view"), view);
        self.shader.set_mat4(c_str!("model"), model);

        gl::BindVertexArray(self.vao);
        gl::DrawArrays(gl::POINTS, 0, 1);
        gl::BindVertexArray(0);
    }
}
