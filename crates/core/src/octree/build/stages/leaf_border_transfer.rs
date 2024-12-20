use c_str_macro::c_str;
use engine::prelude::*;

use crate::{
    config::Config,
    constants::Axis,
    octree::{build::BrickPoolValues, NodeData, OctreeTextures},
};

pub struct LeafBorderTransferPass {
    shader: Shader,
}

impl LeafBorderTransferPass {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/leafBorderTransfer.comp.glsl"),
        }
    }

    pub unsafe fn run(
        &self,
        textures: &OctreeTextures,
        geometry_node_data: &NodeData,
        brick_pool_values: BrickPoolValues,
    ) {
        self.shader.use_program();

        let config = Config::instance();

        self.shader
            .set_uint(c_str!("octreeLevel"), config.last_octree_level());

        match brick_pool_values {
            BrickPoolValues::Colors => helpers::bind_3d_image_texture(
                1,
                textures.brick_pool_colors[0], // We use +X texture for lowest level
                gl::READ_WRITE,
                gl::RGBA8,
            ),
            BrickPoolValues::Normals => helpers::bind_3d_image_texture(
                1,
                textures.brick_pool_normals,
                gl::READ_WRITE,
                gl::RGBA8,
            ),
            BrickPoolValues::Irradiance => helpers::bind_3d_image_texture(
                1,
                textures.brick_pool_irradiance[0], // We use +X texture for lower level
                gl::READ_WRITE,
                gl::RGBA8,
            ),
        }

        let geometry_nodes_in_level =
            geometry_node_data.nodes_per_level[config.last_octree_level() as usize];
        let geometry_groups_count =
            (geometry_nodes_in_level as f32 / config.working_group_size as f32).ceil() as u32;

        for axis in Axis::all_axis().iter() {
            self.shader.set_uint(c_str!("axis"), (*axis).into());
            let neighbor_texture_index: usize = (*axis).into();
            let neighbor_texture_index = neighbor_texture_index * 2;
            helpers::bind_image_texture(
                0,
                textures.neighbors[neighbor_texture_index].0,
                gl::READ_ONLY,
                gl::R32UI,
            );

            helpers::bind_image_texture(
                2,
                geometry_node_data.level_start_indices.0,
                gl::READ_ONLY,
                gl::R32UI,
            );
            self.shader.dispatch(geometry_groups_count);
            self.shader.wait();
        }
    }
}
