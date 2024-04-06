use c_str_macro::c_str;
use engine::prelude::*;

#[derive(Pausable)]
pub struct DebugConeTracer {
    shader: Shader,
    paused: bool,
}

impl DebugConeTracer {
    pub unsafe fn new() -> Self {
        Self {
            shader: compile_shaders!("assets/shaders/simple_texture/debugConeTracing.glsl"),
            paused: false,
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

    unsafe fn update(&mut self, scene: &Scene, assets: &AssetRegistry) {
        let active_camera = &scene.cameras[scene.active_camera.unwrap_or(0)].borrow();

        self.shader.use_program();

        // OpenGL settings. TODO: Maybe move to the trait?
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        let (width, height) = common::get_framebuffer_size();
        gl::Viewport(0, 0, width, height);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::Enable(gl::DEPTH_TEST);
        gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

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

        gl::DrawArrays(gl::POINTS, 0, 1);
    }
}
