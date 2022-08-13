use gl::types::*;
use std::{ffi::c_void, mem::size_of, ptr};

pub unsafe fn generate_atomic_counter_buffer(buffer: &mut u32) {
    let initial_value: u32 = 0;

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

pub unsafe fn generate_linear_buffer(
    size: usize,
    format: GLenum,
    texture: *mut GLuint,
    texture_buffer: *mut GLuint,
) -> u32 {
    if *texture_buffer > 0 {
        gl::DeleteBuffers(1, texture_buffer);
    }

    gl::GenBuffers(1, texture_buffer);

    gl::BindBuffer(gl::TEXTURE_BUFFER, *texture_buffer);
    gl::BufferData(
        gl::TEXTURE_BUFFER,
        size as isize,
        ptr::null::<c_void>(),
        gl::STATIC_DRAW,
    );

    let _error = gl::GetError();

    if *texture > 0 {
        gl::DeleteTextures(1, texture);
    }

    gl::GenTextures(1, texture);
    gl::BindTexture(gl::TEXTURE_BUFFER, *texture);
    gl::TexBuffer(gl::TEXTURE_BUFFER, format, *texture_buffer);
    gl::BindBuffer(gl::TEXTURE_BUFFER, 0);

    let error = gl::GetError();

    if error > 0 {
        // TODO: Use something like glewGetErrorString
        println!("{error}");
    }

    error
}
