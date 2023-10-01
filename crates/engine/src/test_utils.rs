use egui_glfw_gl::glfw::{self, Context};

pub fn init_opengl_context() -> (glfw::Glfw, glfw::Window) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    // No need for window to be visible for tests
    glfw.window_hint(glfw::WindowHint::Visible(false));

    // OpenGL version hints
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Create a windowed mode window and its OpenGL context
    let (mut window, _) = glfw
        .create_window(640, 480, "Hidden Window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    (glfw, window)
}

#[test]
fn compute_shader_works() {
    use crate::prelude::*;

    let (_glfw, _window) = init_opengl_context();

    // Test shader code that just reads and writes to a buffer
    let shader_code = r#"
        #version 460 core

        layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

        uniform layout(binding = 0, r32ui) uimageBuffer testBuffer;

        void main() {
            uint number = imageLoad(testBuffer, 0).r;
            uint result = (number + 1) * 2;
            imageStore(testBuffer, 0, uvec4(result, 0, 0, 0));
        }
    "#;

    let shader = Shader::new_compute_from_string(&shader_code);

    unsafe {
        shader.use_program();
        // Create a buffer for the shader
        let (texture, texture_buffer) = helpers::generate_texture_buffer(1, gl::R32UI, 0_u32);
        helpers::bind_image_texture(0, texture, gl::READ_WRITE, gl::R32UI);

        // First time
        shader.dispatch(1);
        shader.wait();
        let values = helpers::get_values_from_texture_buffer(texture_buffer, 1, 0_u32);
        assert_eq!(values[0], 2);

        // Second time
        shader.dispatch(1);
        shader.wait();
        let values = helpers::get_values_from_texture_buffer(texture_buffer, 1, 0_u32);
        assert_eq!(values[0], 6);

        // Clearing results in the same as the first time
        helpers::clear_texture_buffer(texture_buffer, 1, 0_u32, gl::DYNAMIC_DRAW);
        shader.dispatch(1);
        shader.wait();
        let values = helpers::get_values_from_texture_buffer(texture_buffer, 1, 0_u32);
        assert_eq!(values[0], 2);
    }
}
