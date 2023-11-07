use c_str_macro::c_str;
use engine::prelude::*;

use crate::{
    config::Config,
    octree::{OctreeData, OctreeTextures},
};

pub struct AppendBorderVoxelFragmentsPass {
    shader: Shader,
}

impl AppendBorderVoxelFragmentsPass {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/appendBorderVoxelFragments.comp.glsl",),
        }
    }

    pub unsafe fn run(
        &self,
        geometry_data: &OctreeData,
        border_data: &mut OctreeData,
        octree_level: u32,
        level_start: u32,
        nodes_in_current_level: u32,
        textures: &OctreeTextures,
    ) {
        log::debug!(
            "{}, {}, {}",
            octree_level,
            level_start,
            nodes_in_current_level
        );
        let config = Config::instance();
        self.shader.use_program();
        self.shader
            .set_uint(c_str!("maxOctreeLevel"), config.last_octree_level());
        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.shader
            .set_uint(c_str!("levelStart"), level_start);
        self.shader
            .set_uint(c_str!("nextLevelStart"), level_start + nodes_in_current_level);
        self.shader
            .set_uint(c_str!("octreeLevel"), octree_level);

        helpers::bind_image_texture(
            0,
            border_data.voxel_data.voxel_positions.texture(),
            gl::WRITE_ONLY,
            gl::RGB10_A2UI,
        );
        helpers::bind_image_texture(1, textures.node_positions.0, gl::READ_ONLY, gl::RGB10_A2UI);

        let next_voxel_fragment_counter = helpers::generate_atomic_counter_buffer();
        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, next_voxel_fragment_counter);

        let (debug_texture, debug_texture_buffer) = helpers::generate_texture_buffer(20, gl::R32F, 42f32);
        helpers::bind_image_texture(7, debug_texture, gl::WRITE_ONLY, gl::R32F);

        self.run_pass(nodes_in_current_level, textures);

        let values = helpers::get_values_from_texture_buffer(debug_texture_buffer, 20, 0f32);
        dbg!(&values);

        // Get the number of voxel fragments
        let number_of_voxel_fragments =
            helpers::get_value_from_atomic_counter(next_voxel_fragment_counter);
        border_data.voxel_data.number_of_voxel_fragments = number_of_voxel_fragments;
        log::debug!(
            "border voxel fragments for {}: {}",
            octree_level,
            number_of_voxel_fragments
        );
        panic!();
    }

    unsafe fn run_pass(&self, nodes_in_current_level: u32, textures: &OctreeTextures) {
        let call_offset = 0;
        self.run_half(call_offset, nodes_in_current_level, textures);

        let call_offset = 3;
        self.run_half(call_offset, nodes_in_current_level, textures);
    }

    unsafe fn run_half(
        &self,
        call_offset: usize,
        nodes_in_current_level: u32,
        textures: &OctreeTextures,
    ) {
        self.shader
            .set_uint(c_str!("callOffset"), call_offset as u32);
        for texture_offset in 0..(textures.neighbors.len() / 2) {
            helpers::bind_image_texture(
                2 + texture_offset as u32,
                textures.neighbors[texture_offset + call_offset].0,
                gl::READ_ONLY,
                gl::R32UI,
            );
        }

        let config = Config::instance();

        let groups_count =
            (nodes_in_current_level as f32 / config.working_group_size as f32).ceil() as u32;
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
