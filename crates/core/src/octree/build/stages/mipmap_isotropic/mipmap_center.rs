use c_str_macro::c_str;
use renderer::prelude::*;

use crate::{
    config::Config,
    octree::{NodeData, OctreeTextures},
};

pub struct MipmapCenterPass {
    shader: Shader,
}

impl MipmapCenterPass {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!(
                "assets/shaders/octree/isotropicMipMaps/mipmapCenter.comp.glsl",
            ),
        }
    }

    pub unsafe fn run(&self, textures: &OctreeTextures, node_data: &NodeData, level: u32) {
        self.shader.use_program();

        let config = Config::instance();

        self.shader.set_uint(c_str!("octreeLevel"), level);
        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

        helpers::bind_image_texture(0, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_3d_image_texture(1, textures.brick_pool_normals, gl::READ_WRITE, gl::RGBA32F);
        helpers::bind_image_texture(2, node_data.level_start_indices.0, gl::READ_ONLY, gl::R32UI);

        let nodes_in_level = node_data.nodes_per_level[level as usize];
        let groups_count = (nodes_in_level as f32 / config.working_group_size as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
