use c_str_macro::c_str;

use crate::{
    config::CONFIG,
    constants::{CHILDREN_PER_NODE, WORKING_GROUP_SIZE},
    rendering::shader::Shader,
    voxelization::{helpers, octree::common::OCTREE_NODE_POOL, voxelize::VOXEL_POSITIONS},
};

use super::common::{NODES_PER_LEVEL, OCTREE_NODE_POSITIONS};

pub struct StoreNodePositions {
    shader: Shader,
}

impl StoreNodePositions {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/storeNodePositions.comp.glsl"),
        }
    }

    pub unsafe fn run(&self, octree_level: u32, number_of_voxel_fragments: u32) {
        self.shader.use_program();

        self.shader.set_uint(c_str!("octreeLevel"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        helpers::bind_image_texture(0, VOXEL_POSITIONS.0, gl::READ_ONLY, gl::RGB10_A2UI);
        helpers::bind_image_texture(1, OCTREE_NODE_POSITIONS.0, gl::WRITE_ONLY, gl::RGB10_A2UI);
        helpers::bind_image_texture(2, OCTREE_NODE_POOL.0, gl::READ_ONLY, gl::R32UI);

        let (debug_texture, debug_texture_buffer) =
            helpers::generate_texture_buffer(7, gl::R32F, 69_f32);
        helpers::bind_image_texture(3, debug_texture, gl::WRITE_ONLY, gl::R32F);

        let groups_count =
            (number_of_voxel_fragments as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;
        self.shader.dispatch(groups_count);
        self.shader.wait();

        // let values = helpers::get_values_from_texture_buffer(debug_texture_buffer, 7, 420_f32);
        // dbg!(&values);
    }
}
