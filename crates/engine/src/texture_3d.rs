use std::ffi::c_void;

use gl::types::*;

pub struct Texture3DV2(pub GLuint);

impl Texture3DV2 {
    pub unsafe fn new_rgba(size_one_dimension: u32) -> Self {
        let id = Self::generate_texture(
            size_one_dimension,
            gl::RGBA8,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            0u32,
        );
        Self(id)
    }

    pub unsafe fn new_r32ui(size_one_dimension: u32) -> Self {
        let id = Self::generate_texture(
            size_one_dimension,
            gl::R32UI,
            gl::RED_INTEGER,
            gl::UNSIGNED_INT,
            0u32,
        );
        Self(id)
    }

    pub unsafe fn new_rgb10_a2ui(size_one_dimension: u32) -> Self {
        let id = Self::generate_texture(
            size_one_dimension,
            gl::RGB10_A2,
            gl::RGBA_INTEGER,
            gl::UNSIGNED_INT_2_10_10_10_REV,
            0u32,
        );
        Self(id)
    }

    pub unsafe fn new_rgba32f(size_one_dimension: u32) -> Self {
        let id = Self::generate_texture(size_one_dimension, gl::RGBA32F, gl::RGBA, gl::FLOAT, 0u128);
        Self(id)
    }

    /// Copies the contents from this texture to `other`.
    /// This does not alter the contents of `self`.
    /// Only copies with offset 0 and limit `size`.
    pub unsafe fn copy(&self, other: &mut Texture3DV2, size: usize) {
        gl::CopyImageSubData(
            self.0,
            gl::TEXTURE_3D,
            0,
            0,
            0,
            0,
            other.0,
            gl::TEXTURE_3D,
            0,
            0,
            0,
            0,
            size as i32,
            size as i32,
            size as i32,
        );
    }

    /// Generates a 3D texture
    /// Takes `T` as a default_value to account for size differences in the components
    unsafe fn generate_texture<T: Clone>(
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
}
