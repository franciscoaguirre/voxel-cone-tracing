use c_str_macro::c_str;
use engine::prelude::*;

use crate::{
    config::Config,
    octree::{NodeData, OctreeTextures},
};

pub struct ProcessRawBrickPoolColors {
    shader: Shader,
}

impl ProcessRawBrickPoolColors {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/processRawBrickPoolColors.comp.glsl"),
        }
    }

    pub unsafe fn run(&self, node_data: &NodeData, textures: &OctreeTextures) {
        let config = Config::instance();
        
        self.shader.use_program();
        self.shader
            .set_uint(c_str!("octreeLevel"), config.last_octree_level());

        helpers::bind_image_texture(0, node_data.level_start_indices.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_3d_image_texture(1, textures.brick_pool_colors_raw, gl::READ_ONLY, gl::R32UI);
        helpers::bind_3d_image_texture(2, textures.brick_pool_colors[0], gl::WRITE_ONLY, gl::RGBA8);

        let nodes_in_level = node_data.nodes_per_level[config.last_octree_level() as usize];
        let groups_count = (nodes_in_level as f32 / config.working_group_size as f32).ceil() as u32;
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
