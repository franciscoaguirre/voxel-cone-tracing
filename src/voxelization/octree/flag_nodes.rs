use c_str_macro::c_str;
use gl::types::*;

use crate::constants::VOXEL_DIMENSION;
use crate::rendering::shader::Shader;

pub struct FlagNodesPass {
    shader: Shader,
    number_of_voxel_fragments: u32,
    voxel_position_texture: GLuint,
    octree_node_pool_texture: GLuint,
}

impl FlagNodesPass {
    pub fn init(
        number_of_voxel_fragments: u32,
        voxel_position_texture: GLuint,
        octree_node_pool_texture: GLuint,
    ) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/flag_nodes.comp.glsl"),
            number_of_voxel_fragments,
            voxel_position_texture,
            octree_node_pool_texture,
        }
    }

    pub unsafe fn run(&self, octree_level: u32) {
        self.shader.use_program();

        self.shader.set_int(
            c_str!("number_of_voxel_fragments"),
            self.number_of_voxel_fragments as i32,
        );
        self.shader
            .set_int(c_str!("octree_level"), octree_level as i32);
        self.shader
            .set_int(c_str!("voxel_dimension"), VOXEL_DIMENSION);

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
            self.octree_node_pool_texture,
            0,
            gl::FALSE,
            0,
            gl::WRITE_ONLY,
            gl::R32UI,
        );

        self.shader.dispatch(65_535); // TODO: Calculate number of groups
        self.shader.wait();
    }
}
