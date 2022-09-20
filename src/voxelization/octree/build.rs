use c_str_macro::c_str;
use gl::types::*;
use std::{ffi::c_void, mem::size_of};

use super::{
    common::{OCTREE_NODE_POOL_TEXTURE, OCTREE_NODE_POOL_TEXTURE_BUFFER, TILES_PER_LEVEL},
    helpers,
};
use crate::rendering::shader::Shader;
use crate::{
    constants::{OCTREE_LEVELS, VOXEL_DIMENSION},
    voxelization,
};

pub unsafe fn build_octree(voxel_position_texture: GLuint, number_of_voxel_fragments: u32) {
    let mut allocated_tiles_counter: u32 = 0;
    let _error: GLenum = gl::GetError();
    voxelization::helpers::generate_atomic_counter_buffer(&mut allocated_tiles_counter);

    let max_node_pool_size = helpers::get_max_node_pool_size();
    TILES_PER_LEVEL.push(1);

    voxelization::helpers::generate_linear_buffer(
        size_of::<GLuint>() * max_node_pool_size as usize,
        gl::R32UI,
        &mut OCTREE_NODE_POOL_TEXTURE,
        &mut OCTREE_NODE_POOL_TEXTURE_BUFFER,
    );

    gl::BindBuffer(gl::TEXTURE_BUFFER, OCTREE_NODE_POOL_TEXTURE_BUFFER);
    let data = vec![0u32; max_node_pool_size];
    gl::BufferData(
        gl::TEXTURE_BUFFER,
        (size_of::<GLuint>() * max_node_pool_size) as isize,
        data.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::TEXTURE_BUFFER, 0);

    let flag_nodes_shader = Shader::new_compute("assets/shaders/octree/flag_nodes.comp.glsl");
    let allocate_nodes_shader =
        Shader::new_compute("assets/shaders/octree/allocate_nodes.comp.glsl");

    let mut first_tile_in_level: i32 = 0; // Index of first tile in a given octree level
    let mut first_free_tile: i32 = 1; // Index of first free tile (unallocated) in the octree

    for octree_level in 0..OCTREE_LEVELS {
        flag_nodes_shader.use_program();

        flag_nodes_shader.set_int(
            c_str!("number_of_voxel_fragments"),
            number_of_voxel_fragments as i32,
        );
        flag_nodes_shader.set_int(c_str!("octree_level"), octree_level as i32);
        flag_nodes_shader.set_int(c_str!("voxel_dimension"), VOXEL_DIMENSION);

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

        flag_nodes_shader.dispatch(65_535); // TODO: Calculate number of groups
        flag_nodes_shader.wait();

        allocate_nodes_shader.use_program();

        allocate_nodes_shader.set_int(c_str!("first_tile_in_level"), first_tile_in_level);
        allocate_nodes_shader.set_int(c_str!("first_free_tile"), first_free_tile);
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

        allocate_nodes_shader.dispatch(65_535); // TODO: Calculate number of groups
        allocate_nodes_shader.wait();

        let mut tiles_allocated: GLuint = 0;
        let reset: GLuint = 0;
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, allocated_tiles_counter);
        gl::GetBufferSubData(
            gl::ATOMIC_COUNTER_BUFFER,
            0,
            size_of::<GLuint>() as isize,
            helpers::get_mutable_pointer(&mut tiles_allocated),
        );
        gl::BufferSubData(
            gl::ATOMIC_COUNTER_BUFFER,
            0,
            size_of::<GLuint>() as isize,
            helpers::get_constant_pointer(&reset),
        );
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);

        dbg!(tiles_allocated);

        TILES_PER_LEVEL.push(tiles_allocated);
        first_tile_in_level += TILES_PER_LEVEL[octree_level as usize] as i32;
        first_free_tile += tiles_allocated as i32;
    }

    helpers::show_values_per_tile(0, 8);
    // TODO: Mipmap to inner nodes
}
