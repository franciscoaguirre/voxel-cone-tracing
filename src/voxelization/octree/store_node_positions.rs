use c_str_macro::c_str;

use crate::{
    config::CONFIG,
    constants::{NODES_PER_TILE, WORKING_GROUP_SIZE},
    rendering::shader::Shader,
    voxelization::{helpers, octree::common::OCTREE_NODE_POOL, voxelize::VOXEL_POSITIONS},
};

use super::common::{OCTREE_NODE_POSITIONS, TILES_PER_LEVEL};

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

        let tiles_in_level = TILES_PER_LEVEL[octree_level as usize];
        let nodes_in_level = tiles_in_level * NODES_PER_TILE;
        let groups_count = (nodes_in_level as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        // self.shader.dispatch(groups_count);
        self.shader.dispatch(1);
        // self.shader.dispatch(number_of_voxel_fragments);
        self.shader.wait();

        let values = helpers::get_values_from_texture_buffer(debug_texture_buffer, 7, 420_f32);
        dbg!(&values);

        // let values =
        //     helpers::get_values_from_texture_buffer(OCTREE_NODE_POSITIONS.1, 10_000, 69_u32);

        // for value in values.iter() {
        //     if *value != 0 {
        //         dbg!(value);
        //     }
        // }
    }
}
