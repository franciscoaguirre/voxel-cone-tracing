use core::marker::PhantomData;

use gl::types::{GLuint, GLenum};

use super::helpers;

pub type Texture = GLuint;
pub type Textures<const N: usize> = [GLuint; N];
pub type Texture2D = GLuint;
pub type Texture3D = GLuint;
pub type TextureBuffer = GLuint;
pub type BufferTexture = (Texture, TextureBuffer);

/// Represents a texture buffer.
/// Usually used to pass in and get out of compute shaders.
// TODO: Rename to just `BufferTexture` when we want to make changes to the whole
// codebase ;)
// This struct is actually really cheap to `clone`, given that the real data sits
// on the GPU.
#[derive(Clone)]
pub struct BufferTextureV2<T> {
    texture: Texture,
    buffer: TextureBuffer,
    length: usize,
    _marker: PhantomData<T>,
}

impl<T: GetGLEnum + Bounded + Clone> BufferTextureV2<T> {
    /// Get a texture and buffer in the GPU just by passing in a vec. Simple!
    pub unsafe fn from_data(data: Vec<T>) -> Self {
        let format = T::get_gl_enum();
        let length = data.len();
        let (texture, buffer) = helpers::generate_texture_buffer_with_initial_data(length, format, data);
        Self {
            texture,
            buffer,
            length,
            _marker: PhantomData,
        }
    }

    /// Constructor with (buffer, texture)
    // TODO: Remove once we switch the whole codebase to use this
    pub fn from_texture_and_buffer((texture, buffer): (GLuint, GLuint)) -> Self {
        Self {
            texture,
            buffer,
            length: 0, // Not to be used when using this constructor
            _marker: PhantomData,
        }
    }

    /// Texture getter
    /// Only gets the index of the texture in the GPU
    pub fn texture(&self) -> Texture {
        self.texture
    }

    /// Buffer getter
    /// Only gets the index of the buffer in the GPU, not the data
    pub fn buffer(&self) -> TextureBuffer {
        self.buffer
    }

    /// Gets the data from the buffer from the GPU
    pub unsafe fn data(&self) -> Vec<T> {
        helpers::get_values_from_texture_buffer(self.buffer, self.length, T::max())
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

/// Trait for transforming a rust type into an OpenGL type
trait GetGLEnum {
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
