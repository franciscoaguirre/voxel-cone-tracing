use engine::prelude::*;

/// The buffers relating to voxels
#[derive(Clone)]
pub struct VoxelData {
    pub voxel_positions: BufferTextureV2<u32>,
    pub number_of_voxel_fragments: u32,
    pub voxel_colors: BufferTexture,
    pub voxel_normals: BufferTexture,
}

/// Converter from a single buffer texture
/// Only sets `voxel_positions`
impl From<BufferTextureV2<u32>> for VoxelData {
    fn from(buffer_texture: BufferTextureV2<u32>) -> Self {
        let length = buffer_texture.len() as u32;
        VoxelData {
            voxel_positions: buffer_texture,
            number_of_voxel_fragments: length,
            voxel_colors: (0, 0),
            voxel_normals: (0, 0),
        }
    }
}
