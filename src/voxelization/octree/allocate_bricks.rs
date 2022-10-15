use c_str_macro::c_str;
use gl::types::*;

use crate::constants::{BRICK_POOL_RESOLUTION, WORKING_GROUP_SIZE};
use crate::rendering::shader::Shader;

pub struct AllocateBricksPass {
    shader: Shader,
    next_free_brick_counter: u32,
    octree_node_pool_texture: GLuint,
    octree_node_pool_brick_pointers_texture: GLuint,
}

impl AllocateBricksPass {
    pub fn init(
        next_free_brick_counter: u32,
        octree_node_pool_texture: GLuint,
        octree_node_pool_brick_pointers_texture: GLuint,
    ) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/allocate_bricks.comp.glsl"),
            next_free_brick_counter,
            octree_node_pool_texture,
            octree_node_pool_brick_pointers_texture,
        }
    }

    pub unsafe fn run(&self, all_tiles_allocated: u32) {
        self.shader.set_uint(
            c_str!("brick_pool_resolution"),
            BRICK_POOL_RESOLUTION as u32,
        );
        self.shader.use_program();

        gl::BindImageTexture(
            0,
            self.octree_node_pool_texture,
            0,
            gl::FALSE,
            0,
            gl::READ_ONLY,
            gl::R32UI,
        );

        gl::BindImageTexture(
            1,
            self.octree_node_pool_brick_pointers_texture,
            0,
            gl::FALSE,
            0,
            gl::WRITE_ONLY,
            gl::R32UI,
        );

        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, self.next_free_brick_counter);

        self.shader
            .dispatch(all_tiles_allocated / WORKING_GROUP_SIZE + 1);
        self.shader.wait();
    }
}
