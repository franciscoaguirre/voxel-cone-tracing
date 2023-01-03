use gl::types::*;
use std::{ffi::c_void, mem::size_of};

pub unsafe fn generate_atomic_counter_buffer() -> GLuint {
    let mut buffer: u32 = 0;
    let initial_value: u32 = 0;

    gl::GenBuffers(1, &mut buffer);
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, buffer);
    gl::BufferData(
        gl::ATOMIC_COUNTER_BUFFER,
        size_of::<GLuint>() as isize,
        initial_value as *const c_void,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);

    buffer
}

pub unsafe fn generate_texture_buffer<T>(
    size: usize,
    format: GLenum,
    default_value: T,
) -> (GLuint, GLuint)
where
    T: Clone,
{
    let mut texture_buffer: GLuint = 0;
    gl::GenBuffers(1, &mut texture_buffer);

    gl::BindBuffer(gl::TEXTURE_BUFFER, texture_buffer);

    let mut texture: GLuint = 0;

    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_BUFFER, texture);
    gl::TexBuffer(gl::TEXTURE_BUFFER, format, texture_buffer);
    gl::BindBuffer(gl::TEXTURE_BUFFER, 0);

    clear_texture_buffer(texture_buffer, size, default_value);

    (texture, texture_buffer)
}

pub unsafe fn generate_3d_texture(size_one_dimension: u32) -> GLuint {
    let mut texture: GLuint = 0;

    // TODO: Apparently powers of two are recommended, but using the next power of
    // two understandably makes this really large really fast.
    // let size_one_dimension = size_one_dimension.next_power_of_two() as i32;

    let size = size_one_dimension.pow(3);

    let initial_data = vec![0u32; size as usize];

    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_3D, texture);
    gl::TexImage3D(
        gl::TEXTURE_3D,
        0,
        gl::RGBA8 as i32,
        size_one_dimension as i32,
        size_one_dimension as i32,
        size_one_dimension as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        initial_data.as_ptr() as *const c_void,
    );
    gl::BindTexture(gl::TEXTURE_3D, 0);

    texture
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

pub unsafe fn get_values_from_texture_buffer<T>(
    texture_buffer: GLuint,
    size: usize,
    default_value: T,
) -> Vec<T>
where
    T: Clone,
{
    let values = vec![default_value; size];
    gl::BindBuffer(gl::TEXTURE_BUFFER, texture_buffer);
    gl::GetBufferSubData(
        gl::TEXTURE_BUFFER,
        0,
        (size_of::<T>() * size) as isize,
        values.as_ptr() as *mut c_void,
    );

    values
}

pub unsafe fn clear_texture_buffer<T>(texture_buffer: GLuint, size: usize, default_value: T)
where
    T: Clone,
{
    gl::BindBuffer(gl::TEXTURE_BUFFER, texture_buffer);
    let data = vec![default_value; size];
    gl::BufferData(
        gl::TEXTURE_BUFFER,
        (size_of::<T>() * size) as isize,
        data.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::TEXTURE_BUFFER, 0);
}

pub unsafe fn bind_image_texture(
    image_index: u32,
    texture: GLuint,
    access: GLenum, // gl::READ_WRITE, gl::READ_ONLY, gl::WRITE_ONLY
    format: GLenum, // gl::R32UI, gl::RGB10_A2UI, gl::RGB8
) {
    gl::BindImageTexture(image_index, texture, 0, gl::FALSE, 0, access, format);
}

pub unsafe fn bind_3d_image_texture(
    image_index: u32,
    texture: GLuint,
    access: GLenum,
    format: GLenum,
) {
    gl::BindImageTexture(image_index, texture, 0, gl::TRUE, 0, access, format);
}
