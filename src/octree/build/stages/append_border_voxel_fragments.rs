use c_str_macro::c_str;

use crate::rendering::shader::compile_compute;
use crate::{
    config::CONFIG,
    helpers,
    octree::{OctreeData, OctreeTextures},
    rendering::shader::Shader,
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
        textures: &OctreeTextures,
    ) {
        self.shader.use_program();
        self.shader
            .set_uint(c_str!("octreeLevel"), CONFIG.last_octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        helpers::bind_image_texture(
            0,
            geometry_data.node_data.level_start_indices.0,
            gl::READ_ONLY,
            gl::R32UI,
        );
        helpers::bind_image_texture(
            1,
            border_data.voxel_data.voxel_positions.0,
            gl::WRITE_ONLY,
            gl::RGB10_A2UI,
        );
        helpers::bind_image_texture(2, textures.node_positions.0, gl::READ_ONLY, gl::RGB10_A2UI);

        let next_voxel_fragment_counter = helpers::generate_atomic_counter_buffer();
        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, next_voxel_fragment_counter);

        self.run_pass(geometry_data.voxel_data.number_of_voxel_fragments, textures);

        // Get the number of voxel fragments
        let number_of_voxel_fragments =
            helpers::get_value_from_atomic_counter(next_voxel_fragment_counter);
        border_data.voxel_data.number_of_voxel_fragments = number_of_voxel_fragments;
    }

    unsafe fn run_pass(&self, number_of_voxel_fragments: u32, textures: &OctreeTextures) {
        let call_offset = 0;
        self.run_half(call_offset, number_of_voxel_fragments, textures);

        let call_offset = 3;
        self.run_half(call_offset, number_of_voxel_fragments, textures);
    }

    unsafe fn run_half(
        &self,
        call_offset: usize,
        number_of_voxel_fragments: u32,
        textures: &OctreeTextures,
    ) {
        self.shader
            .set_uint(c_str!("callOffset"), call_offset as u32);
        for texture_offset in 0..(textures.neighbors.len() / 2) {
            helpers::bind_image_texture(
                3 + texture_offset as u32,
                textures.neighbors[texture_offset + call_offset].0,
                gl::READ_ONLY,
                gl::R32UI,
            );
        }

        let groups_count =
            (number_of_voxel_fragments as f32 / CONFIG.working_group_size as f32).ceil() as u32;
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
