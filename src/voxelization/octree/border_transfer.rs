use c_str_macro::c_str;
use gl::types::*;

use crate::{config::CONFIG, rendering::shader::Shader, voxelization::helpers};

use super::common::{
    BRICK_POOL_COLORS_TEXTURE, OCTREE_NODE_POOL, OCTREE_NODE_POOL_BRICK_POINTERS,
    OCTREE_NODE_POOL_NEIGHBOUR_X, OCTREE_NODE_POOL_NEIGHBOUR_Y, OCTREE_NODE_POOL_NEIGHBOUR_Z,
};

pub struct BorderTransferPass {
    shader: Shader,
    voxel_positions_texture: GLuint,
}

impl BorderTransferPass {
    pub fn init(voxel_positions_texture: GLuint) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/border_transfer.comp.glsl"),
            voxel_positions_texture,
        }
    }

    pub unsafe fn run(&self, axis: u32) {
        self.shader.use_program();

        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.shader.set_uint(c_str!("axis"), axis);
        self.shader
            .set_uint(c_str!("maxOctreeLevel"), CONFIG.octree_levels - 1);

        helpers::bind_image_texture(
            0,
            self.voxel_positions_texture,
            gl::READ_ONLY,
            gl::RGB10_A2UI,
        );
        helpers::bind_image_texture(1, OCTREE_NODE_POOL.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(
            2,
            OCTREE_NODE_POOL_BRICK_POINTERS.0,
            gl::READ_ONLY,
            gl::R32UI,
        );
        helpers::bind_image_texture(
            3,
            if axis == 0 {
                OCTREE_NODE_POOL_NEIGHBOUR_X.0
            } else if axis == 1 {
                OCTREE_NODE_POOL_NEIGHBOUR_Y.0
            } else {
                OCTREE_NODE_POOL_NEIGHBOUR_Z.0
            },
            gl::READ_ONLY,
            gl::R32UI,
        );
        helpers::bind_3d_image_texture(4, BRICK_POOL_COLORS_TEXTURE, gl::READ_WRITE, gl::RGBA8);

        self.shader.dispatch(65_535); // TODO: Calculate number of groups
        self.shader.wait();
    }
}
