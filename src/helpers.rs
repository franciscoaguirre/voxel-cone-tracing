use gl::types::*;
use image::{GenericImageView, ImageFormat};
use std::{
    ffi::{c_void, CStr},
    mem::size_of,
};

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

/// Generates a buffer texture initialized with a default value
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

/// Generates a buffer texture initialized with the data provided
pub unsafe fn generate_texture_buffer_with_data<T>(
    size: usize,
    format: GLenum,
    data: Vec<T>,
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

    gl::BufferData(
        gl::TEXTURE_BUFFER,
        (size_of::<T>() * size) as isize,
        data.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    gl::BindBuffer(gl::TEXTURE_BUFFER, 0);

    (texture, texture_buffer)
}

pub unsafe fn fill_texture_buffer_with_data<T>(texture_buffer: GLuint, data: &Vec<T>) {
    gl::BindBuffer(gl::TEXTURE_BUFFER, texture_buffer);
    gl::BufferData(
        gl::TEXTURE_BUFFER,
        (size_of::<T>() * data.len()) as isize,
        data.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::TEXTURE_BUFFER, 0);
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

pub extern "system" fn gl_debug_output_callback(
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _user_param: *mut c_void,
) {
    // Ignore performance errors
    if type_ == 33360 {
        return;
    }

    if id == 131_169 || id == 131_185 || id == 131_218 || id == 131_204 {
        // ignore these non-significant error codes
        return;
    }

    println!("---------------");
    let message = unsafe { CStr::from_ptr(message).to_str().unwrap() };

    println!("Debug message ({}): {}", id, message);
    match source {
        gl::DEBUG_SOURCE_API => println!("Source: API"),
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => println!("Source: Window System"),
        gl::DEBUG_SOURCE_SHADER_COMPILER => println!("Source: Shader Compiler"),
        gl::DEBUG_SOURCE_THIRD_PARTY => println!("Source: Third Party"),
        gl::DEBUG_SOURCE_APPLICATION => println!("Source: Application"),
        gl::DEBUG_SOURCE_OTHER => println!("Source: Other"),
        _ => println!("Source: Unknown enum value"),
    }

    match type_ {
        gl::DEBUG_TYPE_ERROR => println!("Type: Error"),
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => println!("Type: Deprecated Behaviour"),
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => println!("Type: Undefined Behaviour"),
        gl::DEBUG_TYPE_PORTABILITY => println!("Type: Portability"),
        gl::DEBUG_TYPE_PERFORMANCE => println!("Type: Performance"),
        gl::DEBUG_TYPE_MARKER => println!("Type: Marker"),
        gl::DEBUG_TYPE_PUSH_GROUP => println!("Type: Push Group"),
        gl::DEBUG_TYPE_POP_GROUP => println!("Type: Pop Group"),
        gl::DEBUG_TYPE_OTHER => println!("Type: Other"),
        _ => println!("Type: Unknown enum value"),
    }

    match severity {
        gl::DEBUG_SEVERITY_HIGH => println!("Severity: high"),
        gl::DEBUG_SEVERITY_MEDIUM => println!("Severity: medium"),
        gl::DEBUG_SEVERITY_LOW => println!("Severity: low"),
        gl::DEBUG_SEVERITY_NOTIFICATION => println!("Severity: notification"),
        _ => println!("Severity: Unknown enum value"),
    }
}

/// Helper function to load a texture.
/// Used mostly for testing and quick prototyping.
/// Takes in a PNG.
#[allow(dead_code)]
pub fn load_texture(image_path: &str) -> GLuint {
    let img_data = std::fs::read(image_path).expect("Failed to read image");
    let img = image::load_from_memory_with_format(&img_data, ImageFormat::Png)
        .expect("Failed to open image")
        .flipv();
    let (width, height) = img.dimensions();
    let img_data = img.to_rgba8().into_raw();

    unsafe {
        let mut texture_id = 0;
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            width as GLint,
            height as GLint,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img_data.as_ptr() as *const c_void,
        );
        texture_id
    }
}
