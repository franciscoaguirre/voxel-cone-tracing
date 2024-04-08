use std::ffi::c_void;

use c_str_macro::c_str;
use engine::{prelude::*, time::TimeManager};

#[derive(Pausable)]
pub struct Voxelizer {
    voxelization_shader: Shader,
    pub voxels_texture: Texture3Dv2,
    paused: bool,
}

impl Voxelizer {
    pub unsafe fn new() -> Self {
        Self {
            voxelization_shader: compile_shaders!(
                "assets/shaders/voxel_fragment/voxelizeTo3DTexture.glsl"
            ),
            voxels_texture: Texture3Dv2::new(256), // TODO: Not hardcode?
            paused: false,
        }
    }
}

impl Kernel for Voxelizer {
    unsafe fn setup(&mut self, assets: &mut AssetRegistry) {
        assets.register_texture("voxels_texture", self.voxels_texture.id());
    }

    unsafe fn update(&mut self, scene: &Scene, assets: &AssetRegistry, time: &TimeManager) {
        let active_camera = &scene.cameras[scene.active_camera.unwrap_or(0)].borrow();

        // Clear previous voxels texture
        let clear_color = [0f32; 4];
        let mut previous_bound_texture_id = 0;
        gl::GetIntegerv(gl::TEXTURE_BINDING_3D, &mut previous_bound_texture_id);
        gl::BindTexture(gl::TEXTURE_3D, self.voxels_texture.id());
        gl::ClearTexImage(
            self.voxels_texture.id(),
            0,
            gl::RGBA,
            gl::FLOAT,
            clear_color.as_ptr() as *const c_void,
        );
        gl::BindTexture(gl::TEXTURE_3D, previous_bound_texture_id as u32);

        // Create new voxels texture
        self.voxelization_shader.use_program();
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::Viewport(0, 0, 256, 256);
        gl::ColorMask(gl::FALSE, gl::FALSE, gl::FALSE, gl::FALSE);
        gl::Disable(gl::CULL_FACE);
        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::BLEND);
        self.voxelization_shader
            .set_mat4(c_str!("projection"), &active_camera.get_projection_matrix());
        self.voxelization_shader
            .set_mat4(c_str!("view"), &active_camera.transform.get_view_matrix());
        self.voxelization_shader.set_vec3(
            c_str!("pointLight.position"),
            scene.light.transform().position.x,
            scene.light.transform().position.y,
            scene.light.transform().position.z,
        );
        self.voxelization_shader
            .set_vec3(c_str!("pointLight.color"), 1.0, 1.0, 1.0);
        // TODO: Do not hardcode to white.
        gl::BindTexture(gl::TEXTURE_3D, self.voxels_texture.id());
        helpers::bind_3d_image_texture(0, self.voxels_texture.id(), gl::READ_WRITE, gl::RGBA8);
        let model_normalization_matrix = scene.aabb.normalization_matrix();
        for object in scene.objects.iter() {
            object.borrow_mut().draw(
                &self.voxelization_shader,
                &model_normalization_matrix,
                assets,
            );
        }
        gl::GenerateMipmap(gl::TEXTURE_3D);
        gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
    }
}
