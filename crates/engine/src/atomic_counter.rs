use std::mem::size_of;
use std::ffi::c_void;

use gl::types::GLuint;

use crate::helpers;

#[derive(Clone, Copy)]
pub struct AtomicCounter(pub GLuint);

impl AtomicCounter {
    /// Instantiates an atomic counter in the GPU
    pub unsafe fn new() -> Self {
        let mut buffer: u32 = 0;
        let initial_value: [u32; 1] = [0];

        gl::GenBuffers(1, &mut buffer);
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, buffer);
        gl::BufferData(
            gl::ATOMIC_COUNTER_BUFFER,
            size_of::<GLuint>() as isize,
            initial_value.as_ptr() as *const c_void,
            gl::DYNAMIC_READ,
        );
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);

        Self(buffer)
    }

    /// Binds to the currently active shader
    pub unsafe fn bind(&self, index: u32) {
        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, index, self.0);
    }

    /// Gets the value from the atomic counter from the GPU
    pub unsafe fn value(&self) -> GLuint {
        let mut value: GLuint = 0;
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, self.0);
        gl::GetBufferSubData(
            gl::ATOMIC_COUNTER_BUFFER,
            0,
            size_of::<GLuint>() as isize,
            helpers::get_mutable_pointer(&mut value),
        );
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);

        value
    }

    /// Resets the atomic counter
    pub unsafe fn reset(&self) {
        let reset: GLuint = 0;
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, self.0);
        gl::BufferSubData(
            gl::ATOMIC_COUNTER_BUFFER,
            0,
            size_of::<GLuint>() as isize,
            helpers::get_constant_pointer(&reset),
        );
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);
    }

    pub unsafe fn memory_barrier() {
        gl::MemoryBarrier(gl::ATOMIC_COUNTER_BUFFER);
    }

    pub unsafe fn unbind() {
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);
    }
}
