use crate::prelude::{
    common, compile_shaders, AssetRegistry, GeometryFramebuffer, Kernel, Pausable, Scene, Shader,
};
use c_str_macro::c_str;

#[derive(Pausable)]
pub struct GeometryBuffers {
    shader: Shader,
    framebuffer: GeometryFramebuffer,
    paused: bool,
}

impl GeometryBuffers {
    pub unsafe fn new() -> Self {
        Self {
            shader: compile_shaders!("assets/shaders/octree/viewMap.glsl"),
            framebuffer: GeometryFramebuffer::new(),
            paused: false,
        }
    }
}

impl Kernel for GeometryBuffers {
    unsafe fn setup(&mut self, assets: &mut AssetRegistry) {
        for (name, texture) in self.framebuffer.textures().iter() {
            assets.register_texture(name, *texture);
        }
    }
    unsafe fn update(&mut self, scene: &Scene, assets: &AssetRegistry) {
        let active_camera = &scene.cameras[scene.active_camera.unwrap_or(0)].borrow();
        self.shader.use_program();
        self.shader
            .set_mat4(c_str!("projection"), &active_camera.get_projection_matrix());
        self.shader
            .set_mat4(c_str!("view"), &active_camera.transform.get_view_matrix());
        self.shader.set_uint(c_str!("voxelDimension"), 256); // TODO: Get `voxel_dimension` somewhere available.
        let (width, height) = common::get_framebuffer_size();
        gl::Viewport(0, 0, width, height);
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.fbo());
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        for object in scene.objects.iter() {
            object
                .borrow()
                .draw(&self.shader, &scene.aabb.normalization_matrix(), assets);
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
}
