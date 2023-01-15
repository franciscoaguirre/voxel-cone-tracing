use c_str_macro::c_str;
use gl::types::*;
use log::debug;

use crate::{
    config::CONFIG,
    constants::WORKING_GROUP_SIZE,
    rendering::shader::Shader,
    voxelization::{helpers, octree},
};

use super::common::{
    OCTREE_NODE_POOL, OCTREE_NODE_POOL_NEIGHBOUR_X, OCTREE_NODE_POOL_NEIGHBOUR_X_NEGATIVE,
    OCTREE_NODE_POOL_NEIGHBOUR_Y, OCTREE_NODE_POOL_NEIGHBOUR_Y_NEGATIVE,
    OCTREE_NODE_POOL_NEIGHBOUR_Z, OCTREE_NODE_POOL_NEIGHBOUR_Z_NEGATIVE,
};

pub struct NeighbourPointersPass {
    shader: Shader,
    voxel_positions_texture: GLuint,
    number_of_voxel_fragments: u32,
}

impl NeighbourPointersPass {
    pub fn init(voxel_positions_texture: GLuint, number_of_voxel_fragments: u32) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/neighbour_pointers.comp.glsl"),
            voxel_positions_texture,
            number_of_voxel_fragments,
        }
    }

    pub unsafe fn run(&self, current_octree_level: u32) {
        self.shader.use_program();

        // Set uniforms
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension as u32);
        self.shader
            .set_uint(c_str!("octreeLevel"), current_octree_level);

        // Bind images
        helpers::bind_image_texture(0, OCTREE_NODE_POOL.0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(1, self.voxel_positions_texture, gl::WRITE_ONLY, gl::R32UI);

        helpers::bind_image_texture(2, OCTREE_NODE_POOL_NEIGHBOUR_X.0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(
            3,
            OCTREE_NODE_POOL_NEIGHBOUR_X_NEGATIVE.0,
            gl::WRITE_ONLY,
            gl::R32UI,
        );
        helpers::bind_image_texture(4, OCTREE_NODE_POOL_NEIGHBOUR_Y.0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(
            5,
            OCTREE_NODE_POOL_NEIGHBOUR_Y_NEGATIVE.0,
            gl::WRITE_ONLY,
            gl::R32UI,
        );
        helpers::bind_image_texture(6, OCTREE_NODE_POOL_NEIGHBOUR_Z.0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(
            7,
            OCTREE_NODE_POOL_NEIGHBOUR_Z_NEGATIVE.0,
            gl::WRITE_ONLY,
            gl::R32UI,
        );

        let groups_count =
            (self.number_of_voxel_fragments as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
