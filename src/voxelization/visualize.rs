use c_str_macro::c_str;
use cgmath::Matrix4;
use gl::types::*;

use crate::config::CONFIG;
use crate::rendering::shader::Shader;

pub struct RenderVoxelFragmentsShader {
    shader: Shader,
    voxel_positions_texture: GLuint,
    voxel_colors_texture: GLuint,
    number_of_voxel_fragments: u32,
    vao: GLuint,
}

impl RenderVoxelFragmentsShader {
    pub fn init(
        voxel_positions_texture: GLuint,
        voxel_colors_texture: GLuint,
        number_of_voxel_fragments: u32,
    ) -> Self {
        Self {
            shader: compile_shaders!(
                "assets/shaders/voxel_fragment/renderVoxel.vert.glsl",
                "assets/shaders/voxel_fragment/renderVoxel.frag.glsl",
                "assets/shaders/voxel_fragment/renderVoxel.geom.glsl",
            ),
            voxel_positions_texture,
            voxel_colors_texture,
            number_of_voxel_fragments,
            vao: unsafe {
                let mut vao = 0;
                gl::GenVertexArrays(1, &mut vao);

                vao
            },
        }
    }

    pub unsafe fn run(&self, projection: &Matrix4<f32>, view: &Matrix4<f32>, model: &Matrix4<f32>) {
        gl::Enable(gl::DEPTH_TEST);

        self.shader.use_program();

        gl::BindImageTexture(
            0,
            self.voxel_positions_texture,
            0,
            gl::TRUE,
            0,
            gl::READ_ONLY,
            gl::RGB10_A2UI,
        );

        gl::BindImageTexture(
            1,
            self.voxel_colors_texture,
            0,
            gl::TRUE,
            0,
            gl::READ_ONLY,
            gl::RGBA8,
        );

        self.shader.set_mat4(c_str!("projection"), projection);
        self.shader.set_mat4(c_str!("view"), view);
        self.shader.set_mat4(c_str!("model"), model);

        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.shader
            .set_float(c_str!("halfDimension"), 1.0 / CONFIG.voxel_dimension as f32);

        gl::BindVertexArray(self.vao);
        gl::DrawArrays(gl::POINTS, 0, self.number_of_voxel_fragments as i32);
    }
}
