use c_str_macro::c_str;

use super::super::super::{OctreeTextures, VoxelData};
use crate::{config::CONFIG, constants::WORKING_GROUP_SIZE, helpers, rendering::shader::Shader};

pub struct FlagNodesPass {
    shader: Shader,
}

impl FlagNodesPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/flagNodes.comp.glsl"),
        }
    }

    pub unsafe fn run(&self, voxel_data: &VoxelData, textures: &OctreeTextures, octree_level: u32) {
        self.shader.use_program();

        self.shader.set_uint(
            c_str!("numberOfVoxelFragments"),
            voxel_data.number_of_voxel_fragments,
        );
        self.shader.set_uint(c_str!("octreeLevel"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        helpers::bind_image_texture(0, voxel_data.voxel_positions, gl::READ_ONLY, gl::RGB10_A2);
        helpers::bind_image_texture(1, textures.node_pool.0, gl::READ_WRITE, gl::R32UI);

        let groups_count =
            (voxel_data.number_of_voxel_fragments as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
