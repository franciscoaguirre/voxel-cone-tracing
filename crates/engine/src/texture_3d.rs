use std::ffi::c_void;

use gl::types::*;

#[derive(Debug, Clone, Copy)]
pub struct Texture3Dv2(GLuint);

impl Texture3Dv2 {
    pub unsafe fn new(size_one_dimension: i32) -> Self {
        let mut texture_id = 0;
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_3D, texture_id);

        let wrap = gl::CLAMP_TO_BORDER as i32;
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, wrap);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, wrap);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, wrap);

        gl::TexParameteri(
            gl::TEXTURE_3D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        let levels = 8; // TODO: This assumes 256 voxels per dimension. Do not hardcode this.
        gl::TexStorage3D(
            gl::TEXTURE_3D,
            levels,
            gl::RGBA8,
            size_one_dimension,
            size_one_dimension,
            size_one_dimension,
        );
        let initial_data = vec![0u32; size_one_dimension.pow(3) as usize];
        gl::TexSubImage3D(
            gl::TEXTURE_3D,
            0,
            0,
            0,
            0,
            size_one_dimension,
            size_one_dimension,
            size_one_dimension,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            initial_data.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_3D);
        gl::BindTexture(gl::TEXTURE_3D, 0);

        Self(texture_id)
    }

    pub fn id(&self) -> GLuint {
        self.0
    }
}
