use gl::types::GLenum;

/// Trait for transforming a rust type into an OpenGL type
pub trait GetGLEnum {
    /// Get the OpenGL enum that represents this particular type
    fn get_gl_enum() -> GLenum;
}

impl GetGLEnum for u32 {
    fn get_gl_enum() -> GLenum {
        gl::R32UI
    }
}

/// Trait for bounded types, i.e. types that have a minimum and maximum value
pub trait Bounded {
    /// The maximum possible value for this type
    fn max() -> Self;

    /// The minimum possible value for this type
    fn min() -> Self;
}

impl Bounded for u32 {
    fn max() -> Self {
        u32::MAX
    }

    fn min() -> Self {
        u32::MIN
    }
}
