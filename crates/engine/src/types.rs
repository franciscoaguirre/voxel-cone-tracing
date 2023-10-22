use gl::types::GLuint;

pub type Texture = GLuint;
pub type Textures<const N: usize> = [Texture2D; N];
pub type Texture2D = GLuint;
pub type Texture3D = GLuint;
pub type TextureBuffer = GLuint;

/// Represents a texture buffer
// Deprecated. We should switch to `BufferTextureV2`
pub type BufferTexture = (Texture, TextureBuffer);
