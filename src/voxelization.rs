use std::{ffi::c_void, mem::size_of};

use cgmath::{Point3, Vector3};
use gl::types::*;

const VOXEL_DIMENSION: i32 = 256;

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

fn calculate_voxel_fragment_list_length() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        gl::Viewport(0, 0, VOXEL_DIMENSION, VOXEL_DIMENSION);

        let ortho = cgmath::ortho(-1.0, 1.0, -1.0, 1.0, 2.0 - 1.0, 3.0);
        let model_view_projection_x = ortho
            * cgmath::Matrix4::look_at_rh(
                Point3::new(2.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::unit_y(),
            );
        let model_view_projection_y = ortho
            * cgmath::Matrix4::look_at_rh(
                Point3::new(0.0, 2.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                -Vector3::unit_z(),
            );
        let model_view_projection_z = ortho
            * cgmath::Matrix4::look_at_rh(
                Point3::new(0.0, 0.0, 2.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::unit_y(),
            );

        gl::Disable(gl::CULL_FACE);
        gl::Disable(gl::DEPTH_TEST);
        gl::ColorMask(gl::FALSE, gl::FALSE, gl::FALSE, gl::FALSE);
    }
}

fn build_voxel_fragment_list() {
    let mut atomic_buffer: u32 = 0;

    generate_atomic_counter_buffer(&mut atomic_buffer);

    calculate_voxel_fragment_list_length();
}
