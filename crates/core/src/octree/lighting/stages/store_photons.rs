use c_str_macro::c_str;
use cgmath::vec3;
use gl::types::GLuint;
use engine::prelude::*;

use crate::{
    config::Config,
    octree::OctreeTextures,
};

pub struct StorePhotons {
    shader: Shader,
}

impl StorePhotons {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/storePhotons.comp.glsl"),
        }
    }
}

pub struct StorePhotonsInput {
    pub light_view_map: Texture2D,
    pub node_pool: BufferTexture,
    pub brick_pool_photons: Texture3D,
}

impl ShaderPass for StorePhotons {
    type Input = StorePhotonsInput;

    unsafe fn run(&self, input: Self::Input) {
        self.shader.use_program();

        let config = Config::instance();

        self.shader
            .set_uint(c_str!("octreeLevel"), config.last_octree_level());
        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, input.light_view_map);
        self.shader
            .set_int(c_str!("lightViewMap"), 0);

        helpers::bind_image_texture(0, input.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_3d_image_texture(
            1,
            input.brick_pool_photons,
            gl::READ_WRITE,
            gl::R32UI,
        );
        let total_photons = helpers::generate_texture_buffer(1, gl::R32UI, 0u32);
        helpers::bind_image_texture(2, total_photons.0, gl::READ_WRITE, gl::R32UI);

        let (viewport_width, viewport_height) = config.viewport_dimensions();

        self.shader.dispatch_xyz(vec3(
            (viewport_width as f32 / 32 as f32).ceil() as u32,
            (viewport_height as f32 / 32 as f32).ceil() as u32,
            1,
        ));
        self.shader.wait();

        let total_photons_values =
            helpers::get_values_from_texture_buffer(total_photons.1, 1, 69u32);
        log::debug!("Total photons in scene: {}", total_photons_values[0]);
    }
}
