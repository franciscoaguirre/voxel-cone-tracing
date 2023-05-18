use std::mem::size_of;

use crate::{
    config::CONFIG,
    helpers,
    rendering::{model::Model, shader::Shader, transform::Transform},
    types::BufferTexture,
};
use c_str_macro::c_str;

use cgmath::point3;
use gl::types::*;

unsafe fn calculate_voxel_fragment_list_length(
    voxelization_shader: &Shader,
    models: &[&Model; 1],
    atomic_counter: &mut u32,
) {
    voxelization_shader.use_program();
    voxelization_shader.set_bool(c_str!("shouldStore"), false);
    voxelization_shader.set_bool(c_str!("hasBump"), false);
    voxelize_scene(voxelization_shader, models, atomic_counter);
}

unsafe fn populate_voxel_fragment_list(
    voxelization_shader: &Shader,
    models: &[&Model; 1],
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
    helpers::bind_image_texture(2, voxel_normals.0, gl::WRITE_ONLY, gl::RGBA8);

    voxelize_scene(voxelization_shader, models, atomic_counter);
}

unsafe fn voxelize_scene(
    voxelization_shader: &Shader,
    models: &[&Model; 1], // TODO: More than one?
    atomic_counter: &mut u32,
) {
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    gl::Viewport(
        0,
        0,
        CONFIG.voxel_dimension as i32,
        CONFIG.voxel_dimension as i32,
    );

    // TODO: This should be the aabb of the entire scene
    // We could fix this by preprocessing the `models` array before this function to get
    // one big aabb that spans the whole scene.
    let scene_aabb = &models[0].aabb;
    let model_normalization_matrix = helpers::get_scene_normalization_matrix(scene_aabb);

    voxelization_shader.set_mat4(
        c_str!("modelNormalizationMatrix"),
        &model_normalization_matrix,
    );
    voxelization_shader.set_int(c_str!("voxelDimension"), CONFIG.voxel_dimension as i32);

    gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, *atomic_counter);

    voxelization_shader.set_vec3(c_str!("fallbackColor"), 1.0, 1.0, 1.0);

    let mut right_camera = Transform::default();
    right_camera.position = point3(-2.0, 0.0, 0.0);
    right_camera.set_rotation_y(0.0);
    let right_view_matrix = right_camera.get_view_matrix();

    let mut top_camera = Transform::default();
    top_camera.position = point3(0.0, 2.0, 0.0);
    top_camera.set_rotation_x(-90.0);
    top_camera.set_rotation_y(90.0);
    let top_view_matrix = top_camera.get_view_matrix();

    let mut far_camera = Transform::default();
    far_camera.position = point3(0.0, 0.0, 2.0);
    far_camera.set_rotation_y(-90.0);
    let far_view_matrix = far_camera.get_view_matrix();

    voxelization_shader.set_mat4_array(
        c_str!("axisProjections"),
        &[&right_view_matrix, &top_view_matrix, &far_view_matrix],
    );
    gl::Disable(gl::CULL_FACE);
    gl::Disable(gl::DEPTH_TEST);

    // TODO: We should apparently disable depth test and colormask false flase flase
    for model in models {
        // TODO: Do we need to set more things in the shader?
        model.draw(voxelization_shader);
    }

    gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
    gl::Viewport(
        0,
        0,
        CONFIG.viewport_width as i32,
        CONFIG.viewport_height as i32,
    );
}

pub unsafe fn build_voxel_fragment_list(
    model: &Model,
) -> (BufferTexture, u32, BufferTexture, BufferTexture) {
    let mut atomic_counter: u32 = helpers::generate_atomic_counter_buffer();

    let voxelization_shader = Shader::with_geometry_shader(
        "assets/shaders/voxel_fragment/voxelize.vert.glsl",
        "assets/shaders/voxel_fragment/voxelize.frag.glsl",
        "assets/shaders/voxel_fragment/voxelize.geom.glsl",
    );
    let models = [model];

    calculate_voxel_fragment_list_length(&voxelization_shader, &models, &mut atomic_counter);
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
        gl::RGBA8,
        0u32,
    );

    *count = 0;

    gl::UnmapBuffer(gl::ATOMIC_COUNTER_BUFFER);
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);

    populate_voxel_fragment_list(
        &voxelization_shader,
        &models,
        &mut atomic_counter,
        voxel_positions,
        voxel_colors,
        voxel_normals,
    );

    gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);

    (
        voxel_positions,
        number_of_voxel_fragments,
        voxel_colors,
        voxel_normals,
    )
}
