use gl::types::*;
use std::mem::size_of;

use super::{
    common::{
        OCTREE_NODE_POOL_BRICK_POINTERS_TEXTURE, OCTREE_NODE_POOL_BRICK_POINTERS_TEXTURE_BUFFER,
        OCTREE_NODE_POOL_TEXTURE, OCTREE_NODE_POOL_TEXTURE_BUFFER, TILES_PER_LEVEL,
    },
    helpers,
};
use crate::{
    constants::OCTREE_LEVELS,
    voxelization::{
        self,
        octree::{
            allocate_bricks::AllocateBricksPass, allocate_nodes::AllocateNodesPass,
            flag_nodes::FlagNodesPass,
        },
    },
};

pub unsafe fn build_octree(voxel_position_texture: GLuint, number_of_voxel_fragments: u32) {
    let allocated_tiles_counter = voxelization::helpers::generate_atomic_counter_buffer();
    let next_free_brick_counter: u32 = voxelization::helpers::generate_atomic_counter_buffer();

    let max_node_pool_size = helpers::get_max_node_pool_size();
    TILES_PER_LEVEL.push(1);

    voxelization::helpers::generate_linear_buffer(
        size_of::<GLuint>() * max_node_pool_size as usize,
        gl::R32UI,
        &mut OCTREE_NODE_POOL_TEXTURE,
        &mut OCTREE_NODE_POOL_TEXTURE_BUFFER,
    );

    voxelization::helpers::generate_linear_buffer(
        size_of::<GLuint>() * max_node_pool_size as usize,
        gl::R32UI,
        &mut OCTREE_NODE_POOL_BRICK_POINTERS_TEXTURE,
        &mut OCTREE_NODE_POOL_BRICK_POINTERS_TEXTURE_BUFFER,
    );

    voxelization::helpers::clear_texture_buffer(
        OCTREE_NODE_POOL_TEXTURE_BUFFER,
        max_node_pool_size,
    );

    voxelization::helpers::clear_texture_buffer(
        OCTREE_NODE_POOL_BRICK_POINTERS_TEXTURE,
        max_node_pool_size,
    );

    let flag_nodes_pass = FlagNodesPass::init(
        number_of_voxel_fragments,
        voxel_position_texture,
        OCTREE_NODE_POOL_TEXTURE,
    );
    let allocate_nodes_pass =
        AllocateNodesPass::init(allocated_tiles_counter, OCTREE_NODE_POOL_TEXTURE);
    let allocate_bricks_pass = AllocateBricksPass::init(
        next_free_brick_counter,
        OCTREE_NODE_POOL_TEXTURE,
        OCTREE_NODE_POOL_BRICK_POINTERS_TEXTURE,
    );

    let mut first_tile_in_level: i32 = 0; // Index of first tile in a given octree level
    let mut first_free_tile: i32 = 1; // Index of first free tile (unallocated) in the octree

    for octree_level in 0..OCTREE_LEVELS {
        flag_nodes_pass.run(octree_level);

        allocate_nodes_pass.run(first_tile_in_level, first_free_tile);

        let tiles_allocated =
            voxelization::helpers::get_value_from_atomic_counter(allocated_tiles_counter);
        dbg!(tiles_allocated);

        TILES_PER_LEVEL.push(tiles_allocated);
        first_tile_in_level += TILES_PER_LEVEL[octree_level as usize] as i32;
        first_free_tile += tiles_allocated as i32;
    }

    // helpers::show_values_per_tile(0, 8);

    let all_tiles_allocated: u32 = TILES_PER_LEVEL.iter().sum();

    allocate_bricks_pass.run(all_tiles_allocated);

    // TODO: Mipmap to inner nodes

    let values = voxelization::helpers::get_values_from_texture_buffer(
        OCTREE_NODE_POOL_BRICK_POINTERS_TEXTURE_BUFFER,
        max_node_pool_size,
    );
    dbg!(&values[..20]);
}
