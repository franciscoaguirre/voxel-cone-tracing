use std::mem::size_of;

use gl::types::*;

use super::helpers;
use crate::constants;

static mut OCTREE_NODE_POOL_TEXTURE: GLuint = 0;
static mut OCTREE_NODE_POOL_TEXTURE_BUFFER: GLuint = 0;

pub unsafe fn build_octree() {
    let mut atomic_counter: u32 = 0;
    let _error: GLenum = gl::GetError();
    helpers::generate_atomic_counter_buffer(&mut atomic_counter);

    let number_of_tiles = (0..constants::OCTREE_LEVELS - 1)
        .map(|exponent| 8u32.pow(exponent))
        .sum::<u32>();
    let node_pool_size = number_of_tiles * 8;
    helpers::generate_linear_buffer(
        size_of::<GLuint>() * node_pool_size as usize,
        gl::RGBA8,
        &mut OCTREE_NODE_POOL_TEXTURE,
        &mut OCTREE_NODE_POOL_TEXTURE_BUFFER,
    );

    // Initialize root tile
    // Subdivide nodes until OCTREE_LEVELS (compute shader)
    // Mipmap to inner nodes
}
