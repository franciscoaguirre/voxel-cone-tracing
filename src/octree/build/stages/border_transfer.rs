use c_str_macro::c_str;

use crate::{
    config::CONFIG,
    constants::Axis,
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
        axis: Axis,
    ) {
        self.shader.use_program();

        self.shader.set_uint(c_str!("octreeLevel"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

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

        self.shader.set_uint(c_str!("axis"), axis.into());
        let neighbors = match axis {
            Axis::X => textures.neighbors[0].0,
            Axis::Y => textures.neighbors[2].0,
            Axis::Z => textures.neighbors[4].0,
        };
        helpers::bind_image_texture(0, neighbors, gl::READ_ONLY, gl::R32UI);
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
