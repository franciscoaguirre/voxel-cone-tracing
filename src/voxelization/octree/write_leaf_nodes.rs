use c_str_macro::c_str;
use gl::types::*;

use crate::{config::CONFIG, constants::WORKING_GROUP_SIZE, rendering::shader::Shader};

use super::common::{BRICK_POOL_COLORS_TEXTURE, OCTREE_NODE_POOL, OCTREE_NODE_POOL_BRICK_POINTERS};

pub struct WriteLeafNodesPass {
    shader: Shader,
    voxel_positions_texture: GLuint,
    voxel_colors_texture: GLuint,
    number_of_voxel_fragments: u32,
}

impl WriteLeafNodesPass {
    pub fn init(
        voxel_positions_texture: GLuint,
        voxel_colors_texture: GLuint,
        number_of_voxel_fragments: u32,
    ) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/write_leaf_nodes.comp.glsl"),
            voxel_positions_texture,
            voxel_colors_texture,
            number_of_voxel_fragments,
        }
    }

    pub unsafe fn run(&self) {
        self.shader.use_program();

        self.shader
            .set_uint(c_str!("voxel_dimension"), CONFIG.voxel_dimension);
        self.shader
            .set_uint(c_str!("max_octree_level"), CONFIG.octree_levels - 1); // Last level

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
            BRICK_POOL_COLORS_TEXTURE,
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

        let groups_count =
            (self.number_of_voxel_fragments as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
