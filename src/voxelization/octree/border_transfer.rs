use c_str_macro::c_str;
use gl::types::*;

use crate::rendering::shader::Shader;

pub struct BorderTransferPass {
    shader: Shader,
}

impl BorderTransferPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/border_transfer.comp.glsl"),
        }
    }

    pub unsafe fn run(&self) {
        self.shader.use_program();

        self.initialize_shader();

        self.shader.dispatch(65_535); // TODO: Calculate number of groups
        self.shader.wait();
    }

    unsafe fn initialize_shader(&self) {}
}
