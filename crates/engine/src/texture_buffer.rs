use core::marker::PhantomData;

use gl::types::{GLuint, GLenum};

use super::{helpers, types::*, enums::*, traits::{GetGLEnum, ArbitraryValue}};

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

impl<T: GetGLEnum + ArbitraryValue + Clone> BufferTextureV2<T> {
    /// Get a texture and buffer in the GPU just by passing in a slice. Simple!
    /// The slice type `T` needs to implement `GetGLEnum` to know its format on the GPU.
    pub unsafe fn from_data(data: &[T]) -> Self {
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

    pub unsafe fn from_data_and_hint(data: &[T], hint: UsageHint) -> Self {
        let format = T::get_gl_enum();
        let length = data.len();
        let (texture, buffer) = helpers::generate_texture_buffer_full(
            length,
            format,
            data,
            hint.into(),
        );
        Self {
            texture,
            buffer,
            length,
            _marker: PhantomData,
        }
    }

    pub unsafe fn fill_with(&mut self, data: &[T], is_dynamic: bool) {
        let usage_hint = if is_dynamic { gl::DYNAMIC_DRAW } else { gl::STATIC_DRAW };
        helpers::fill_texture_buffer_with_data(
            self.buffer(),
            data,
            usage_hint,
        );
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
        helpers::get_values_from_texture_buffer(self.buffer, self.length, T::arbitrary_value())
    }

    /// Returns the length of the collection, set at construction
    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T> BufferTextureV2<T> {
    /// Returns the maximum size, in bytes, for a texture buffer in the current GPU
    pub unsafe fn max_texture_buffer_size() -> i32 {
        let mut result = 0;
        gl::GetIntegerv(gl::MAX_TEXTURE_BUFFER_SIZE, &mut result);
        result
    }
}
