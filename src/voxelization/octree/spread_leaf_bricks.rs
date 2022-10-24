use c_str_macro::c_str;
use gl::types::*;

use crate::{
    constants::{OCTREE_LEVELS, VOXEL_DIMENSION},
    rendering::shader::Shader,
};

pub struct SpreadLeafBricksPass {
    shader: Shader,
    node_pool_brick_pointers_texture: GLuint,
    node_pool_texture: GLuint,
    voxel_positions_texture: GLuint,
}

impl SpreadLeafBricksPass {
    pub fn init(
        node_pool_brick_pointers_texture: GLuint,
        node_pool_texture: GLuint,
        voxel_positions_texture: GLuint,
    ) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/spread_leaf_bricks.comp.glsl"),
            node_pool_brick_pointers_texture,
            node_pool_texture,
            voxel_positions_texture,
        }
    }

    pub unsafe fn run(&self, brick_pool_colors_texture: GLuint) {
        self.shader.use_program();

        self.shader
            .set_uint(c_str!("voxel_dimension"), VOXEL_DIMENSION as u32);
        self.shader
            .set_int(c_str!("octree_levels"), OCTREE_LEVELS as i32);

        gl::BindImageTexture(
            0,
            self.node_pool_brick_pointers_texture,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::R32UI,
        );

        gl::BindImageTexture(
            1,
            brick_pool_colors_texture,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::R32UI,
        );

        gl::BindImageTexture(
            2,
            self.node_pool_texture,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::R32UI,
        );

        gl::BindImageTexture(
            3,
            self.voxel_positions_texture,
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
