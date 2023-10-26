use c_str_macro::c_str;
use cgmath::vec3;
use gl::types::GLuint;
use engine::prelude::*;

use crate::{
    config::Config,
    octree::OctreeTextures,
};

pub struct StorePhotons {
    point_shader: Shader,
    directional_shader: Shader,
}

impl StorePhotons {
    pub fn init() -> Self {
        Self {
            point_shader: compile_compute!("assets/shaders/octree/storePhotonsPoint.comp.glsl"),
            directional_shader: compile_compute!("assets/shaders/octree/storePhotonsDirectional.comp.glsl"),
        }
    }
}

pub struct StorePhotonsInput {
    pub light_view_map: Texture2D,
    pub node_pool: BufferTexture,
    pub brick_pool_photons: Texture3D,
    pub is_directional: bool,
}

impl ShaderPass for StorePhotons {
    type Input<'a> = StorePhotonsInput;

    unsafe fn run(&self, input: Self::Input<'_>) {
        let shader = if input.is_directional { self.directional_shader } else { self.point_shader };

        shader.use_program();
        let config = Config::instance();
        shader
            .set_uint(c_str!("octreeLevel"), config.last_octree_level());
        shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        gl::ActiveTexture(gl::TEXTURE0);
        if input.is_directional {
            gl::BindTexture(gl::TEXTURE_2D, input.light_view_map);
        } else {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, input.light_view_map);
        }
        shader
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
        let local_group_size = if input.is_directional { 32 } else { 12 };
        shader.dispatch_xyz(vec3(
            (viewport_width as f32 / local_group_size as f32).ceil() as u32,
            (viewport_height as f32 / local_group_size as f32).ceil() as u32,
            1,
        ));
        shader.wait();

        let total_photons_values =
            helpers::get_values_from_texture_buffer(total_photons.1, 1, 69u32);
        log::debug!("Total photons in scene: {}", total_photons_values[0]);
    }
}
