use c_str_macro::c_str;

use crate::{
    config::CONFIG,
    constants::{CHILDREN_PER_NODE, WORKING_GROUP_SIZE},
    rendering::shader::Shader,
    voxelization::helpers,
};

use super::common::{
    BRICK_POOL_COLORS_TEXTURE, NODES_PER_LEVEL, OCTREE_LEVEL_START_INDICES,
    OCTREE_NODE_POOL_BRICK_POINTERS, OCTREE_NODE_POOL_NEIGHBOUR_X, OCTREE_NODE_POOL_NEIGHBOUR_Y,
    OCTREE_NODE_POOL_NEIGHBOUR_Z,
};

pub struct BorderTransferPass {
    shader: Shader,
}

impl BorderTransferPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/borderTransfer.comp.glsl"),
        }
    }

    pub unsafe fn run(&self, axis: u32) {
        self.shader.use_program();

        self.shader.set_uint(c_str!("axis"), axis);

        let octree_level = CONFIG.octree_levels - 1;
        self.shader.set_uint(c_str!("octreeLevel"), octree_level);

        helpers::bind_image_texture(
            0,
            OCTREE_NODE_POOL_BRICK_POINTERS.0,
            gl::READ_ONLY,
            gl::R32UI,
        );
        helpers::bind_image_texture(
            1,
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
        helpers::bind_3d_image_texture(2, BRICK_POOL_COLORS_TEXTURE, gl::READ_WRITE, gl::RGBA8);
        helpers::bind_image_texture(3, OCTREE_LEVEL_START_INDICES.0, gl::READ_ONLY, gl::R32UI);

        let (debug_texture, debug_texture_buffer) =
            helpers::generate_texture_buffer(1, gl::R32F, 10_f32);
        helpers::bind_image_texture(4, debug_texture, gl::WRITE_ONLY, gl::R32F);

        let tiles_in_level = NODES_PER_LEVEL[octree_level as usize];
        let nodes_in_level = tiles_in_level * CHILDREN_PER_NODE;
        let groups_count = (nodes_in_level as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        // self.shader.dispatch(groups_count);
        self.shader.dispatch(1);
        self.shader.wait();

        // let values = helpers::get_values_from_texture_buffer(debug_texture_buffer, 1, 20_f32);
        // dbg!(&values);
    }
}
