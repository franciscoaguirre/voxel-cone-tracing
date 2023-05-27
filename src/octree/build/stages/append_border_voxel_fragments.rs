use c_str_macro::c_str;

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
            shader: Shader::new_compute(
                "assets/shaders/octree/appendBorderVoxelFragments.comp.glsl",
            ),
        }
    }

    pub unsafe fn run(
        &self,
        geometry_data: &OctreeData,
        border_data: &mut OctreeData,
        textures: &OctreeTextures,
    ) {
        self.shader.use_program();
        // Last level
        self.shader
            .set_uint(c_str!("octreeLevel"), CONFIG.octree_levels - 1);
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

        for texture_offset in 0..textures.neighbors.len() {
            helpers::bind_image_texture(
                3 + texture_offset as u32,
                textures.neighbors[texture_offset as usize].0,
                gl::READ_ONLY,
                gl::R32UI,
            );
        }

        let next_voxel_fragment_counter = helpers::generate_atomic_counter_buffer();
        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, next_voxel_fragment_counter);

        self.shader
            .dispatch(geometry_data.node_data.nodes_per_level[(CONFIG.octree_levels - 1) as usize]); // Call first with `shouldStore = false`
        self.shader.wait();

        let number_of_voxel_fragments =
            helpers::get_value_from_atomic_counter(next_voxel_fragment_counter);
        border_data.voxel_data.number_of_voxel_fragments = number_of_voxel_fragments;
    }
}
