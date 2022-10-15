use gl::types::*;
use std::{ffi::c_void, mem::size_of, ptr};

pub unsafe fn generate_atomic_counter_buffer() -> u32 {
    let mut buffer: u32 = 0;
    let initial_value: u32 = 0;

    if buffer != 0 {
        gl::DeleteBuffers(1, &buffer);
    }

    gl::GenBuffers(1, &mut buffer);
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, buffer);
    gl::BufferData(
        gl::ATOMIC_COUNTER_BUFFER,
        size_of::<GLuint>() as isize,
        initial_value as *const c_void,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);

    let _error: GLenum = gl::GetError();

    buffer
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

pub fn get_constant_pointer(number: &u32) -> *const c_void {
    number as *const u32 as *const c_void
}

pub fn get_mutable_pointer(number: &mut u32) -> *mut c_void {
    number as *mut u32 as *mut c_void
}

pub unsafe fn get_value_from_atomic_counter(counter: u32) -> GLuint {
    let mut value: GLuint = 0;
    let reset: GLuint = 0;
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, counter);
    gl::GetBufferSubData(
        gl::ATOMIC_COUNTER_BUFFER,
        0,
        size_of::<GLuint>() as isize,
        get_mutable_pointer(&mut value),
    );
    gl::BufferSubData(
        gl::ATOMIC_COUNTER_BUFFER,
        0,
        size_of::<GLuint>() as isize,
        get_constant_pointer(&reset),
    );
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);

    value
}

pub unsafe fn get_values_from_texture_buffer(texture_buffer: GLuint, size: usize) -> Vec<u32> {
    let values = vec![1u32; size];
    gl::BindBuffer(gl::TEXTURE_BUFFER, texture_buffer);
    gl::GetBufferSubData(
        gl::TEXTURE_BUFFER,
        0,
        (size_of::<GLuint>() * size) as isize,
        values.as_ptr() as *mut c_void,
    );
    values
}

pub unsafe fn clear_texture_buffer(texture_buffer: GLuint, size: usize) {
    gl::BindBuffer(gl::TEXTURE_BUFFER, texture_buffer);
    let data = vec![0u32; size];
    gl::BufferData(
        gl::TEXTURE_BUFFER,
        (size_of::<GLuint>() * size) as isize,
        data.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::TEXTURE_BUFFER, 0);
}
