use gl::types::GLuint;

pub type Texture = GLuint;
pub type Textures<const N: usize> = [Texture2D; N];
pub type Texture2D = GLuint;
pub type Texture3D = GLuint;
pub type TextureBuffer = GLuint;

/// Represents a texture buffer
// Deprecated. We should switch to `BufferTextureV2`
pub type BufferTexture = (Texture, TextureBuffer);

// TODO: `Shader` could just have a generic `set_uniform` method
// that takes in a `Uniform` and matches on it to know what method to call.
// It would also be cool if we can add some metadata to the uniform itself,
// like the name, and then we wouldn't even need to name it when setting it,
// only when defining it.
// We could also add additional metadata, if it's a uint, what's it min and
// max values.
// This could be used by the UI to make a slider, for example.
pub enum Uniform {
    Vec2(f32, f32),
    Vec3(f32, f32, f32),
    Bool(bool),
    Uint(u32),
    // TODO: Add all others.
}
