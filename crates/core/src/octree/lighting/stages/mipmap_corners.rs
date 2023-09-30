use c_str_macro::c_str;
use cgmath::vec3;
use gl::types::GLuint;
use engine::prelude::*;

use crate::{
    config::Config,
    octree::OctreeTextures,
};

pub struct MipmapCornersPass {
    shader: Shader,
    light_view_map: GLuint,
}

impl MipmapCornersPass {
    pub fn init(light_view_map: GLuint) -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/mipmapCornerPhotons.comp.glsl"),
            light_view_map,
        }
    }

    pub unsafe fn run(&self, textures: &OctreeTextures, level: u32) {
        self.shader.use_program();

        let config = Config::instance();

        self.shader.set_uint(c_str!("octreeLevel"), level);
        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, self.light_view_map);
        self.shader.set_int(c_str!("lightViewMap"), 0);

        helpers::bind_image_texture(0, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_3d_image_texture(1, textures.brick_pool_photons, gl::READ_WRITE, gl::R32UI);

        let (viewport_width, viewport_height) = config.viewport_dimensions();

        self.shader.dispatch_xyz(vec3(
            (viewport_width as f32 / 32 as f32).ceil() as u32,
            (viewport_height as f32 / 32 as f32).ceil() as u32,
            1,
        ));
        self.shader.wait();
    }
}
