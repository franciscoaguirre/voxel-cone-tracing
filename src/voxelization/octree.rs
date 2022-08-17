use std::{ffi::c_void, mem::size_of};

use c_str_macro::c_str;
use gl::types::*;

use super::{compute_shader::ComputeShader, helpers};
use crate::constants;

static mut OCTREE_NODE_POOL_TEXTURE: GLuint = 0;
static mut OCTREE_NODE_POOL_TEXTURE_BUFFER: GLuint = 0;

pub unsafe fn build_octree(voxel_position_texture: GLuint, number_of_voxel_fragments: u32) {
    let mut atomic_counter: u32 = 0;
    let _error: GLenum = gl::GetError();
    helpers::generate_atomic_counter_buffer(&mut atomic_counter);

    let number_of_tiles = (0..constants::OCTREE_LEVELS - 1)
        .map(|exponent| 8usize.pow(exponent))
        .sum::<usize>();
    let node_pool_size = number_of_tiles * 8;
    helpers::generate_linear_buffer(
        size_of::<GLuint>() * node_pool_size as usize,
        gl::R32UI,
        &mut OCTREE_NODE_POOL_TEXTURE,
        &mut OCTREE_NODE_POOL_TEXTURE_BUFFER,
    );

    gl::BindBuffer(gl::TEXTURE_BUFFER, OCTREE_NODE_POOL_TEXTURE_BUFFER);
    let data = vec![0u32; node_pool_size];
    gl::BufferData(
        gl::TEXTURE_BUFFER,
        (size_of::<GLuint>() * node_pool_size) as isize,
        data.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::TEXTURE_BUFFER, 0);

    let flag_nodes_shader = ComputeShader::new("src/shaders/octree/flag_nodes.comp.glsl");

    flag_nodes_shader.use_program();

    flag_nodes_shader.set_int(
        c_str!("number_of_voxel_fragments"),
        number_of_voxel_fragments as i32,
    );
    flag_nodes_shader.set_int(c_str!("octree_level"), 1);
    flag_nodes_shader.set_int(c_str!("voxel_dimension"), constants::VOXEL_DIMENSION);

    gl::BindImageTexture(
        0,
        voxel_position_texture,
        0,
        gl::FALSE,
        0,
        gl::READ_ONLY,
        gl::RGB10_A2UI,
    );
    gl::BindImageTexture(
        1,
        OCTREE_NODE_POOL_TEXTURE,
        0,
        gl::FALSE,
        0,
        gl::WRITE_ONLY,
        gl::R32UI,
    );

    flag_nodes_shader.dispatch();
    flag_nodes_shader.wait();

    let values = vec![1u32; node_pool_size];
    gl::BindBuffer(gl::TEXTURE_BUFFER, OCTREE_NODE_POOL_TEXTURE_BUFFER);
    gl::GetBufferSubData(
        gl::TEXTURE_BUFFER,
        0,
        (size_of::<GLuint>() * node_pool_size) as isize,
        values.as_ptr() as *mut c_void,
    );
    dbg!(&values[node_pool_size - 20..]);

    // Initialize root tile
    // Subdivide nodes until OCTREE_LEVELS (compute shader)
    // Mipmap to inner nodes
}
