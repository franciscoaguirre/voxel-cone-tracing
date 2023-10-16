use cgmath::Matrix4;
use gl::types::*;
use image::{GenericImageView, ImageFormat};
use std::{
    env,
    ffi::{c_void, CStr},
    mem::size_of,
    path::Path,
};

use crate::{model::Model, aabb::Aabb, types::*};

pub unsafe fn generate_atomic_counter_buffer() -> GLuint {
    generate_atomic_counter_buffer1()
}

pub unsafe fn generate_atomic_counter_buffer1() -> GLuint {
    let mut buffer: u32 = 0;
    let initial_value: [u32; 1] = [0];

    gl::GenBuffers(1, &mut buffer);
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, buffer);
    gl::BufferData(
        gl::ATOMIC_COUNTER_BUFFER,
        size_of::<GLuint>() as isize,
        initial_value.as_ptr() as *const c_void,
        gl::DYNAMIC_READ,
    );
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);

    buffer
}

pub unsafe fn generate_texture_buffer<T>(
    size: usize,
    format: GLenum,
    default_value: T,
) -> BufferTexture
where
    T: Clone,
{
    generate_texture_buffer_with_hint(size, format, default_value, gl::STATIC_DRAW)
}

/// Generates a buffer texture initialized with a default value
pub unsafe fn generate_texture_buffer_with_hint<T>(
    size: usize,
    format: GLenum,
    default_value: T,
    usage_hint: GLuint,
) -> BufferTexture
where
    T: Clone,
{
    generate_texture_buffer_full(size, format, vec![default_value; size], usage_hint)
}

pub unsafe fn generate_texture_buffer_with_initial_data<T>(
    size: usize,
    format: GLenum,
    initial_data: Vec<T>,
) -> BufferTexture {
    generate_texture_buffer_full(size, format, initial_data, gl::STATIC_DRAW)
}

pub unsafe fn generate_texture_buffer_full<T>(
    size: usize,
    format: GLenum,
    initial_data: Vec<T>,
    usage_hint: GLuint,
) -> BufferTexture {
    let mut texture_buffer: GLuint = 0;
    gl::GenBuffers(1, &mut texture_buffer);

    gl::BindBuffer(gl::TEXTURE_BUFFER, texture_buffer);

    let mut texture: GLuint = 0;

    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_BUFFER, texture);
    gl::TexBuffer(gl::TEXTURE_BUFFER, format, texture_buffer);
    gl::BindBuffer(gl::TEXTURE_BUFFER, 0);

    fill_texture_buffer_with_data(texture_buffer, &initial_data, usage_hint);

    (texture, texture_buffer)
}

pub unsafe fn fill_texture_buffer_with_data<T>(
    texture_buffer: GLuint,
    data: &Vec<T>,
    usage_hint: GLenum,
) {
    gl::BindBuffer(gl::TEXTURE_BUFFER, texture_buffer);
    gl::BufferData(
        gl::TEXTURE_BUFFER,
        (size_of::<T>() * data.len()) as isize,
        data.as_ptr() as *const c_void,
        usage_hint,
    );
    gl::BindBuffer(gl::TEXTURE_BUFFER, 0);
}

pub unsafe fn generate_3d_rgba_texture(size_one_dimension: u32) -> GLuint {
    generate_3d_texture(
        size_one_dimension,
        gl::RGBA8,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        0u32,
    )
}

pub unsafe fn generate_3d_rgba32f_texture(size_one_dimension: u32) -> GLuint {
    generate_3d_texture(size_one_dimension, gl::RGBA32F, gl::RGBA, gl::FLOAT, 0u128)
}

pub unsafe fn generate_3d_rgb10_a2ui_texture(size_one_dimension: u32) -> GLuint {
    generate_3d_texture(
        size_one_dimension,
        gl::RGB10_A2,
        gl::RGBA_INTEGER,
        gl::UNSIGNED_INT_2_10_10_10_REV,
        0u32,
    )
}

pub unsafe fn generate_3d_r32ui_texture(size_one_dimension: u32) -> GLuint {
    generate_3d_texture(
        size_one_dimension,
        gl::R32UI,
        gl::RED_INTEGER,
        gl::UNSIGNED_INT,
        0u32,
    )
}

/// Generates a 3D texture
/// Takes `T` as a default_value to account for size differences in the components
unsafe fn generate_3d_texture<T: Clone>(
    size_one_dimension: u32,
    internal_format: GLenum,
    format: GLenum,
    _type: GLenum,
    default_value: T,
) -> GLuint {
    let mut texture: GLuint = 0;

    // TODO: Apparently powers of two are recommended, but using the next power of
    // two understandably makes this really large really fast.
    // let size_one_dimension = size_one_dimension.next_power_of_two() as i32;

    let size = size_one_dimension.pow(3);

    let initial_data = vec![default_value; size as usize];

    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_3D, texture);
    gl::TexImage3D(
        gl::TEXTURE_3D,
        0,
        internal_format as i32,
        size_one_dimension as i32,
        size_one_dimension as i32,
        size_one_dimension as i32,
        0,
        format,
        _type,
        initial_data.as_ptr() as *const c_void,
    );
    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    gl::BindTexture(gl::TEXTURE_3D, 0);

    texture
}

pub fn get_constant_pointer(number: &u32) -> *const c_void {
    number as *const u32 as *const c_void
}

pub fn get_mutable_pointer(number: &mut u32) -> *mut c_void {
    number as *mut u32 as *mut c_void
}

/// Gets the value from the atomic counter passed in AND resets it
pub unsafe fn get_value_from_atomic_counter(counter: u32) -> GLuint {
    let mut value: GLuint = 0;
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, counter);
    gl::GetBufferSubData(
        gl::ATOMIC_COUNTER_BUFFER,
        0,
        size_of::<GLuint>() as isize,
        get_mutable_pointer(&mut value),
    );
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);
    reset_atomic_counter(counter);

    value
}

pub unsafe fn reset_atomic_counter(counter: u32) {
    let reset: GLuint = 0;
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, counter);
    gl::BufferSubData(
        gl::ATOMIC_COUNTER_BUFFER,
        0,
        size_of::<GLuint>() as isize,
        get_constant_pointer(&reset),
    );
    gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);
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

pub unsafe fn clear_texture_buffer<T>(
    texture_buffer: GLuint,
    size: usize,
    default_value: T,
    usage_hint: GLuint,
) where
    T: Clone,
{
    gl::BindBuffer(gl::TEXTURE_BUFFER, texture_buffer);
    let data = vec![default_value; size];
    gl::BufferData(
        gl::TEXTURE_BUFFER,
        (size_of::<T>() * size) as isize,
        data.as_ptr() as *const c_void,
        usage_hint,
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

pub fn r32ui_to_rgb10_a2ui(from: u32) -> (u32, u32, u32) {
    let mask = 0b00000000000000000000001111111111;

    (from & mask, (from >> 10) & mask, (from >> 20) & mask)
}

pub fn rgb10_a2ui_to_r32ui(x: u32, y: u32, z: u32) -> u32 {
    let mask = 0b00000000000000000000001111111111;

    (x & mask) | ((y & mask) << 10) | ((z & mask) << 20)
}

// TODO: Bring back, elsewhere
// #[allow(dead_code)]
// pub fn get_brick_coordinates(node_id: u32) -> Vector3<u32> {
//     let brick_pool_resolution_bricks = CONFIG.brick_pool_resolution / 3;
//     let mut coordinates = vec3(0u32, 0u32, 0u32);
//     coordinates.x = node_id % brick_pool_resolution_bricks;
//     coordinates.y = (node_id / brick_pool_resolution_bricks) % brick_pool_resolution_bricks;
//     coordinates.z = node_id / (brick_pool_resolution_bricks * brick_pool_resolution_bricks);
//     coordinates *= 3;
//     return coordinates;
// }

/// Load a model with a given `name`.
/// Already goes to "assets/models/" to find it and its textures.
/// Only loads `.obj` files
pub unsafe fn load_model(name: &str) -> Model {
    let previous_current_dir = env::current_dir().unwrap();
    env::set_current_dir(Path::new("assets/models")).unwrap();
    let model_file = format!("{name}.obj");
    let model = Model::new(&model_file);
    env::set_current_dir(previous_current_dir).unwrap();
    model
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r32ui_to_rgb10_a2ui_works() {
        let test_data = vec![
            (134461564_u32, (124, 238, 128)),
            (3355484387_u32, (227, 40, 128)),
        ];

        for &(input, expected_result) in test_data.iter() {
            let result = r32ui_to_rgb10_a2ui(input);
            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn rgb10_a2ui_to_r32ui_works() {
        let test_data = vec![
            ((124, 238, 128), 134461564_u32),
            ((227, 40, 128), 3355484387_u32),
        ];

        for &(input, expected_result) in test_data.iter() {
            let result = rgb10_a2ui_to_r32ui(input.0, input.1, input.2);
            assert_eq!(result, expected_result);
        }
    }
}
