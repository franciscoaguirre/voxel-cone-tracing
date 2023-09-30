use c_str_macro::c_str;
use renderer::prelude::*;

use crate::{
    config::Config,
    octree::{OctreeTextures, VoxelData},
};

pub struct StoreNodePositions {
    shader: Shader,
}

impl StoreNodePositions {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/storeNodePositions.comp.glsl"),
        }
    }

    pub unsafe fn run(&self, textures: &OctreeTextures, octree_level: u32, voxel_data: &VoxelData) {
        self.shader.use_program();

        let config = Config::instance();

        self.shader.set_uint(c_str!("octreeLevel"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

        helpers::bind_image_texture(
            0,
            voxel_data.voxel_positions.0,
            gl::READ_ONLY,
            gl::RGB10_A2UI,
        );
        helpers::bind_image_texture(1, textures.node_positions.0, gl::WRITE_ONLY, gl::RGB10_A2UI);
        helpers::bind_image_texture(2, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);

        let groups_count = (voxel_data.number_of_voxel_fragments as f32
            / config.working_group_size as f32)
            .ceil() as u32;
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
