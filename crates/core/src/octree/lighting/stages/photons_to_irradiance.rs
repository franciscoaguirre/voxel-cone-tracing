use c_str_macro::c_str;
use cgmath::vec3;
use gl::types::GLuint;
use renderer::prelude::*;

use crate::{
    config::Config,
    octree::OctreeTextures,
};

pub struct PhotonsToIrradiance {
    shader: Shader,
}

impl PhotonsToIrradiance {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/photonsToIrradiance.comp.glsl"),
        }
    }

    pub unsafe fn run(&self, textures: &OctreeTextures, light_view_map: GLuint) {
        self.shader.use_program();

        let config = Config::instance();

        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.shader
            .set_uint(c_str!("octreeLevel"), config.last_octree_level());

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_3D, textures.brick_pool_colors[0]); // Last level of colors
        self.shader.set_int(c_str!("brickPoolColors"), 0);

        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_3D, textures.brick_pool_photons);
        self.shader.set_int(c_str!("brickPoolPhotons"), 1);

        gl::ActiveTexture(gl::TEXTURE2);
        gl::BindTexture(gl::TEXTURE_2D, light_view_map);
        self.shader.set_int(c_str!("lightViewMap"), 2);

        helpers::bind_3d_image_texture(
            0,
            textures.brick_pool_irradiance[0],
            gl::WRITE_ONLY,
            gl::RGBA8,
        );
        helpers::bind_image_texture(1, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);

        let (viewport_width, viewport_height) = config.viewport_dimensions();

        self.shader.dispatch_xyz(vec3(
            (viewport_width as f32 / 32 as f32).ceil() as u32,
            (viewport_height as f32 / 32 as f32).ceil() as u32,
            1,
        ));
        self.shader.wait();
    }
}
