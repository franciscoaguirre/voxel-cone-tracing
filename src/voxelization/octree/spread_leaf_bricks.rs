use c_str_macro::c_str;
use gl::types::*;

use crate::{
    constants::{OCTREE_LEVELS, VOXEL_DIMENSION},
    rendering::shader::Shader,
};

use super::common::{OCTREE_NODE_POOL, OCTREE_NODE_POOL_BRICK_POINTERS};

pub struct SpreadLeafBricksPass {
    shader: Shader,
    voxel_positions_texture: GLuint,
}

impl SpreadLeafBricksPass {
    pub fn init(voxel_positions_texture: GLuint) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/spread_leaf_bricks.comp.glsl"),
            voxel_positions_texture,
        }
    }

    pub unsafe fn run(&self, brick_pool_colors_texture: GLuint) {
        self.shader.use_program();

        self.shader
            .set_uint(c_str!("voxel_dimension"), VOXEL_DIMENSION as u32);
        self.shader.set_uint(c_str!("octree_levels"), OCTREE_LEVELS);

        gl::BindImageTexture(
            0,
            OCTREE_NODE_POOL_BRICK_POINTERS.0,
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
            OCTREE_NODE_POOL.0,
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
