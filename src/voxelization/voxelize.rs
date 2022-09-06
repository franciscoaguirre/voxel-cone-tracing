use std::{ffi::c_void, mem::size_of};

use super::helpers;
use crate::{constants, rendering::model::Model, rendering::shader::Shader};
use c_str_macro::c_str;

use gl::types::*;

static mut VOXEL_POSITION_TEXTURE: GLuint = 0;
static mut VOXEL_POSITION_TEXTURE_BUFFER: GLuint = 0;
static mut VOXEL_DIFFUSE_TEXTURE: GLuint = 0;
static mut VOXEL_DIFFUSE_TEXTURE_BUFFER: GLuint = 0;
// static mut VOXEL_NORMAL_TEXTURE: GLuint = 0;
// static mut VOXEL_NORMAL_TEXTURE_BUFFER: GLuint = 0;

unsafe fn calculate_voxel_fragment_list_length(
    voxelization_shader: &Shader,
    models: &[Model; 1],
    atomic_counter: &mut u32,
) {
    voxelization_shader.useProgram();
    voxelization_shader.setBool(c_str!("should_store"), false);
    voxelize_scene(voxelization_shader, models, atomic_counter);
}

unsafe fn populate_voxel_fragment_list(
    voxelization_shader: &Shader,
    models: &[Model; 1],
    atomic_counter: &mut u32,
) {
    voxelization_shader.useProgram();
    voxelization_shader.setBool(c_str!("should_store"), true);

    gl::BindImageTexture(
        0,
        VOXEL_POSITION_TEXTURE,
        0,
        gl::FALSE,
        0,
        gl::READ_WRITE,
        gl::RGB10_A2UI,
    );
    gl::BindImageTexture(
        1,
        VOXEL_DIFFUSE_TEXTURE,
        0,
        gl::FALSE,
        0,
        gl::READ_WRITE,
        gl::RGBA8,
    );
    voxelization_shader.setInt(c_str!("u_voxelPos"), 0);

    voxelize_scene(voxelization_shader, models, atomic_counter);
}

unsafe fn voxelize_scene(
    voxelization_shader: &Shader,
    models: &[Model; 1], // TODO: More than one?
    atomic_counter: &mut u32,
) {
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    gl::Viewport(0, 0, constants::VOXEL_DIMENSION, constants::VOXEL_DIMENSION);

    // TODO: This should be the aabb of the entire scene
    let scene_aabb = &models[0].aabb;
    let aabb_middle_point = scene_aabb.middle_point();
    let aabb_longer_side = scene_aabb.longer_axis_length();

    let center_scene_matrix = cgmath::Matrix4::from_translation(-aabb_middle_point);
    // aabb_longer_side is divided by two and we then use the inverse because
    // NDC coordinates goes from -1 to 1
    let normalize_size_matrix = cgmath::Matrix4::from_scale(2f32 / aabb_longer_side);

    let model_normalization_matrix = normalize_size_matrix * center_scene_matrix;

    voxelization_shader.setMat4(
        c_str!("model_normalization_matrix"),
        &model_normalization_matrix,
    );
    voxelization_shader.setInt(c_str!("voxel_dimension"), constants::VOXEL_DIMENSION);

    gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, *atomic_counter);

    voxelization_shader.setVec3(c_str!("fallback_color"), 1.0, 1.0, 1.0);
    for model in models {
        // TODO: Do we need to set more things in the shader?
        model.Draw(voxelization_shader);
    }

    gl::Disable(gl::CULL_FACE);
    gl::Disable(gl::DEPTH_TEST);
    gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
    gl::Viewport(0, 0, constants::SOURCE_WIDTH, constants::SOURCE_HEIGHT);
}

pub unsafe fn build_voxel_fragment_list() -> (u32, u32, u32) {
    let mut atomic_counter: u32 = 0;
    let _error: GLenum = gl::GetError();
    helpers::generate_atomic_counter_buffer(&mut atomic_counter);

    let (voxelization_shader, cow_model) = {
        gl::Enable(gl::DEPTH_TEST);

        let our_shader = Shader::with_geometry_shader(
            "src/shaders/voxel_fragment/voxelize.vert.glsl",
            "src/shaders/voxel_fragment/voxelize.frag.glsl",
            "src/shaders/voxel_fragment/voxelize.geom.glsl",
        );

        let our_model = Model::new("assets/colored_cow.obj");

        (our_shader, our_model)
    };
    let models = [cow_model];

    calculate_voxel_fragment_list_length(&voxelization_shader, &models, &mut atomic_counter);
    gl::MemoryBarrier(gl::ATOMIC_COUNTER_BUFFER);

    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, atomic_counter);
    let count = gl::MapBufferRange(
        gl::ATOMIC_COUNTER_BUFFER,
        0,
        size_of::<GLuint>() as isize,
        gl::MAP_READ_BIT | gl::MAP_WRITE_BIT,
    ) as *mut GLuint;

    let _error = gl::GetError();
    // TODO: How to show these errors?

    let number_of_voxel_fragments = *count;

    helpers::generate_linear_buffer(
        size_of::<GLuint>() * number_of_voxel_fragments as usize,
        gl::R32UI,
        &mut VOXEL_POSITION_TEXTURE,
        &mut VOXEL_POSITION_TEXTURE_BUFFER,
    );
    helpers::generate_linear_buffer(
        size_of::<GLuint>() * number_of_voxel_fragments as usize,
        gl::RGBA8,
        &mut VOXEL_DIFFUSE_TEXTURE,
        &mut VOXEL_DIFFUSE_TEXTURE_BUFFER,
    );

    *count = 0;

    gl::UnmapBuffer(gl::ATOMIC_COUNTER_BUFFER);
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);

    populate_voxel_fragment_list(&voxelization_shader, &models, &mut atomic_counter);

    let values = vec![1u32; number_of_voxel_fragments as usize];
    gl::BindBuffer(gl::TEXTURE_BUFFER, VOXEL_POSITION_TEXTURE_BUFFER);
    gl::GetBufferSubData(
        gl::TEXTURE_BUFFER,
        0,
        (size_of::<GLuint>() * number_of_voxel_fragments as usize) as isize,
        values.as_ptr() as *mut c_void,
    );

    dbg!(&values);

    gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);

    (
        number_of_voxel_fragments,
        VOXEL_POSITION_TEXTURE,
        VOXEL_DIFFUSE_TEXTURE,
    )
}
