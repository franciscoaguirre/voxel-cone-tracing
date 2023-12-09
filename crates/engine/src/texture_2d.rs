use gl::types::GLuint;

pub struct Texture2DV2(pub GLuint);

impl Texture2DV2 {
    pub unsafe fn new_rgba(width: i32, height: i32, nearest: bool) -> Self {
        let mut id = 0;
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            width,
            height,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );
        let sampler = if nearest { gl::NEAREST as i32 } else { gl::LINEAR as i32 };
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, sampler);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, sampler);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        Self(id)
    }
}
