use std::{ffi::c_void, mem};

use c_str_macro::c_str;
use gl::types::GLuint;

use crate::camera::Camera;

use super::shader::Shader;

/// Cube ready to be rendered.
pub struct Cube {
    /// `Vertex Array Object`, gets populated in `new` with all 8 vertices that make up a cube.
    vao: GLuint,
    /// Indices for `vao`, they exist to reuse vertices.
    indices: Vec<u32>,
}

impl Cube {
    pub unsafe fn new() -> Self {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // let vertices: [f32; 8 * 3] = [
        //     // Near
        //     1.0, 1.0, 1.0, // Right top
        //     1.0, -1.0, 1.0, // Right bottom
        //     -1.0, -1.0, 1.0, // Left bottom
        //     -1.0, 1.0, 1.0, // Left top
        //     // Far
        //     1.0, 1.0, -1.0, // Right top
        //     1.0, -1.0, -1.0, // Right bottom
        //     -1.0, -1.0, -1.0, // Left bottom
        //     -1.0, 1.0, -1.0, // Left top
        // ];

        let vertices: [f32; 36 * 3] = [
            -1.0, -1.0, -1.0, // something
            1.0, -1.0, -1.0, // something
            1.0, 1.0, -1.0, // something
            1.0, 1.0, -1.0, // something
            -1.0, 1.0, -1.0, // something
            -1.0, -1.0, -1.0, // something
            -1.0, -1.0, 1.0, // something
            1.0, -1.0, 1.0, // something
            1.0, 1.0, 1.0, // something
            1.0, 1.0, 1.0, // something
            -1.0, 1.0, 1.0, // something
            -1.0, -1.0, 1.0, // something
            -1.0, 1.0, 1.0, // something
            -1.0, 1.0, -1.0, // something
            -1.0, -1.0, -1.0, // something
            -1.0, -1.0, -1.0, // something
            -1.0, -1.0, 1.0, // something
            -1.0, 1.0, 1.0, // something
            1.0, 1.0, 1.0, // something
            1.0, 1.0, -1.0, // something
            1.0, -1.0, -1.0, // something
            1.0, -1.0, -1.0, // something
            1.0, -1.0, 1.0, // something
            1.0, 1.0, 1.0, // something
            -1.0, -1.0, -1.0, // something
            1.0, -1.0, -1.0, // something
            1.0, -1.0, 1.0, // something
            1.0, -1.0, 1.0, // something
            -1.0, -1.0, 1.0, // something
            -1.0, -1.0, -1.0, // something
            -1.0, 1.0, -1.0, // something
            1.0, 1.0, -1.0, // something
            1.0, 1.0, 1.0, // something
            1.0, 1.0, 1.0, // something
            -1.0, 1.0, 1.0, // something
            -1.0, 1.0, -1.0, // something
        ];
        gl::BufferData(
            gl::ARRAY_BUFFER,
            vertices.len() as isize * mem::size_of::<f32>() as isize,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        let mut ebo = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        let indices: [u32; 36] = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
        ];
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            indices.len() as isize * mem::size_of::<u32>() as isize,
            &indices[0] as *const u32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<f32>() as i32,
            0 as *const c_void,
        );

        gl::BindVertexArray(0);

        Self {
            vao,
            indices: indices.to_vec(),
        }
    }

    pub unsafe fn get_vao(&self) -> GLuint {
        self.vao
    }

    pub unsafe fn get_num_indices(&self) -> usize {
        self.indices.len()
    }

    pub unsafe fn render(&self, shader: &Shader, camera: &Camera) {
        shader.use_program();
        shader.set_mat4(c_str!("projection"), &camera.get_projection_matrix());
        shader.set_mat4(c_str!("view"), &camera.transform.get_view_matrix());

        gl::BindVertexArray(self.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        gl::BindVertexArray(0);
    }
}
