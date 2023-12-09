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

impl GetGLEnum for f32 {
    fn get_gl_enum() -> GLenum {
        gl::R32F
    }
}

/// Trait for types where we'd like to have access to an arbitrary value
/// of that type. For using as a default in buffers.
pub trait ArbitraryValue {
    /// Used to get an arbitrary value of the type.
    fn arbitrary_value() -> Self;
}

impl ArbitraryValue for u32 {
    fn arbitrary_value() -> u32 {
        42
    }
}

impl ArbitraryValue for f32 {
    fn arbitrary_value() -> f32 {
        42.0
    }
}
