use c_str_macro::c_str;

use super::super::super::{OctreeTextures, VoxelData};
use crate::{config::CONFIG, helpers, rendering::shader::Shader};

pub struct NeighbourPointersPass {
    shader: Shader,
}

impl NeighbourPointersPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/neighborPointers.comp.glsl"),
        }
    }

    pub unsafe fn run(
        &self,
        voxel_data: &VoxelData,
        textures: &OctreeTextures,
        current_octree_level: u32,
    ) {
        self.shader.use_program();

        // Set uniforms
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension as u32);
        self.shader
            .set_uint(c_str!("octreeLevel"), current_octree_level);

        // Bind images
        helpers::bind_image_texture(0, textures.node_pool.0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(1, voxel_data.voxel_positions, gl::WRITE_ONLY, gl::R32UI);

        helpers::bind_image_texture(2, textures.neighbors[0].0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(3, textures.neighbors[1].0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(4, textures.neighbors[2].0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(5, textures.neighbors[3].0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(6, textures.neighbors[4].0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(7, textures.neighbors[5].0, gl::WRITE_ONLY, gl::R32UI);

        let (debug_texture, debug_texture_buffer) =
            helpers::generate_texture_buffer(9, gl::R32F, 69_f32);
        helpers::bind_image_texture(8, debug_texture, gl::WRITE_ONLY, gl::R32F);

        helpers::bind_image_texture(9, textures.level_start_indices.0, gl::READ_ONLY, gl::R32UI);

        let groups_count = (voxel_data.number_of_voxel_fragments as f32
            / CONFIG.working_group_size as f32)
            .ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
