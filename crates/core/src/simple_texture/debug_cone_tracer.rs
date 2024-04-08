use c_str_macro::c_str;
use engine::{prelude::*, time::TimeManager};
use gl::types::GLuint;

#[derive(Pausable)]
pub struct DebugConeTracer {
    shader: Shader,
    vao: GLuint,
    paused: bool,
}

impl DebugConeTracer {
    pub unsafe fn new() -> Self {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);

        Self {
            shader: compile_shaders!("assets/shaders/simple_texture/debugConeTracing.glsl"),
            paused: false,
            vao,
        }
    }
}

impl Kernel for DebugConeTracer {
    unsafe fn setup(&mut self, assets: &mut AssetRegistry) {
        assets.register_uniform(
            "SimpleDebugConeTracer.gBufferQueryCoordinates",
            Uniform::Vec2(0., 0.),
        );
    }

    unsafe fn update(&mut self, scene: &Scene, assets: &AssetRegistry, _time: &TimeManager) {
        let active_camera = &scene.cameras[scene.active_camera.unwrap_or(0)].borrow();

        self.shader.use_program();
        gl::BindVertexArray(self.vao);

        // OpenGL settings. TODO: Maybe add pre and post update stages to the trait.
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        let (width, height) = common::get_framebuffer_size();
        gl::Viewport(0, 0, width, height);

        // Upload uniforms.
        let g_buffer_query_coordinates = {
            let Uniform::Vec2(x, y) = assets
                .get_uniform("SimpleDebugConeTracer.gBufferQueryCoordinates")
                .unwrap()
            else {
                unreachable!()
            };
            (*x, *y)
        };
        self.shader.set_vec2(
            c_str!("gBufferQueryCoordinates"),
            g_buffer_query_coordinates.0,
            g_buffer_query_coordinates.1,
        );
        self.shader
            .set_mat4(c_str!("projection"), &active_camera.get_projection_matrix());
        self.shader
            .set_mat4(c_str!("view"), &active_camera.transform.get_view_matrix());
        self.shader.set_vec3(
            c_str!("pointLight.position"),
            scene.light.transform().position.x,
            scene.light.transform().position.y,
            scene.light.transform().position.z,
        );
        self.shader
            .set_vec3(c_str!("pointLight.color"), 1.0, 1.0, 1.0);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, *assets.get_texture("positions").unwrap());
        self.shader.set_int(c_str!("gBufferPositions"), 0);

        gl::DrawArrays(gl::POINTS, 0, 1);

        gl::BindTexture(gl::TEXTURE_2D, 0);
        gl::BindVertexArray(0);
    }
}
