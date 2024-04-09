use c_str_macro::c_str;
use engine::prelude::*;

#[derive(Pausable)]
pub struct ConeTracer {
    cone_tracing_shader: Shader,
    quad: Quad,
    paused: bool,
}

impl ConeTracer {
    pub unsafe fn new() -> Self {
        Self {
            cone_tracing_shader: compile_shaders!(
                "assets/shaders/simple_texture/cone_tracing.glsl"
            ),
            quad: Quad::new(),
            paused: false,
        }
    }
}

impl System for ConeTracer {
    unsafe fn setup(&mut self, _assets: &mut AssetRegistry) {}

    unsafe fn update(&mut self, inputs: SystemInputs) {
        let active_camera = &inputs.scene.cameras[inputs.scene.active_camera.unwrap_or(0)].borrow();

        self.cone_tracing_shader.use_program();

        // OpenGL settings.
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        let (width, height) = common::get_framebuffer_size();
        gl::Viewport(0, 0, width, height);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::Enable(gl::DEPTH_TEST);
        gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        // Upload uniforms.
        self.cone_tracing_shader.set_vec3(
            c_str!("pointLight.position"),
            inputs.scene.light.transform().position.x,
            inputs.scene.light.transform().position.y,
            inputs.scene.light.transform().position.z,
        );
        self.cone_tracing_shader
            .set_vec3(c_str!("pointLight.color"), 1.0, 1.0, 1.0);
        // TODO: Do not hardcode to white.

        let mut texture_counter = 0;

        gl::ActiveTexture(gl::TEXTURE0 + texture_counter);
        gl::BindTexture(
            gl::TEXTURE_3D,
            *inputs.assets.get_texture("voxels_texture").unwrap(),
        ); // TODO: Need to register it.
        self.cone_tracing_shader
            .set_int(c_str!("voxelsTexture"), texture_counter as i32);
        texture_counter += 1;

        // Set geometry buffers.
        let g_buffer_textures = vec![
            (
                c_str!("gBufferColors"),
                *inputs.assets.get_texture("colors").unwrap(),
            ),
            (
                c_str!("gBufferPositions"),
                *inputs.assets.get_texture("positions").unwrap(),
            ),
            (
                c_str!("gBufferNormals"),
                *inputs.assets.get_texture("normals").unwrap(),
            ),
            (
                c_str!("gBufferSpeculars"),
                *inputs.assets.get_texture("specular").unwrap(),
            ),
        ];
        for &(texture_name, texture) in g_buffer_textures.iter() {
            gl::ActiveTexture(gl::TEXTURE0 + texture_counter);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            self.cone_tracing_shader
                .set_int(texture_name, texture_counter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            texture_counter += 1;
        }

        gl::BindVertexArray(self.quad.get_vao());
        gl::DrawElements(
            gl::TRIANGLES,
            self.quad.get_num_indices() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        gl::BindVertexArray(0);
    }
}
