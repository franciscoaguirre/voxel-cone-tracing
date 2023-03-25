use c_str_macro::c_str;

use crate::{
    config::CONFIG, constants::WORKING_GROUP_SIZE, helpers, octree::OctreeTextures,
    rendering::shader::Shader,
};

pub struct SpreadLeafBricksPass {
    shader: Shader,
}

impl SpreadLeafBricksPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/spreadLeafBricks.comp.glsl"),
        }
    }

    pub unsafe fn run(&self, textures: &OctreeTextures, nodes_per_level: &[u32]) {
        self.shader.use_program();

        let octree_level = CONFIG.octree_levels - 1;
        self.shader.set_uint(c_str!("octreeLevel"), octree_level);

        helpers::bind_image_texture(0, textures.brick_pointers.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_3d_image_texture(1, textures.brick_pool, gl::READ_WRITE, gl::RGBA8);
        helpers::bind_image_texture(2, textures.level_start_indices.0, gl::READ_ONLY, gl::R32UI);

        let nodes_in_level = nodes_per_level[octree_level as usize];
        let groups_count = (nodes_in_level as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
