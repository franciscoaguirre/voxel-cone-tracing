use c_str_macro::c_str;

use super::super::super::{OctreeTextures, VoxelData};
use crate::{config::CONFIG, helpers, octree::NodeData, rendering::shader::Shader};

pub struct NeighborPointersPass {
    shader: Shader,
}

impl NeighborPointersPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/neighborPointers.comp.glsl"),
        }
    }

    pub unsafe fn run(
        &self,
        voxel_data: &VoxelData,
        node_data: &NodeData,
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
        helpers::bind_image_texture(1, voxel_data.voxel_positions.0, gl::WRITE_ONLY, gl::R32UI);

        helpers::bind_3d_image_texture(2, textures.neighbors, gl::WRITE_ONLY, gl::R32UI);

        let (debug_texture, debug_texture_buffer) =
            helpers::generate_texture_buffer(9, gl::R32F, 69_f32);
        helpers::bind_image_texture(8, debug_texture, gl::WRITE_ONLY, gl::R32F);

        helpers::bind_image_texture(9, node_data.level_start_indices.0, gl::READ_ONLY, gl::R32UI);

        let groups_count = (voxel_data.number_of_voxel_fragments as f32
            / CONFIG.working_group_size as f32)
            .ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
