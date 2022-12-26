use c_str_macro::c_str;
use cgmath::Matrix4;
use gl::types::*;

use crate::config::CONFIG;
use crate::rendering::shader::Shader;

pub unsafe fn render_voxel_fragments(
    voxel_positions_texture: GLuint,
    voxel_colors_texture: GLuint,
    projection: &Matrix4<f32>,
    view: &Matrix4<f32>,
    model: &Matrix4<f32>,
    number_of_voxel_fragments: u32,
    vao: u32,
) {
    gl::Enable(gl::DEPTH_TEST);

    // Set shader program
    let render_voxel_shader = Shader::with_geometry_shader(
        "assets/shaders/voxel_fragment/render_voxel.vert.glsl",
        "assets/shaders/voxel_fragment/render_voxel.frag.glsl",
        "assets/shaders/voxel_fragment/render_voxel.geom.glsl",
    );

    render_voxel_shader.use_program();

    gl::BindImageTexture(
        0,
        voxel_positions_texture,
        0,
        gl::TRUE,
        0,
        gl::READ_ONLY,
        gl::RGB10_A2UI,
    );

    gl::BindImageTexture(
        1,
        voxel_colors_texture,
        0,
        gl::TRUE,
        0,
        gl::READ_ONLY,
        gl::RGBA8,
    );

    render_voxel_shader.set_mat4(c_str!("projection"), projection);
    render_voxel_shader.set_mat4(c_str!("view"), view);
    render_voxel_shader.set_mat4(c_str!("model"), model);

    render_voxel_shader.set_int(c_str!("voxel_dimension"), CONFIG.voxel_dimension);
    render_voxel_shader.set_float(
        c_str!("half_dimension"),
        1.0 / CONFIG.voxel_dimension as f32,
    );

    render_voxel_shader.set_int(
        c_str!("voxel_fragment_count"),
        number_of_voxel_fragments as i32,
    );

    gl::BindVertexArray(vao);
    gl::DrawArrays(gl::POINTS, 0, number_of_voxel_fragments as i32);
}
