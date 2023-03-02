use std::{ffi::c_void, mem::size_of};

use gl::types::*;
use log::debug;

use super::common::OCTREE_NODE_POOL;
use crate::{config::CONFIG, constants};

pub unsafe fn show_nodes(offset: usize, number_of_nodes: usize) {
    let max_node_pool_size = get_max_node_pool_size();

    let values = vec![1u32; max_node_pool_size];
    gl::BindBuffer(gl::TEXTURE_BUFFER, OCTREE_NODE_POOL.1);
    gl::GetBufferSubData(
        gl::TEXTURE_BUFFER,
        0,
        (size_of::<GLuint>() * max_node_pool_size) as isize,
        values.as_ptr() as *mut c_void,
    );

    for node in 0..number_of_nodes {
        let lower_limit: usize = (node + offset) * constants::CHILDREN_PER_NODE as usize;
        let upper_limit: usize = lower_limit + constants::CHILDREN_PER_NODE as usize;
        debug!("{:?}", &values[lower_limit..upper_limit]);
    }
}

pub fn get_max_node_pool_size() -> usize {
    let number_of_tiles = (0..CONFIG.octree_levels)
        .map(|exponent| (constants::CHILDREN_PER_NODE as usize).pow(exponent))
        .sum::<usize>();
    number_of_tiles * 8
}
