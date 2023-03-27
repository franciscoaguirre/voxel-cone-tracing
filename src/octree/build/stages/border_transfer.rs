use c_str_macro::c_str;

use crate::{
    config::CONFIG, constants::WORKING_GROUP_SIZE, helpers, octree::OctreeTextures,
    rendering::shader::Shader,
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

    pub unsafe fn run(&self, textures: &OctreeTextures, nodes_per_level: &[u32], axis: u32) {
        self.shader.use_program();

        self.shader.set_uint(c_str!("axis"), axis);

        let octree_level = CONFIG.octree_levels - 1;
        self.shader.set_uint(c_str!("octreeLevel"), octree_level);

        helpers::bind_image_texture(0, textures.brick_pointers.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(
            1,
            if axis == 0 {
                textures.neighbors[0].0
            } else if axis == 1 {
                textures.neighbors[2].0
            } else {
                textures.neighbors[4].0
            },
            gl::READ_ONLY,
            gl::R32UI,
        );
        helpers::bind_3d_image_texture(2, textures.brick_pool_colors, gl::READ_WRITE, gl::RGBA8);
        helpers::bind_image_texture(3, textures.level_start_indices.0, gl::READ_ONLY, gl::R32UI);

        let (debug_texture, debug_texture_buffer) =
            helpers::generate_texture_buffer(1, gl::R32F, 10_f32);
        helpers::bind_image_texture(4, debug_texture, gl::WRITE_ONLY, gl::R32F);

        let nodes_in_level = nodes_per_level[octree_level as usize];
        let groups_count = (nodes_in_level as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        // self.shader.dispatch(groups_count);
        self.shader.dispatch(1);
        self.shader.wait();

        // let values = helpers::get_values_from_texture_buffer(debug_texture_buffer, 1, 20_f32);
        // dbg!(&values);
    }
}
