use c_str_macro::c_str;
use gl::types::*;

use crate::config::CONFIG;
use crate::constants::WORKING_GROUP_SIZE;
use crate::rendering::shader::Shader;
use crate::voxelization::helpers;

use super::common::OCTREE_NODE_POOL;

pub struct FlagNodesPass {
    shader: Shader,
    number_of_voxel_fragments: u32,
    voxel_position_texture: GLuint,
}

impl FlagNodesPass {
    pub fn init(number_of_voxel_fragments: u32, voxel_position_texture: GLuint) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/flagNodes.comp.glsl"),
            number_of_voxel_fragments,
            voxel_position_texture,
        }
    }

    pub unsafe fn run(&self, octree_level: u32) {
        self.shader.use_program();

        self.shader.set_uint(
            c_str!("numberOfVoxelFragments"),
            self.number_of_voxel_fragments,
        );
        self.shader.set_uint(c_str!("octreeLevel"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        helpers::bind_image_texture(0, self.voxel_position_texture, gl::READ_ONLY, gl::RGB10_A2);
        helpers::bind_image_texture(1, OCTREE_NODE_POOL.0, gl::READ_WRITE, gl::R32UI);

        let groups_count =
            (self.number_of_voxel_fragments as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
