use c_str_macro::c_str;
use cgmath::vec3;
use gl::types::GLuint;

use crate::rendering::shader::compile_compute;
use crate::{config::CONFIG, helpers, octree::OctreeTextures, rendering::shader::Shader};

pub struct MipmapCentersPass {
    shader: Shader,
    light_view_map: GLuint,
}

impl MipmapCentersPass {
    pub fn init(light_view_map: GLuint) -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/mipmapCenterPhotons.comp.glsl"),
            light_view_map,
        }
    }

    pub unsafe fn run(&self, textures: &OctreeTextures, level: u32) {
        self.shader.use_program();

        self.shader.set_uint(c_str!("octreeLevel"), level);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, self.light_view_map);
        self.shader.set_int(c_str!("lightViewMap"), 0);

        helpers::bind_image_texture(0, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_3d_image_texture(1, textures.brick_pool_photons, gl::READ_WRITE, gl::R32UI);

        self.shader.dispatch_xyz(vec3(
            (CONFIG.viewport_width as f32 / 32 as f32).ceil() as u32,
            (CONFIG.viewport_height as f32 / 32 as f32).ceil() as u32,
            1,
        ));
        self.shader.wait();
    }
}
