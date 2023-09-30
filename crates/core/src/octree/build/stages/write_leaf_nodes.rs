use c_str_macro::c_str;
use renderer::prelude::*;

use crate::{
    config::Config,
    octree::{OctreeTextures, VoxelData},
};

pub struct WriteLeafNodesPass {
    shader: Shader,
}

impl WriteLeafNodesPass {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/writeLeafNodes.comp.glsl"),
        }
    }

    pub unsafe fn run(&self, voxel_data: &VoxelData, textures: &OctreeTextures) {
        self.shader.use_program();
        let config = Config::instance();
        let octree_level = config.last_octree_level();

        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.shader.set_uint(c_str!("octreeLevel"), octree_level); // Last level
        self.shader.set_uint(
            c_str!("numberOfVoxelFragments"),
            voxel_data.number_of_voxel_fragments,
        );

        helpers::bind_image_texture(
            0,
            voxel_data.voxel_positions.0,
            gl::READ_WRITE,
            gl::RGB10_A2UI,
        );
        helpers::bind_image_texture(1, voxel_data.voxel_colors.0, gl::READ_WRITE, gl::RGBA8);
        helpers::bind_3d_image_texture(
            2,
            textures.brick_pool_colors_raw,
            gl::READ_WRITE,
            gl::R32UI,
        );
        helpers::bind_image_texture(3, textures.node_pool.0, gl::READ_WRITE, gl::R32UI);
        helpers::bind_image_texture(4, voxel_data.voxel_normals.0, gl::READ_ONLY, gl::RGBA32F);
        helpers::bind_3d_image_texture(5, textures.brick_pool_normals, gl::WRITE_ONLY, gl::RGBA32F);

        self.shader.dispatch(
            (voxel_data.number_of_voxel_fragments as f32 / config.working_group_size as f32).ceil()
                as u32,
        );
        self.shader.wait();
    }
}
