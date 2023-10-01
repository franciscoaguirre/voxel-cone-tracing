use gl::types::GLuint;

pub type Texture = GLuint;
pub type Texture2D = GLuint;
pub type Texture3D = GLuint;
pub type TextureBuffer = GLuint;
pub type BufferTexture = (Texture, TextureBuffer);
