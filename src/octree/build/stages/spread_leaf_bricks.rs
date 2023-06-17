use c_str_macro::c_str;

use crate::{
    config::CONFIG,
    helpers,
    octree::{build::BrickPoolValues, NodeData, OctreeTextures},
    rendering::shader::Shader,
};

pub struct SpreadLeafBricksPass {
    shader: Shader,
}

impl SpreadLeafBricksPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/spreadLeafBricks.comp.glsl"),
        }
    }

    pub unsafe fn run(
        &self,
        textures: &OctreeTextures,
        node_data: &NodeData,
        brick_pool_values: BrickPoolValues,
    ) {
        self.shader.use_program();

        let octree_level = CONFIG.octree_levels - 1;
        self.shader.set_uint(c_str!("octreeLevel"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        match brick_pool_values {
            BrickPoolValues::Colors => {
                helpers::bind_3d_image_texture(
                    0,
                    textures.brick_pool_colors,
                    gl::READ_WRITE,
                    gl::RGBA8,
                );
            }
            BrickPoolValues::Normals => {
                helpers::bind_3d_image_texture(
                    0,
                    textures.brick_pool_normals,
                    gl::READ_WRITE,
                    gl::RGBA8,
                );
            }
        }
        helpers::bind_image_texture(1, node_data.level_start_indices.0, gl::READ_ONLY, gl::R32UI);

        let nodes_in_level = node_data.nodes_per_level[octree_level as usize];
        let groups_count = (nodes_in_level as f32 / CONFIG.working_group_size as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
