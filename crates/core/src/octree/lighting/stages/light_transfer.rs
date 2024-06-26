use c_str_macro::c_str;
use engine::prelude::*;
use gl::types::GLuint;

use crate::{
    config::Config,
    constants::Axis,
    octree::{NodeData, OctreeTextures},
};

pub struct LightTransfer {
    shader: Shader,
}

impl LightTransfer {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/lightTransfer.comp.glsl"),
        }
    }

    pub unsafe fn run(
        &self,
        textures: &OctreeTextures,
        octree_level: u32,
        node_data: &NodeData,
        axis: Axis,
        _light_view_map: GLuint,
    ) {
        self.shader.use_program();

        let config = Config::instance();

        self.shader.set_uint(c_str!("octreeLevel"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

        // TODO: Only needed for the 2D Node Map. Bring back when implementing it.
        // gl::ActiveTexture(gl::TEXTURE0);
        // gl::BindTexture(gl::TEXTURE_2D_ARRAY, light_view_map);
        // self.shader.set_int(c_str!("lightViewMap"), 0);

        helpers::bind_3d_image_texture(1, textures.brick_pool_photons, gl::READ_WRITE, gl::R32UI);
        helpers::bind_image_texture(2, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(3, node_data.level_start_indices.0, gl::READ_ONLY, gl::R32UI);

        let neighbors = match axis {
            Axis::X => textures.neighbors[0].0,
            Axis::Y => textures.neighbors[2].0,
            Axis::Z => textures.neighbors[4].0,
        };

        helpers::bind_image_texture(0, neighbors, gl::READ_ONLY, gl::R32UI);
        self.shader.set_uint(c_str!("axis"), axis.into());

        let nodes_in_level = node_data.nodes_per_level[octree_level as usize];
        let groups_count = (nodes_in_level as f32 / config.working_group_size as f32).ceil() as u32;
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
