use c_str_macro::c_str;

use crate::{constants::WORKING_GROUP_SIZE, rendering::shader::Shader};

use super::common::OCTREE_NODE_POOL;

pub struct AllocateNodesPass {
    shader: Shader,
    allocated_nodes_counter: u32,
    number_of_voxel_fragments: u32,
}

impl AllocateNodesPass {
    pub fn init(allocated_nodes_counter: u32, number_of_voxel_fragments: u32) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/allocateNodes.comp.glsl"),
            allocated_nodes_counter,
            number_of_voxel_fragments,
        }
    }

    pub unsafe fn run(&self, first_node_in_level: i32, first_free_node: i32) {
        self.shader.use_program();

        self.shader
            .set_int(c_str!("firstNodeInLevel"), first_node_in_level);
        self.shader
            .set_int(c_str!("firstFreeNode"), first_free_node);
        gl::BindImageTexture(
            0,
            OCTREE_NODE_POOL.0,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::R32UI,
        );
        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, self.allocated_nodes_counter);

        let groups_count =
            (self.number_of_voxel_fragments as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        // TODO: Could still send less threads
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
