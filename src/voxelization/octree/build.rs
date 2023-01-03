use std::mem::size_of;

use gl::types::*;
use log::info;

use super::{
    common::{
        BRICK_POOL_COLORS_TEXTURE, OCTREE_NODE_POOL, OCTREE_NODE_POOL_BRICK_POINTERS,
        OCTREE_NODE_POOL_NEIGHBOUR_X, OCTREE_NODE_POOL_NEIGHBOUR_X_NEGATIVE,
        OCTREE_NODE_POOL_NEIGHBOUR_Y, OCTREE_NODE_POOL_NEIGHBOUR_Y_NEGATIVE,
        OCTREE_NODE_POOL_NEIGHBOUR_Z, OCTREE_NODE_POOL_NEIGHBOUR_Z_NEGATIVE, TILES_PER_LEVEL,
    },
    helpers,
};
use crate::{
    config::CONFIG,
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
    let next_free_brick_counter: u32 = voxelization::helpers::generate_atomic_counter_buffer();

    let max_node_pool_size = helpers::get_max_node_pool_size();
    let max_node_pool_size_in_bytes = size_of::<GLuint>() * max_node_pool_size as usize;
    TILES_PER_LEVEL.push(1);

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

    let neighbour_pointers_pass = NeighbourPointersPass::init(voxel_positions_texture);
    let flag_nodes_pass = FlagNodesPass::init(number_of_voxel_fragments, voxel_positions_texture);
    let allocate_nodes_pass = AllocateNodesPass::init(allocated_tiles_counter);
    let allocate_bricks_pass = AllocateBricksPass::init(next_free_brick_counter);
    let write_leaf_nodes_pass =
        WriteLeafNodesPass::init(voxel_positions_texture, voxel_colors_texture);
    let spread_leaf_bricks_pass =
        SpreadLeafBricksPass::init(voxel_positions_texture, number_of_voxel_fragments);
    // let border_transfer_pass = BorderTransferPass::init();

    let mut first_tile_in_level: i32 = 0; // Index of first tile in a given octree level
    let mut first_free_tile: i32 = 1; // Index of first free tile (unallocated) in the octree

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
            octree_level, tiles_allocated
        );

        TILES_PER_LEVEL.push(tiles_allocated);
        first_tile_in_level += TILES_PER_LEVEL[octree_level as usize] as i32;
        first_free_tile += tiles_allocated as i32;
    }

    let all_tiles_allocated: u32 = TILES_PER_LEVEL.iter().sum();

    allocate_bricks_pass.run(all_tiles_allocated);

    let brick_pool_colors_texture_size_one_dimension = CONFIG.brick_pool_resolution;
    let brick_pool_colors_texture =
        voxelization::helpers::generate_3d_texture(brick_pool_colors_texture_size_one_dimension);
    BRICK_POOL_COLORS_TEXTURE = brick_pool_colors_texture;

    // let size = brick_pool_colors_texture_size_one_dimension.pow(3);

    write_leaf_nodes_pass.run(brick_pool_colors_texture);

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

    spread_leaf_bricks_pass.run(brick_pool_colors_texture);

    // border_transfer_pass.run();
    // border_transfer_pass.run();
    // border_transfer_pass.run();

    // TODO: Mipmap to inner nodes
}
