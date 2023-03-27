use std::{ffi::c_void, mem::size_of};

use gl::types::GLuint;

use super::shader::Shader;

pub struct Quad {
    vao: GLuint,
    indices: Vec<u32>,
    shader: Shader,
}

impl Quad {
    pub unsafe fn new() -> Self {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let vertices: [f32; 20] = [
            1.0, 0.0, 0.0, 1.0, 1.0, // Top right
            1.0, -1.0, 0.0, 1.0, 0.0, // Bottom right
            0.0, -1.0, 0.0, 0.0, 0.0, // Bottom left
            0.0, 0.0, 0.0, 0.0, 1.0, // Top left
        ];
        gl::BufferData(
            gl::ARRAY_BUFFER,
            vertices.len() as isize * size_of::<f32>() as isize,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        let mut ebo = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            indices.len() as isize * size_of::<u32>() as isize,
            &indices[0] as *const u32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            5 * size_of::<f32>() as i32,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as *const c_void,
        );

        let shader = Shader::new(
            "assets/shaders/renderQuad.vert.glsl",
            "assets/shaders/renderQuad.frag.glsl",
        );

        Self {
            vao,
            indices: indices.to_vec(),
            shader,
        }
    }

    pub unsafe fn render(&self, texture: GLuint) {
        self.shader.use_program();

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::BindVertexArray(self.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        gl::BindVertexArray(0);
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
}
