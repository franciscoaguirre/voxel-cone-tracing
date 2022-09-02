use std::{ffi::c_void, mem::size_of};

use c_str_macro::c_str;
use gl::types::*;

use super::{compute_shader::ComputeShader, helpers};
use crate::constants;

static mut OCTREE_NODE_POOL_TEXTURE: GLuint = 0;
static mut OCTREE_NODE_POOL_TEXTURE_BUFFER: GLuint = 0;

pub unsafe fn build_octree(voxel_position_texture: GLuint, number_of_voxel_fragments: u32) {
    let mut allocated_tiles_counter: u32 = 0;
    let _error: GLenum = gl::GetError();
    helpers::generate_atomic_counter_buffer(&mut allocated_tiles_counter);

    let nodes_per_level = vec![1u32];

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
    let mut data = vec![0u32; node_pool_size];
    data[0] = 1u32;
    gl::BufferData(
        gl::TEXTURE_BUFFER,
        (size_of::<GLuint>() * node_pool_size) as isize,
        data.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::TEXTURE_BUFFER, 0);

    let flag_nodes_shader = ComputeShader::new("src/shaders/octree/flag_nodes.comp.glsl");
    let allocate_nodes_shader = ComputeShader::new("src/shaders/octree/allocate_nodes.comp.glsl");
    // let initialize_nodes_shader =
    //     ComputeShader::new("src/shaders/octree/initialize_nodes.comp.glsl");

    let starting_node_in_level = 0; // Index of first node in a given octree level
    let starting_free_space = 1; // Index of first free space (unallocated) in the octree

    for octree_level in 1..2 {
        flag_nodes_shader.use_program();

        flag_nodes_shader.set_int(
            c_str!("number_of_voxel_fragments"),
            number_of_voxel_fragments as i32,
        );
        flag_nodes_shader.set_int(c_str!("octree_level"), octree_level);
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

        // TODO: Subdivide nodes until OCTREE_LEVELS (compute shader)

        allocate_nodes_shader.use_program();

        allocate_nodes_shader.set_int(c_str!("starting_node_in_level"), starting_node_in_level);
        allocate_nodes_shader.set_int(c_str!("starting_free_space"), starting_free_space);
        gl::BindImageTexture(
            0,
            OCTREE_NODE_POOL_TEXTURE,
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::R32UI,
        );
        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, allocated_tiles_counter);

        allocate_nodes_shader.dispatch();
        allocate_nodes_shader.wait();

        let mut tiles_allocated: GLuint = 0;
        let reset: GLuint = 0;
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, allocated_tiles_counter);
        gl::GetBufferSubData(
            gl::ATOMIC_COUNTER_BUFFER,
            0,
            size_of::<GLuint>() as isize,
            get_mutable_pointer(&mut tiles_allocated),
        );
        gl::BufferSubData(
            gl::ATOMIC_COUNTER_BUFFER,
            0,
            size_of::<GLuint>() as isize,
            get_constant_pointer(&reset),
        );
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);

        dbg!(&tiles_allocated);

        let _nodes_allocated = tiles_allocated * 8;
    }

    let values = vec![1u32; node_pool_size];
    gl::BindBuffer(gl::TEXTURE_BUFFER, OCTREE_NODE_POOL_TEXTURE_BUFFER);
    gl::GetBufferSubData(
        gl::TEXTURE_BUFFER,
        0,
        (size_of::<GLuint>() * node_pool_size) as isize,
        values.as_ptr() as *mut c_void,
    );

    dbg!(&values[..20]);

    // TODO: Mipmap to inner nodes
}

fn get_constant_pointer(number: &u32) -> *const c_void {
    number as *const u32 as *const c_void
}

fn get_mutable_pointer(number: &mut u32) -> *mut c_void {
    number as *mut u32 as *mut c_void
}
