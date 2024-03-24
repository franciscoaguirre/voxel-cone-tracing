use std::ffi::c_void;

use c_str_macro::c_str;
use engine::prelude::*;

use super::GpuKernel;

pub struct Voxelizer {
    voxelization_shader: Shader,
    pub voxels_texture: Texture3Dv2,
}

pub struct VoxelizerRunInputs<'a> {
    pub camera: &'a Camera,
    pub light: &'a Light,
    pub scene_aabb: &'a Aabb,
    pub objects: &'a mut [Object],
}

impl GpuKernel for Voxelizer {
    type InitInputs<'a> = ();
    type RunInputs<'a> = VoxelizerRunInputs<'a>;

    unsafe fn init(_: ()) -> Self {
        Self {
            voxelization_shader: compile_shaders!(
                "assets/shaders/voxel_fragment/voxelizeTo3DTexture.glsl"
            ),
            voxels_texture: Texture3Dv2::new(256), // TODO: Not hardcode?
        }
    }

    unsafe fn run<'a>(&self, inputs: VoxelizerRunInputs<'a>) {
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
            .set_mat4(c_str!("projection"), &inputs.camera.get_projection_matrix());
        self.voxelization_shader
            .set_mat4(c_str!("view"), &inputs.camera.transform.get_view_matrix());
        self.voxelization_shader.set_vec3(
            c_str!("pointLight.position"),
            inputs.light.transform().position.x,
            inputs.light.transform().position.y,
            inputs.light.transform().position.z,
        );
        self.voxelization_shader
            .set_vec3(c_str!("pointLight.color"), 1.0, 1.0, 1.0);
        // TODO: Do not hardcode to white.
        gl::BindTexture(gl::TEXTURE_3D, self.voxels_texture.id());
        helpers::bind_3d_image_texture(0, self.voxels_texture.id(), gl::READ_WRITE, gl::RGBA8);
        let model_normalization_matrix = inputs.scene_aabb.normalization_matrix();
        for object in inputs.objects.iter_mut() {
            object.draw(&self.voxelization_shader, &model_normalization_matrix);
        }
        gl::GenerateMipmap(gl::TEXTURE_3D);
        gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
    }
}
