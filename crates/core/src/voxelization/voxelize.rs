use std::mem::size_of;

use c_str_macro::c_str;
use cgmath::{point3, Matrix4, SquareMatrix};
use gl::types::*;
use engine::prelude::*;

use crate::config::Config;

unsafe fn calculate_voxel_fragment_list_length(
    voxelization_shader: &Shader,
    objects: &mut [Object],
    scene_aabb: &Aabb,
    atomic_counter: &mut u32,
) {
    voxelization_shader.use_program();
    voxelization_shader.set_bool(c_str!("shouldStore"), false);
    voxelization_shader.set_bool(c_str!("hasBump"), false);
    voxelize_scene(voxelization_shader, objects, scene_aabb, atomic_counter);
}

unsafe fn populate_voxel_fragment_list(
    voxelization_shader: &Shader,
    objects: &mut [Object],
    scene_aabb: &Aabb,
    atomic_counter: &mut u32,
    voxel_positions: BufferTexture,
    voxel_colors: BufferTexture,
    voxel_normals: BufferTexture,
) {
    voxelization_shader.use_program();
    voxelization_shader.set_bool(c_str!("shouldStore"), true);
    voxelization_shader.set_bool(c_str!("hasBump"), false);

    helpers::bind_image_texture(0, voxel_positions.0, gl::WRITE_ONLY, gl::RGB10_A2UI);
    helpers::bind_image_texture(1, voxel_colors.0, gl::WRITE_ONLY, gl::RGBA8);
    helpers::bind_image_texture(2, voxel_normals.0, gl::WRITE_ONLY, gl::RGBA32F);

    voxelize_scene(voxelization_shader, objects, scene_aabb, atomic_counter);
}

unsafe fn voxelize_scene(
    voxelization_shader: &Shader,
    objects: &mut [Object],
    scene_aabb: &Aabb,
    atomic_counter: &mut u32,
) {
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    let config = Config::instance();

    gl::Viewport(
        0,
        0,
        config.voxel_dimension() as i32,
        config.voxel_dimension() as i32,
    );

    let model_normalization_matrix = scene_aabb.normalization_matrix();

    // Same for every object
    voxelization_shader.set_int(c_str!("voxelDimension"), config.voxel_dimension() as i32);

    gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, *atomic_counter);

    voxelization_shader.set_vec3(c_str!("fallbackColor"), 1.0, 1.0, 1.0);

    gl::Disable(gl::CULL_FACE);
    gl::Disable(gl::DEPTH_TEST);
    // TODO: We should apparently disable depth test and colormask false flase flase
    for object in objects.iter_mut() {
        object.draw(voxelization_shader, &model_normalization_matrix);
    }

    let (viewport_width, viewport_height) = config.viewport_dimensions();

    gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
    gl::Viewport(
        0,
        0,
        viewport_width,
        viewport_height,
    );
}

pub unsafe fn build_voxel_fragment_list(
    objects: &mut [Object],
    scene_aabb: &Aabb,
) -> (BufferTextureV2<u32>, u32, BufferTexture, BufferTexture) {
    let mut atomic_counter: u32 = helpers::generate_atomic_counter_buffer();

    let voxelization_shader = compile_shaders!(
        "assets/shaders/voxel_fragment/voxelize.vert.glsl",
        "assets/shaders/voxel_fragment/voxelize.frag.glsl",
        "assets/shaders/voxel_fragment/voxelize.geom.glsl",
    );

    calculate_voxel_fragment_list_length(&voxelization_shader, objects, scene_aabb, &mut atomic_counter);
    gl::MemoryBarrier(gl::ATOMIC_COUNTER_BUFFER);

    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, atomic_counter);
    let count = gl::MapBufferRange(
        gl::ATOMIC_COUNTER_BUFFER,
        0,
        size_of::<GLuint>() as isize,
        gl::MAP_READ_BIT | gl::MAP_WRITE_BIT,
    ) as *mut GLuint;

    let number_of_voxel_fragments = *count;

    let voxel_positions: (GLuint, GLuint) = helpers::generate_texture_buffer(
        size_of::<GLuint>() * number_of_voxel_fragments as usize,
        gl::R32UI,
        0u32,
    );
    let voxel_colors: (GLuint, GLuint) = helpers::generate_texture_buffer(
        size_of::<GLuint>() * number_of_voxel_fragments as usize,
        gl::RGBA8,
        0u32,
    );
    let voxel_normals: (GLuint, GLuint) = helpers::generate_texture_buffer(
        size_of::<GLuint>() * number_of_voxel_fragments as usize,
        gl::RGBA32F,
        0u32,
    );

    *count = 0;

    gl::UnmapBuffer(gl::ATOMIC_COUNTER_BUFFER);
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);

    populate_voxel_fragment_list(
        &voxelization_shader,
        objects,
        scene_aabb,
        &mut atomic_counter,
        voxel_positions,
        voxel_colors,
        voxel_normals,
    );

    gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);

    (
        BufferTextureV2::from_texture_and_buffer(voxel_positions),
        number_of_voxel_fragments,
        voxel_colors,
        voxel_normals,
    )
}
