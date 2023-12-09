use gl::types::GLenum;

pub enum TextureFormat {
    Rgb10A2Ui,
    Rgba32f,
    R32Ui,
    Rgba8,
}

impl Into<GLenum> for TextureFormat {
    fn into(self) -> GLenum {
        use TextureFormat::*;
        match self {
            Rgb10A2Ui => gl::RGB10_A2UI,
            Rgba32f => gl::RGBA32F,
            R32Ui => gl::R32UI,
            Rgba8 => gl::RGBA8,
        }
    }
}

pub enum TextureAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl Into<GLenum> for TextureAccess {
    fn into(self) -> GLenum {
        use TextureAccess::*;
        match self {
            ReadOnly => gl::READ_ONLY,
            WriteOnly => gl::WRITE_ONLY,
            ReadWrite => gl::READ_WRITE,
        }
    }
}

pub enum UsageHint {
    DynamicRead,
}

impl Into<GLenum> for UsageHint {
    fn into(self) -> GLenum {
        use UsageHint::*;
        match self {
            DynamicRead => gl::DYNAMIC_READ,
        }
    }
}
