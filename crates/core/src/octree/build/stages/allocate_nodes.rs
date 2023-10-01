use c_str_macro::c_str;
use engine::prelude::*;

use crate::{
    config::Config,
    octree::{OctreeTextures, VoxelData},
};

pub struct AllocateNodesPass {
    shader: Shader,
}

impl AllocateNodesPass {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/allocateNodes.comp.glsl"),
        }
    }

    pub unsafe fn run(
        &self,
        voxel_data: &VoxelData,
        textures: &OctreeTextures,
        allocated_nodes_counter: u32,
        first_node_in_level: i32,
        first_free_node: i32,
    ) {
        self.shader.use_program();

        self.shader
            .set_int(c_str!("firstNodeInLevel"), first_node_in_level);
        self.shader
            .set_int(c_str!("firstFreeNode"), first_free_node);
        gl::BindImageTexture(
            0,
            textures.node_pool.0,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::R32UI,
        );
        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, allocated_nodes_counter);

        let config = Config::instance();

        // TODO: The number of nodes should be `nodes_per_level` but for that
        // `voxel_data` and `node_data` need to be the top level split instead of
        // `geometry_data` and `border_data`
        // TODO: Should move to its own function since we use it all over the place
        let groups_count = (voxel_data.number_of_voxel_fragments as f32
            / config.working_group_size as f32)
            .ceil() as u32;

        // TODO: Could send even less threads
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
