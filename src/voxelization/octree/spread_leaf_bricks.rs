use c_str_macro::c_str;
use gl::types::*;

use crate::{config::CONFIG, rendering::shader::Shader};

use super::common::{OCTREE_NODE_POOL, OCTREE_NODE_POOL_BRICK_POINTERS};

pub struct SpreadLeafBricksPass {
    shader: Shader,
    voxel_positions_texture: GLuint,
    number_of_voxel_fragments: GLuint,
}

impl SpreadLeafBricksPass {
    pub fn init(voxel_positions_texture: GLuint, number_of_voxel_fragments: u32) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/spread_leaf_bricks.comp.glsl"),
            voxel_positions_texture,
            number_of_voxel_fragments,
        }
    }

    pub unsafe fn run(&self, brick_pool_colors_texture: GLuint) {
        self.shader.use_program();

        self.shader
            .set_uint(c_str!("voxel_dimension"), CONFIG.voxel_dimension as u32);
        self.shader
            .set_uint(c_str!("octree_levels"), CONFIG.octree_levels - 1);
        self.shader.set_uint(
            c_str!("number_of_voxel_fragments"),
            self.number_of_voxel_fragments,
        );

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
            gl::TRUE,
            0,
            gl::READ_WRITE,
            gl::RGBA8,
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
