use std::{ffi::c_void, mem::size_of};

use crate::{constants::NODES_PER_TILE, rendering::shader::Shader};
use c_str_macro::c_str;
use cgmath::Matrix4;
use gl::types::*;

use super::{compute_shader::ComputeShader, helpers};
use crate::constants;

static mut OCTREE_NODE_POOL_TEXTURE: GLuint = 0;
static mut OCTREE_NODE_POOL_TEXTURE_BUFFER: GLuint = 0;

fn get_max_node_pool_size() -> usize {
    let number_of_tiles = (0..constants::OCTREE_LEVELS)
        .map(|exponent| (constants::NODES_PER_TILE as usize).pow(exponent))
        .sum::<usize>();
    number_of_tiles * 8
}

unsafe fn show_values_per_tile(offset: usize, number_of_tiles: usize) {
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

static mut TILES_PER_LEVEL: Vec<u32> = Vec::new();

pub unsafe fn build_octree(voxel_position_texture: GLuint, number_of_voxel_fragments: u32) {
    let mut allocated_tiles_counter: u32 = 0;
    let _error: GLenum = gl::GetError();
    helpers::generate_atomic_counter_buffer(&mut allocated_tiles_counter);

    let max_node_pool_size = get_max_node_pool_size();
    TILES_PER_LEVEL.push(1);

    helpers::generate_linear_buffer(
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

    let flag_nodes_shader = ComputeShader::new("src/shaders/octree/flag_nodes.comp.glsl");
    let allocate_nodes_shader = ComputeShader::new("src/shaders/octree/allocate_nodes.comp.glsl");

    let mut first_tile_in_level: i32 = 0; // Index of first tile in a given octree level
    let mut first_free_tile: i32 = 1; // Index of first free tile (unallocated) in the octree

    for octree_level in 0..constants::OCTREE_LEVELS {
        flag_nodes_shader.use_program();

        flag_nodes_shader.set_int(
            c_str!("number_of_voxel_fragments"),
            number_of_voxel_fragments as i32,
        );
        flag_nodes_shader.set_int(c_str!("octree_level"), octree_level as i32);
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
            get_mutable_pointer(&mut tiles_allocated),
        );
        gl::BufferSubData(
            gl::ATOMIC_COUNTER_BUFFER,
            0,
            size_of::<GLuint>() as isize,
            get_constant_pointer(&reset),
        );
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);

        dbg!(tiles_allocated);

        TILES_PER_LEVEL.push(tiles_allocated);
        first_tile_in_level += TILES_PER_LEVEL[octree_level as usize] as i32;
        first_free_tile += tiles_allocated as i32;
    }

    show_values_per_tile(0, 5);
    // TODO: Mipmap to inner nodes
}

pub unsafe fn render_octree(
    model: &Matrix4<f32>,
    view: &Matrix4<f32>,
    projection: &Matrix4<f32>,
    octree_level: i32,
) {
    let visualize_octree_shader = Shader::with_geometry_shader(
        "src/shaders/octree/visualize.vert.glsl",
        "src/shaders/octree/visualize.frag.glsl",
        "src/shaders/octree/visualize.geom.glsl",
    );

    visualize_octree_shader.useProgram();

    gl::BindImageTexture(
        0,
        OCTREE_NODE_POOL_TEXTURE,
        0,
        gl::TRUE,
        0,
        gl::READ_WRITE,
        gl::R32UI,
    );

    visualize_octree_shader.setInt(c_str!("octree_levels"), octree_level);
    visualize_octree_shader.setInt(c_str!("voxel_dimension"), constants::VOXEL_DIMENSION);

    visualize_octree_shader.setMat4(c_str!("projection"), projection);
    visualize_octree_shader.setMat4(c_str!("view"), view);
    visualize_octree_shader.setMat4(c_str!("model"), model);

    visualize_octree_shader.setFloat(
        c_str!("half_dimension"),
        1.0 / constants::VOXEL_DIMENSION as f32,
    );

    let mut vao = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    gl::DrawArrays(gl::POINTS, 0, 8u32.pow(octree_level as u32) as i32);
}

fn get_constant_pointer(number: &u32) -> *const c_void {
    number as *const u32 as *const c_void
}

fn get_mutable_pointer(number: &mut u32) -> *mut c_void {
    number as *mut u32 as *mut c_void
}
