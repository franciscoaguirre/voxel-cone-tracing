use c_str_macro::c_str;

use crate::{
    config::CONFIG,
    constants::{X_AXIS, Y_AXIS, Z_AXIS},
    helpers,
    octree::{build::BrickPoolValues, NodeData, OctreeTextures},
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

    pub unsafe fn run(
        &self,
        textures: &OctreeTextures,
        node_data: &NodeData,
        octree_level: u32,
        brick_pool_values: BrickPoolValues,
    ) {
        self.shader.use_program();

        self.shader.set_uint(c_str!("octreeLevel"), octree_level);

        match brick_pool_values {
            BrickPoolValues::Colors => helpers::bind_3d_image_texture(
                1,
                textures.brick_pool_colors,
                gl::READ_WRITE,
                gl::RGBA8,
            ),
            BrickPoolValues::Normals => helpers::bind_3d_image_texture(
                1,
                textures.brick_pool_normals,
                gl::READ_WRITE,
                gl::RGBA8,
            ),
        }
        helpers::bind_image_texture(2, node_data.level_start_indices.0, gl::READ_ONLY, gl::R32UI);

        let nodes_in_level = node_data.nodes_per_level[octree_level as usize];
        let groups_count = (nodes_in_level as f32 / CONFIG.working_group_size as f32).ceil() as u32;

        self.shader.set_uint(c_str!("axis"), X_AXIS);
        helpers::bind_3d_image_texture(0, textures.neighbors, gl::READ_ONLY, gl::R32UI);
        self.shader.dispatch(groups_count);
        self.shader.wait();

        self.shader.set_uint(c_str!("axis"), Y_AXIS);
        helpers::bind_3d_image_texture(0, textures.neighbors, gl::READ_ONLY, gl::R32UI);
        self.shader.dispatch(groups_count);
        self.shader.wait();

        self.shader.set_uint(c_str!("axis"), Z_AXIS);
        helpers::bind_3d_image_texture(0, textures.neighbors, gl::READ_ONLY, gl::R32UI);
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
