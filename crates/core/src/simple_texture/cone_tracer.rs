use c_str_macro::c_str;
use engine::prelude::*;

use super::GpuKernel;

pub struct ConeTracer {
    cone_tracing_shader: Shader,
}

pub struct ConeTracerRunInputs<'a> {
    pub camera: &'a Camera,
    pub light: &'a Light,
    pub objects: &'a mut [Object],
    pub scene_aabb: &'a Aabb,
    pub voxels_texture: Texture3Dv2,
}

impl GpuKernel for ConeTracer {
    type InitInputs<'a> = ();
    type RunInputs<'a> = ConeTracerRunInputs<'a>;

    unsafe fn init(_: ()) -> Self {
        Self {
            cone_tracing_shader: compile_shaders!(
                "assets/shaders/simple_texture/cone_tracing.glsl"
            ),
        }
    }

    unsafe fn run<'a>(&self, inputs: ConeTracerRunInputs<'a>) {
        self.cone_tracing_shader.use_program();

        // OpenGL settings.
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        let (width, height) = common::get_framebuffer_size();
        gl::Viewport(0, 0, width, height);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        // Upload uniforms.
        self.cone_tracing_shader
            .set_mat4(c_str!("projection"), &inputs.camera.get_projection_matrix());
        self.cone_tracing_shader
            .set_mat4(c_str!("view"), &inputs.camera.transform.get_view_matrix());
        dbg!(&inputs.light.transform());
        self.cone_tracing_shader.set_vec3(
            c_str!("pointLight.position"),
            inputs.light.transform().position.x,
            inputs.light.transform().position.y,
            inputs.light.transform().position.z,
        );
        self.cone_tracing_shader
            .set_vec3(c_str!("pointLight.color"), 1.0, 1.0, 1.0);
        // TODO: Do not hardcode to white.
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_3D, inputs.voxels_texture.id());
        self.cone_tracing_shader.set_int(c_str!("voxelsTexture"), 0);
        let model_normalization_matrix = inputs.scene_aabb.normalization_matrix();
        for object in inputs.objects.iter_mut() {
            object.draw(&self.cone_tracing_shader, &model_normalization_matrix);
        }
    }
}
