use c_str_macro::c_str;
use engine::prelude::*;

use crate::{
    config::Config,
    octree::{OctreeTextures, VoxelData, NodeData},
};

pub struct NeighborPointersPass {
    shader: Shader,
}

impl NeighborPointersPass {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/neighborPointers.comp.glsl"),
        }
    }

    pub unsafe fn run(
        &self,
        voxel_data: &VoxelData,
        node_data: &NodeData,
        textures: &OctreeTextures,
        current_octree_level: u32,
        level_start: u32,
        nodes_in_current_level: u32
    ) {
        self.shader.use_program();

        let config = Config::instance();

        // Set uniforms
        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension() as u32);
        self.shader
            .set_uint(c_str!("octreeLevel"), current_octree_level);
        self.shader
            .set_uint(c_str!("levelStart"), level_start);
        self.shader
            .set_uint(c_str!("nextLevelStart"), level_start + nodes_in_current_level);
        self.shader.set_int(c_str!("axis"), 0);

        // Bind images
        helpers::bind_image_texture(0, textures.node_pool.0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(1, voxel_data.voxel_positions.texture(), gl::WRITE_ONLY, gl::R32UI);

        helpers::bind_image_texture(2, textures.neighbors[0].0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(3, textures.neighbors[1].0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(4, textures.node_positions.0, gl::READ_ONLY, gl::RGB10_A2UI);

        let groups_count = (nodes_in_current_level as f32
            / config.working_group_size as f32)
            .ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();

        self.shader.set_int(c_str!("axis"), 1);
        helpers::bind_image_texture(2, textures.neighbors[2].0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(3, textures.neighbors[3].0, gl::WRITE_ONLY, gl::R32UI);

        self.shader.dispatch(groups_count);
        self.shader.wait();

        self.shader.set_int(c_str!("axis"), 2);
        helpers::bind_image_texture(2, textures.neighbors[4].0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(3, textures.neighbors[5].0, gl::WRITE_ONLY, gl::R32UI);

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
