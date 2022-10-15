use std::{ffi::c_void, mem::size_of};

use gl::types::*;

use super::common::OCTREE_NODE_POOL_TEXTURE_BUFFER;
use crate::constants;

pub unsafe fn show_values_per_tile(offset: usize, number_of_tiles: usize) {
    let max_node_pool_size = get_max_node_pool_size();

    let values = vec![1u32; max_node_pool_size];
    gl::BindBuffer(gl::TEXTURE_BUFFER, OCTREE_NODE_POOL_TEXTURE_BUFFER);
    gl::GetBufferSubData(
        gl::TEXTURE_BUFFER,
        0,
        (size_of::<GLuint>() * max_node_pool_size) as isize,
        values.as_ptr() as *mut c_void,
    );

    for tile in 0..number_of_tiles {
        let lower_limit: usize = (tile + offset) * constants::NODES_PER_TILE as usize;
        let upper_limit: usize = lower_limit + constants::NODES_PER_TILE as usize;
        dbg!(&values[lower_limit..upper_limit]);
    }
}

pub fn get_max_node_pool_size() -> usize {
    let number_of_tiles = (0..constants::OCTREE_LEVELS)
        .map(|exponent| (constants::NODES_PER_TILE as usize).pow(exponent))
        .sum::<usize>();
    number_of_tiles * 8
}
