use std::mem::size_of;

use gl::types::*;
use log::info;

use super::{
    common::{
        BRICK_POOL_COLORS_TEXTURE, OCTREE_LEVEL_START_INDICES, OCTREE_NODE_POOL,
        OCTREE_NODE_POOL_BRICK_POINTERS, OCTREE_NODE_POOL_NEIGHBOUR_X,
        OCTREE_NODE_POOL_NEIGHBOUR_X_NEGATIVE, OCTREE_NODE_POOL_NEIGHBOUR_Y,
        OCTREE_NODE_POOL_NEIGHBOUR_Y_NEGATIVE, OCTREE_NODE_POOL_NEIGHBOUR_Z,
        OCTREE_NODE_POOL_NEIGHBOUR_Z_NEGATIVE, TILES_PER_LEVEL,
    },
    helpers,
    mipmap_center::MipmapCenterPass,
    mipmap_corners::MipmapCornersPass,
    mipmap_edges::MipmapEdgesPass,
    mipmap_faces::MipmapFacesPass,
};
use crate::{
    config::CONFIG,
    constants::{X_AXIS, Y_AXIS, Z_AXIS},
    voxelization::{
        self,
        octree::{
            allocate_bricks::AllocateBricksPass, allocate_nodes::AllocateNodesPass,
            border_transfer::BorderTransferPass, flag_nodes::FlagNodesPass,
            neighbour_pointers::NeighbourPointersPass, spread_leaf_bricks::SpreadLeafBricksPass,
            write_leaf_nodes::WriteLeafNodesPass,
        },
    },
};

pub unsafe fn build_octree(
    voxel_positions_texture: GLuint,
    number_of_voxel_fragments: u32,
    voxel_colors_texture: GLuint,
) {
    let allocated_tiles_counter = voxelization::helpers::generate_atomic_counter_buffer();
    let next_free_brick_counter = voxelization::helpers::generate_atomic_counter_buffer();

    let max_node_pool_size = helpers::get_max_node_pool_size();
    let max_node_pool_size_in_bytes = size_of::<GLuint>() * max_node_pool_size as usize;
    TILES_PER_LEVEL.push(1);

    initialize_common_textures(max_node_pool_size_in_bytes);

    let neighbour_pointers_pass =
        NeighbourPointersPass::init(voxel_positions_texture, number_of_voxel_fragments);
    let flag_nodes_pass = FlagNodesPass::init(number_of_voxel_fragments, voxel_positions_texture);
    let allocate_nodes_pass =
        AllocateNodesPass::init(allocated_tiles_counter, number_of_voxel_fragments);
    let allocate_bricks_pass = AllocateBricksPass::init(next_free_brick_counter);
    let write_leaf_nodes_pass = WriteLeafNodesPass::init(
        voxel_positions_texture,
        voxel_colors_texture,
        number_of_voxel_fragments,
    );
    // let spread_leaf_bricks_pass = SpreadLeafBricksPass::init();
    let border_transfer_pass = BorderTransferPass::init();
    let mipmap_center_pass = MipmapCenterPass::init();
    let mipmap_faces_pass = MipmapFacesPass::init();
    let mipmap_corners_pass = MipmapCornersPass::init();
    let mipmap_edges_pass = MipmapEdgesPass::init();

    let mut octree_level_start_indices = Vec::with_capacity(CONFIG.octree_levels as usize);

    let mut first_tile_in_level = 0; // Index of first tile in a given octree level
    let mut first_free_tile = 1; // Index of first free tile (unallocated) in the octree

    octree_level_start_indices.push(first_tile_in_level);

    for octree_level in 0..CONFIG.octree_levels {
        if octree_level > 0 {
            neighbour_pointers_pass.run(octree_level);
        }

        flag_nodes_pass.run(octree_level);

        allocate_nodes_pass.run(first_tile_in_level, first_free_tile);

        let tiles_allocated =
            voxelization::helpers::get_value_from_atomic_counter(allocated_tiles_counter);
        info!(
            "Tiles allocated in level {}: {}",
            octree_level + 1,
            tiles_allocated
        );

        TILES_PER_LEVEL.push(tiles_allocated);
        first_tile_in_level += TILES_PER_LEVEL[octree_level as usize] as i32;
        first_free_tile += tiles_allocated as i32;

        octree_level_start_indices.push(first_tile_in_level);
    }

    OCTREE_LEVEL_START_INDICES = voxelization::helpers::generate_texture_buffer_with_data(
        (CONFIG.octree_levels + 1) as usize,
        gl::R32UI,
        octree_level_start_indices,
    );

    let all_tiles_allocated: u32 = TILES_PER_LEVEL.iter().sum();

    allocate_bricks_pass.run(all_tiles_allocated);

    let brick_pool_colors_texture_size_one_dimension = CONFIG.brick_pool_resolution;
    BRICK_POOL_COLORS_TEXTURE =
        voxelization::helpers::generate_3d_texture(brick_pool_colors_texture_size_one_dimension);

    // let size = brick_pool_colors_texture_size_one_dimension.pow(3);

    write_leaf_nodes_pass.run();

    // let mut bricks = vec![1u32; size as usize];
    // gl::BindTexture(gl::TEXTURE_3D, brick_pool_colors_texture);
    // gl::GetTexImage(
    //     gl::TEXTURE_3D,
    //     0,
    //     gl::RGBA,
    //     gl::UNSIGNED_BYTE,
    //     bricks.as_mut_ptr() as *mut c_void,
    // );
    // gl::BindTexture(gl::TEXTURE_3D, 0);

    // spread_leaf_bricks_pass.run();

    border_transfer_pass.run(X_AXIS);
    border_transfer_pass.run(Y_AXIS);
    border_transfer_pass.run(Z_AXIS);

    for level in (0..CONFIG.octree_levels - 1).rev() {
        mipmap_center_pass.run(level);
        mipmap_faces_pass.run(level);
        mipmap_corners_pass.run(level);
        mipmap_edges_pass.run(level);

        if level > 0 {
            border_transfer_pass.run(X_AXIS);
            border_transfer_pass.run(Y_AXIS);
            border_transfer_pass.run(Z_AXIS);
        }
    }
}

unsafe fn initialize_common_textures(max_node_pool_size_in_bytes: usize) {
    OCTREE_NODE_POOL = voxelization::helpers::generate_texture_buffer(
        max_node_pool_size_in_bytes,
        gl::R32UI,
        0u32,
    );
    OCTREE_NODE_POOL_BRICK_POINTERS = voxelization::helpers::generate_texture_buffer(
        max_node_pool_size_in_bytes,
        gl::R32UI,
        0u32,
    );
    OCTREE_NODE_POOL_NEIGHBOUR_X = voxelization::helpers::generate_texture_buffer(
        max_node_pool_size_in_bytes,
        gl::R32UI,
        0u32,
    );
    OCTREE_NODE_POOL_NEIGHBOUR_X_NEGATIVE = voxelization::helpers::generate_texture_buffer(
        max_node_pool_size_in_bytes,
        gl::R32UI,
        0u32,
    );
    OCTREE_NODE_POOL_NEIGHBOUR_Y = voxelization::helpers::generate_texture_buffer(
        max_node_pool_size_in_bytes,
        gl::R32UI,
        0u32,
    );
    OCTREE_NODE_POOL_NEIGHBOUR_Y_NEGATIVE = voxelization::helpers::generate_texture_buffer(
        max_node_pool_size_in_bytes,
        gl::R32UI,
        0u32,
    );
    OCTREE_NODE_POOL_NEIGHBOUR_Z = voxelization::helpers::generate_texture_buffer(
        max_node_pool_size_in_bytes,
        gl::R32UI,
        0u32,
    );
    OCTREE_NODE_POOL_NEIGHBOUR_Z_NEGATIVE = voxelization::helpers::generate_texture_buffer(
        max_node_pool_size_in_bytes,
        gl::R32UI,
        0u32,
    );
}
