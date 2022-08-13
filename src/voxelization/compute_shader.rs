use std::{
    ffi::{c_void, CString},
    fs::File,
    io::Read,
    ptr, str,
};

use gl::types::*;

use super::helpers;

static mut INPUT_TEXTURE: GLuint = 0;
static mut INPUT_TEXTURE_BUFFER: GLuint = 0;

#[derive(Default)]
pub struct ComputeShader {
    pub id: u32,
    texture: u32,
    size: u32,
}

impl ComputeShader {
    pub fn new(path: &str, size: u32) -> Self {
        let mut shader = Self::default();
        let mut shader_file =
            File::open(path).unwrap_or_else(|_| panic!("Failed to open {}", path));
        let mut shader_code = String::new();
        shader_file
            .read_to_string(&mut shader_code)
            .expect("Failed to read shader");
        let shader_code = CString::new(shader_code.as_bytes()).unwrap();

        unsafe {
            let shader_id = gl::CreateShader(gl::COMPUTE_SHADER);
            gl::ShaderSource(shader_id, 1, &shader_code.as_ptr(), ptr::null());
            gl::CompileShader(shader_id);
            shader.check_compile_errors(shader_id, "COMPUTE");

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, shader_id);
            gl::LinkProgram(program_id);
            shader.check_compile_errors(shader_id, "PROGRAM");

            gl::DeleteShader(shader_id);

            shader.id = program_id;
            shader.size = size;

            helpers::generate_linear_buffer(
                size as usize,
                gl::RGBA8,
                &mut INPUT_TEXTURE,
                &mut INPUT_TEXTURE_BUFFER,
            );
        }

        shader
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);

        gl::BindImageTexture(0, INPUT_TEXTURE, 0, gl::FALSE, 0, gl::WRITE_ONLY, gl::R32F);
    }

    pub unsafe fn dispatch(&self) {
        gl::DispatchCompute(1, 1, 1);
    }

    pub unsafe fn wait(&self) {
        gl::MemoryBarrier(gl::ALL_BARRIER_BITS);
    }

    pub unsafe fn set_values(&self, _values: *const c_void) {
        gl::BindBuffer(gl::TEXTURE_BUFFER, INPUT_TEXTURE_BUFFER);
        gl::BufferData(
            gl::TEXTURE_BUFFER,
            self.size as isize,
            ptr::null(),
            gl::STATIC_DRAW,
        );
    }

    pub unsafe fn get_values(&self) -> Vec<u32> {
        let values = vec![0u32; self.size as usize];
        gl::GetBufferSubData(
            gl::TEXTURE_BUFFER,
            0,
            self.size as isize,
            values.as_ptr() as *mut c_void,
        );
        values
    }

    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = vec![0u8; 1024];
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                    type_,
                    str::from_utf8(&info_log).unwrap()
                );
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                    type_,
                    str::from_utf8(&info_log).unwrap()
                );
            }
        }
    }
}
