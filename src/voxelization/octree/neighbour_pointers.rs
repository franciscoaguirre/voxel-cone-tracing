use c_str_macro::c_str;
use gl::types::*;

use crate::{config::CONFIG, rendering::shader::Shader, voxelization::helpers::bind_image_texture};

use super::common::{
    OCTREE_NODE_POOL, OCTREE_NODE_POOL_NEIGHBOUR_X, OCTREE_NODE_POOL_NEIGHBOUR_X_NEGATIVE,
    OCTREE_NODE_POOL_NEIGHBOUR_Y, OCTREE_NODE_POOL_NEIGHBOUR_Y_NEGATIVE,
    OCTREE_NODE_POOL_NEIGHBOUR_Z, OCTREE_NODE_POOL_NEIGHBOUR_Z_NEGATIVE,
};

pub struct NeighbourPointersPass {
    shader: Shader,
    voxel_positions_texture: GLuint,
}

impl NeighbourPointersPass {
    pub fn init(voxel_positions_texture: GLuint) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/neighbour_pointers.comp.glsl"),
            voxel_positions_texture,
        }
    }

    pub unsafe fn run(&self, current_octree_level: u32) {
        self.shader.use_program();

        // Set uniforms
        self.shader
            .set_uint(c_str!("voxel_dimension"), CONFIG.voxel_dimension as u32);
        self.shader
            .set_uint(c_str!("current_octree_level"), current_octree_level);

        // Bind images
        bind_image_texture(0, OCTREE_NODE_POOL.0, gl::READ_WRITE, gl::R32UI);
        bind_image_texture(1, self.voxel_positions_texture, gl::READ_WRITE, gl::R32UI);

        bind_image_texture(2, OCTREE_NODE_POOL_NEIGHBOUR_X.0, gl::READ_WRITE, gl::R32UI);
        bind_image_texture(
            3,
            OCTREE_NODE_POOL_NEIGHBOUR_X_NEGATIVE.0,
            gl::READ_WRITE,
            gl::R32UI,
        );
        bind_image_texture(4, OCTREE_NODE_POOL_NEIGHBOUR_Y.0, gl::READ_WRITE, gl::R32UI);
        bind_image_texture(
            5,
            OCTREE_NODE_POOL_NEIGHBOUR_Y_NEGATIVE.0,
            gl::READ_WRITE,
            gl::R32UI,
        );
        bind_image_texture(6, OCTREE_NODE_POOL_NEIGHBOUR_Z.0, gl::READ_WRITE, gl::R32UI);
        bind_image_texture(
            7,
            OCTREE_NODE_POOL_NEIGHBOUR_Z_NEGATIVE.0,
            gl::READ_WRITE,
            gl::R32UI,
        );

        self.shader.dispatch(65_535); // TODO: Calculate number of groups
        self.shader.wait();
    }
}
