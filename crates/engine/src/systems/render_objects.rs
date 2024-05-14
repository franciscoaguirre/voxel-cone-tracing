use crate::{
    prelude::{compile_shaders, AssetRegistry, Pausable, Shader, System},
    system::{SystemInfo, SystemInputs},
};
use c_str_macro::c_str;

#[derive(Pausable)]
pub struct RenderObjects {
    shader: Shader,
    paused: bool,
    pause_next_frame: bool,
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
            pause_next_frame: false,
        }
    }
}

impl System for RenderObjects {
    unsafe fn setup(&mut self, _assets: &mut AssetRegistry) {}
    unsafe fn update(&mut self, inputs: SystemInputs) {
        self.shader.use_program();
        let camera = &inputs.scene.cameras[inputs.scene.active_camera.unwrap_or(0)].borrow();
        self.shader
            .set_mat4(c_str!("projection"), &camera.get_projection_matrix());
        self.shader
            .set_mat4(c_str!("view"), &camera.transform.get_view_matrix());
        for object in inputs.scene.objects.iter() {
            object.borrow_mut().draw(
                &self.shader,
                &inputs.scene.aabb.normalization_matrix(),
                inputs.assets,
            );
        }
    }
    fn get_info(&self) -> SystemInfo {
        SystemInfo {
            name: "RenderObjects",
        }
    }
}