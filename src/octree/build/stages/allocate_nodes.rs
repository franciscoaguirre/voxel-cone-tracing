use c_str_macro::c_str;

use crate::{
    config::CONFIG,
    octree::{OctreeTextures, VoxelData},
    rendering::shader::{Shader, compile_compute, compile_shaders},
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

        let groups_count = (voxel_data.number_of_voxel_fragments as f32
            / CONFIG.working_group_size as f32)
            .ceil() as u32;

        // TODO: Could send even less threads
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
