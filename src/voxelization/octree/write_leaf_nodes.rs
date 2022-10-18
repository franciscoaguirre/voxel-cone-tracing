use c_str_macro::c_str;
use gl::types::*;

use crate::{
    constants::{OCTREE_LEVELS, VOXEL_DIMENSION},
    rendering::shader::Shader,
};

pub struct WriteLeafNodesPass {
    shader: Shader,
    voxel_positions_texture: GLuint,
    voxel_colors_texture: GLuint,
    node_pool_brick_pointers_texture: GLuint,
    node_pool_texture: GLuint,
}

impl WriteLeafNodesPass {
    pub fn init(
        voxel_positions_texture: GLuint,
        voxel_colors_texture: GLuint,
        node_pool_brick_pointers_texture: GLuint,
        node_pool_texture: GLuint,
    ) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/write_leaf_nodes.comp.glsl"),
            voxel_positions_texture,
            voxel_colors_texture,
            node_pool_brick_pointers_texture,
            node_pool_texture,
        }
    }

    pub unsafe fn run(&self, brick_pool_colors_texture: GLuint) {
        self.shader.use_program();

        self.shader
            .set_uint(c_str!("voxel_dimension"), VOXEL_DIMENSION as u32);
        self.shader
            .set_int(c_str!("max_octree_level"), OCTREE_LEVELS as i32);

        // Bind images
        gl::BindImageTexture(
            0,
            self.voxel_positions_texture,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::R32UI,
        );

        gl::BindImageTexture(
            1,
            self.voxel_colors_texture,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::R32UI,
        );

        gl::BindImageTexture(
            2,
            self.node_pool_brick_pointers_texture,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::R32UI,
        );

        gl::BindImageTexture(
            3,
            brick_pool_colors_texture,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::R32UI,
        );

        gl::BindImageTexture(
            4,
            self.node_pool_texture,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::R32UI,
        );

        self.shader.dispatch(65_535); // TODO: Calculate number of groups
        self.shader.wait();
    }
}
