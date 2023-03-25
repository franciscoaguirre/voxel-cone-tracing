use c_str_macro::c_str;

use crate::{
    constants::WORKING_GROUP_SIZE, helpers, octree::OctreeTextures, rendering::shader::Shader,
};

pub struct MipmapCenterPass {
    shader: Shader,
}

impl MipmapCenterPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/mipmapCenter.comp.glsl"),
        }
    }

    pub unsafe fn run(&self, textures: &OctreeTextures, nodes_per_level: &[u32], level: u32) {
        self.shader.use_program();

        self.shader.set_uint(c_str!("octreeLevel"), level);

        helpers::bind_image_texture(0, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(1, textures.brick_pointers.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_3d_image_texture(2, textures.brick_pool, gl::READ_WRITE, gl::RGBA8);
        helpers::bind_image_texture(3, textures.level_start_indices.0, gl::READ_ONLY, gl::R32UI);

        let nodes_in_level = nodes_per_level[level as usize];
        let groups_count = (nodes_in_level as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
