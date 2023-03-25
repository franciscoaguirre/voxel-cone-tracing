use c_str_macro::c_str;

use crate::config::CONFIG;
use crate::helpers;
use crate::octree::OctreeTextures;
use crate::rendering::shader::Shader;

pub struct AllocateBricksPass {
    shader: Shader,
    next_free_brick_counter: u32,
}

impl AllocateBricksPass {
    pub fn init(next_free_brick_counter: u32) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/allocateBricks.comp.glsl"),
            next_free_brick_counter,
        }
    }

    pub unsafe fn run(&self, textures: &OctreeTextures, all_nodes_allocated: u32) {
        self.shader.use_program();

        self.shader.set_uint(
            c_str!("brickPoolResolution"),
            CONFIG.brick_pool_resolution as u32,
        );

        helpers::bind_image_texture(0, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(1, textures.brick_pointers.0, gl::WRITE_ONLY, gl::R32UI);

        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, self.next_free_brick_counter);

        self.shader
            .dispatch(all_nodes_allocated / CONFIG.working_group_size + 1);
        self.shader.wait();
    }
}
