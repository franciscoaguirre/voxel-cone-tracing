use c_str_macro::c_str;
use gl::types::*;

use crate::config::CONFIG;
use crate::constants::WORKING_GROUP_SIZE;
use crate::rendering::shader::Shader;

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
            c_str!("number_of_voxel_fragments"),
            self.number_of_voxel_fragments,
        );
        self.shader.set_uint(c_str!("octree_level"), octree_level);
        self.shader
            .set_uint(c_str!("voxel_dimension"), CONFIG.voxel_dimension);

        gl::BindImageTexture(
            0,
            self.voxel_position_texture,
            0,
            gl::FALSE,
            0,
            gl::READ_ONLY,
            gl::RGB10_A2UI,
        );
        gl::BindImageTexture(
            1,
            OCTREE_NODE_POOL.0,
            0,
            gl::FALSE,
            0,
            gl::WRITE_ONLY,
            gl::R32UI,
        );

        let groups_count =
            (self.number_of_voxel_fragments as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
