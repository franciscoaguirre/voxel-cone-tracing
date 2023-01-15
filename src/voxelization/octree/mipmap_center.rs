use c_str_macro::c_str;

use crate::{
    constants::{NODES_PER_TILE, WORKING_GROUP_SIZE},
    rendering::shader::Shader,
    voxelization::helpers,
};

use super::common::{
    BRICK_POOL_COLORS_TEXTURE, OCTREE_LEVEL_START_INDICES, OCTREE_NODE_POOL,
    OCTREE_NODE_POOL_BRICK_POINTERS, TILES_PER_LEVEL,
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

    pub unsafe fn run(&self, level: u32) {
        self.shader.use_program();

        self.shader.set_uint(c_str!("octreeLevel"), level);

        helpers::bind_image_texture(0, OCTREE_NODE_POOL.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(
            1,
            OCTREE_NODE_POOL_BRICK_POINTERS.0,
            gl::READ_ONLY,
            gl::R32UI,
        );
        helpers::bind_3d_image_texture(2, BRICK_POOL_COLORS_TEXTURE, gl::READ_WRITE, gl::RGBA8);
        helpers::bind_image_texture(3, OCTREE_LEVEL_START_INDICES.0, gl::READ_ONLY, gl::R32UI);

        let tiles_in_level = TILES_PER_LEVEL[level as usize];
        let nodes_in_level = tiles_in_level * NODES_PER_TILE;
        let groups_count = (nodes_in_level as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
