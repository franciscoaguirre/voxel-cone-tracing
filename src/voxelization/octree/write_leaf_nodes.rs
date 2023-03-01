use c_str_macro::c_str;
use gl::types::*;

use crate::{config::CONFIG, constants::{CHILDREN_PER_NODE, WORKING_GROUP_SIZE}, rendering::shader::Shader};

use super::common::{BRICK_POOL_COLORS_TEXTURE, OCTREE_NODE_POOL, OCTREE_NODE_POOL_BRICK_POINTERS, NODES_PER_LEVEL};

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
            shader: Shader::new_compute("assets/shaders/octree/writeLeafNodes.comp.glsl"),
            voxel_positions_texture,
            voxel_colors_texture,
            number_of_voxel_fragments,
        }
    }

    pub unsafe fn run(&self) {
        self.shader.use_program();
        let octree_level = CONFIG.octree_levels - 1;

        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.shader
            .set_uint(c_str!("octreeLevel"), octree_level); // Last level
        self.shader
            .set_uint(c_str!("number_of_voxel_fragments"), self.number_of_voxel_fragments);

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

        let tiles_in_level = NODES_PER_LEVEL[octree_level as usize];
        let nodes_in_level = tiles_in_level * CHILDREN_PER_NODE;
        let groups_count = (nodes_in_level as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        dbg!(groups_count);
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
