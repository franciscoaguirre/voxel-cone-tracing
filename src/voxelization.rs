use std::{ffi::c_void, mem::size_of};

use crate::{camera::Camera, constants, Model, Shader};
use c_str_macro::c_str;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3, Vector3};
use gl::types::*;

fn generate_atomic_counter_buffer(buffer: &mut u32) {
    let initial_value: u32 = 0;

    unsafe {
        if *buffer != 0 {
            gl::DeleteBuffers(1, buffer);
        }

        gl::GenBuffers(1, buffer);
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, *buffer);
        gl::BufferData(
            gl::ATOMIC_COUNTER_BUFFER,
            size_of::<GLuint>() as isize,
            initial_value as *const c_void,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);
    }
}

unsafe fn calculate_voxel_fragment_list_length(voxelization_shader: &Shader, models: &[Model; 1]) {
    voxelization_shader.useProgram();
    voxelization_shader.setBool(c_str!("should_store"), false);
    voxelize_scene(voxelization_shader, models);
}

unsafe fn populate_voxel_fragment_list(voxelization_shader: &Shader, models: &[Model; 1]) {
    voxelization_shader.useProgram();
    voxelization_shader.setBool(c_str!("should_store"), true);
    voxelize_scene(voxelization_shader, models);
}

unsafe fn voxelize_scene(voxelization_shader: &Shader, models: &[Model; 1]) {
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    gl::Viewport(0, 0, constants::VOXEL_DIMENSION, constants::VOXEL_DIMENSION);

    let origin_point = Point3::new(0.0, 0.0, 0.0);
    let ortho = cgmath::ortho(-1.0, 1.0, -1.0, 1.0, 2.0 - 1.0, 3.0);
    let x_ortho_projection = ortho
        * cgmath::Matrix4::look_at_rh(Point3::new(2.0, 0.0, 0.0), origin_point, Vector3::unit_y());
    let y_ortho_projection = ortho
        * cgmath::Matrix4::look_at_rh(Point3::new(0.0, 2.0, 0.0), origin_point, -Vector3::unit_z());
    let z_ortho_projection = ortho
        * cgmath::Matrix4::look_at_rh(Point3::new(0.0, 0.0, 2.0), origin_point, Vector3::unit_y());

    voxelization_shader.setMat4(c_str!("x_ortho_projection"), &x_ortho_projection);
    voxelization_shader.setMat4(c_str!("y_ortho_projection"), &y_ortho_projection);
    voxelization_shader.setMat4(c_str!("z_ortho_projection"), &z_ortho_projection);
    voxelization_shader.setInt(c_str!("voxel_dimension"), constants::VOXEL_DIMENSION);

    for model in models {
        // TODO: Do we need to set more things in the shader?
        model.Draw(&voxelization_shader);
    }

    gl::Disable(gl::CULL_FACE);
    gl::Disable(gl::DEPTH_TEST);
    gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
    gl::Viewport(0, 0, constants::SOURCE_WIDTH, constants::SOURCE_HEIGHT);
}

pub unsafe fn build_voxel_fragment_list() {
    let mut atomic_buffer: u32 = 0;
    let error: GLenum = gl::GetError();
    generate_atomic_counter_buffer(&mut atomic_buffer);

    let (voxelization_shader, cow_model) = {
        gl::Enable(gl::DEPTH_TEST);

        let our_shader = Shader::with_geometry_shader(
            "src/shaders/voxelize.vert.glsl",
            "src/shaders/voxelize.frag.glsl",
            "src/shaders/voxelize.geom.glsl",
        );

        let our_model = Model::new("assets/cow.obj");

        (our_shader, our_model)
    };
    let models = [cow_model];

    calculate_voxel_fragment_list_length(&voxelization_shader, &models);
    gl::MemoryBarrier(gl::ATOMIC_COUNTER_BUFFER);
    let error: GLenum = gl::GetError();

    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, atomic_buffer);
    let count = gl::MapBufferRange(
        gl::ATOMIC_COUNTER_BUFFER,
        0,
        size_of::<GLuint>() as isize,
        gl::MAP_READ_BIT | gl::MAP_WRITE_BIT,
    );

    let number_of_voxel_fragments = &*count;

    dbg!(number_of_voxel_fragments);
}
