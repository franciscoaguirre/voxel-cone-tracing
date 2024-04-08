use crate::{
    prelude::{compile_shaders, AssetRegistry, Kernel, Pausable, Scene, Shader},
    time::TimeManager,
};
use c_str_macro::c_str;

#[derive(Pausable)]
pub struct RenderObjects {
    shader: Shader,
    paused: bool,
}

impl RenderObjects {
    pub fn new() -> Self {
        Self {
            shader: compile_shaders!(
                "assets/shaders/model/modelLoading.vert.glsl",
                "assets/shaders/model/modelLoading.frag.glsl",
                "assets/shaders/model/modelLoading.geom.glsl",
            ),
            paused: false,
        }
    }
}

impl Kernel for RenderObjects {
    unsafe fn setup(&mut self, _assets: &mut AssetRegistry) {}
    unsafe fn update(&mut self, scene: &Scene, assets: &AssetRegistry, time: &TimeManager) {
        self.shader.use_program();
        let camera = &scene.cameras[scene.active_camera.unwrap_or(0)].borrow();
        self.shader
            .set_mat4(c_str!("projection"), &camera.get_projection_matrix());
        self.shader
            .set_mat4(c_str!("view"), &camera.transform.get_view_matrix());
        for object in scene.objects.iter() {
            object
                .borrow_mut()
                .draw(&self.shader, &scene.aabb.normalization_matrix(), assets);
        }
    }
}
