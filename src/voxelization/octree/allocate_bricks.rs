use c_str_macro::c_str;

use crate::config::CONFIG;
use crate::rendering::shader::Shader;

use super::common::{OCTREE_NODE_POOL, OCTREE_NODE_POOL_BRICK_POINTERS};

pub struct AllocateBricksPass {
    shader: Shader,
    next_free_brick_counter: u32,
}

impl AllocateBricksPass {
    pub fn init(next_free_brick_counter: u32) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/allocate_bricks.comp.glsl"),
            next_free_brick_counter,
        }
    }

    pub unsafe fn run(&self, all_tiles_allocated: u32) {
        self.shader.use_program();

        self.shader.set_uint(
            c_str!("brick_pool_resolution"),
            CONFIG.brick_pool_resolution as u32,
        );

        gl::BindImageTexture(
            0,
            OCTREE_NODE_POOL.0,
            0,
            gl::FALSE,
            0,
            gl::READ_ONLY,
            gl::R32UI,
        );

        gl::BindImageTexture(
            1,
            OCTREE_NODE_POOL_BRICK_POINTERS.0,
            0,
            gl::FALSE,
            0,
            gl::WRITE_ONLY,
            gl::R32UI,
        );

        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, self.next_free_brick_counter);

        self.shader
            .dispatch(all_tiles_allocated / CONFIG.working_group_size + 1);
        self.shader.wait();
    }
}
