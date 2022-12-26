use c_str_macro::c_str;
use cgmath::Matrix4;

use super::common::OCTREE_NODE_POOL;
use crate::config::CONFIG;
use crate::rendering::shader::Shader;
use crate::voxelization::helpers;
use crate::voxelization::octree::common::{
    BRICK_POOL_COLORS_TEXTURE, OCTREE_NODE_POOL_BRICK_POINTERS,
};

pub unsafe fn render_octree(
    model: &Matrix4<f32>,
    view: &Matrix4<f32>,
    projection: &Matrix4<f32>,
    octree_level: i32,
    show_empty_nodes: bool,
) {
    let visualize_octree_shader = Shader::with_geometry_shader(
        "assets/shaders/octree/visualize.vert.glsl",
        "assets/shaders/octree/visualize.frag.glsl",
        "assets/shaders/octree/visualize.geom.glsl",
    );

    visualize_octree_shader.use_program();

    gl::BindImageTexture(
        0,
        OCTREE_NODE_POOL.0,
        0,
        gl::TRUE,
        0,
        gl::READ_WRITE,
        gl::R32UI,
    );

    helpers::bind_image_texture(
        1,
        OCTREE_NODE_POOL_BRICK_POINTERS.0,
        gl::READ_WRITE,
        gl::R32UI,
    );

    helpers::bind_image_texture(2, BRICK_POOL_COLORS_TEXTURE, gl::READ_WRITE, gl::RGBA8);

    visualize_octree_shader.set_int(c_str!("octree_levels"), octree_level);
    visualize_octree_shader.set_int(c_str!("voxel_dimension"), CONFIG.voxel_dimension);
    visualize_octree_shader.set_bool(c_str!("show_empty_nodes"), show_empty_nodes);

    visualize_octree_shader.set_mat4(c_str!("projection"), projection);
    visualize_octree_shader.set_mat4(c_str!("view"), view);
    visualize_octree_shader.set_mat4(c_str!("model"), model);

    visualize_octree_shader.set_float(
        c_str!("half_dimension"),
        1.0 / CONFIG.voxel_dimension as f32,
    );

    let mut vao = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    if octree_level == 8 {
        // TODO: make this work, having octree_levels uniform at 8 is what makes it fail so console
        // log the shader errors, it seems to be failing silently
        for offset in 0..8 {
            visualize_octree_shader.set_int(c_str!("offset"), 1);
            visualize_octree_shader.set_int(c_str!("draw_by_parts"), offset);
            gl::DrawArrays(gl::POINTS, 0, 8u32.pow(7 as u32) as i32);
        }
    } else {
        visualize_octree_shader.set_int(c_str!("offset"), 0);
        visualize_octree_shader.set_int(c_str!("draw_by_parts"), 0);
        gl::DrawArrays(gl::POINTS, 0, 8u32.pow(octree_level as u32) as i32);
    }

    // helpers::show_values_per_tile(0, 2);
}
