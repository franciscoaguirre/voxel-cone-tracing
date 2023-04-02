use c_str_macro::c_str;

use crate::{
    config::CONFIG, constants::WORKING_GROUP_SIZE, helpers, octree::OctreeTextures,
    rendering::shader::Shader, voxelization::voxelize::VOXEL_POSITIONS,
};

pub struct StoreNodePositions {
    shader: Shader,
}

impl StoreNodePositions {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/storeNodePositions.comp.glsl"),
        }
    }

    pub unsafe fn run(
        &self,
        textures: &OctreeTextures,
        octree_level: u32,
        number_of_voxel_fragments: u32,
    ) {
        self.shader.use_program();

        self.shader.set_uint(c_str!("octreeLevel"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        helpers::bind_image_texture(0, VOXEL_POSITIONS.0, gl::READ_ONLY, gl::RGB10_A2UI);
        helpers::bind_image_texture(1, textures.node_positions.0, gl::WRITE_ONLY, gl::RGB10_A2UI);
        helpers::bind_image_texture(2, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);

        let groups_count =
            (number_of_voxel_fragments as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
