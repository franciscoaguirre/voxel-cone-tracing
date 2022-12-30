use c_str_macro::c_str;
use gl::types::*;

use crate::{config::CONFIG, rendering::shader::Shader};

use super::common::{OCTREE_NODE_POOL, OCTREE_NODE_POOL_BRICK_POINTERS};

pub struct WriteLeafNodesPass {
    shader: Shader,
    voxel_positions_texture: GLuint,
    voxel_colors_texture: GLuint,
}

impl WriteLeafNodesPass {
    pub fn init(voxel_positions_texture: GLuint, voxel_colors_texture: GLuint) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/write_leaf_nodes.comp.glsl"),
            voxel_positions_texture,
            voxel_colors_texture,
        }
    }

    pub unsafe fn run(&self, brick_pool_colors_texture: GLuint) {
        self.shader.use_program();

        self.shader
            .set_uint(c_str!("voxel_dimension"), CONFIG.voxel_dimension as u32);
        self.shader.set_int(
            c_str!("max_octree_level"),
            (CONFIG.octree_levels - 1) as i32,
        );

        // Bind images
        gl::BindImageTexture(
            0,
            self.voxel_positions_texture,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::RGB10_A2UI,
        );

        gl::BindImageTexture(
            1,
            self.voxel_colors_texture,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::RGBA8,
        );

        gl::BindImageTexture(
            2,
            OCTREE_NODE_POOL_BRICK_POINTERS.0,
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
            gl::TRUE,
            0,
            gl::WRITE_ONLY,
            gl::RGBA8,
        );

        gl::BindImageTexture(
            4,
            OCTREE_NODE_POOL.0,
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
