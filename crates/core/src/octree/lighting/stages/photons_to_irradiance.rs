use c_str_macro::c_str;
use cgmath::vec3;
use gl::types::GLuint;
use engine::prelude::*;

use crate::{
    config::Config,
    octree::OctreeTextures,
};

pub struct PhotonsToIrradiance {
    directional_shader: Shader,
    point_shader: Shader,
}

impl PhotonsToIrradiance {
    pub fn init() -> Self {
        Self {
            directional_shader: compile_compute!("assets/shaders/octree/photonsToIrradianceDirectional.comp.glsl"),
            point_shader: compile_compute!("assets/shaders/octree/photonsToIrradiancePoint.comp.glsl"),
        }
    }
}

pub struct PhotonsToIrradianceInput {
    pub node_pool: BufferTexture,
    pub brick_pool_colors_last_level: Texture3D,
    pub brick_pool_photons: Texture3D,
    pub brick_pool_irradiance_last_level: Texture3D,
    pub light_view_map: Texture2D,
    pub is_directional: bool,
}

impl ShaderPass for PhotonsToIrradiance {
    type Input<'a> = PhotonsToIrradianceInput;

    unsafe fn run(&self, input: Self::Input<'_>) {
        let shader = if input.is_directional { self.directional_shader } else { self.point_shader };
        let config = Config::instance();

        shader.use_program();
        shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        shader
            .set_uint(c_str!("octreeLevel"), config.last_octree_level());
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_3D, input.brick_pool_colors_last_level);
        shader.set_int(c_str!("brickPoolColors"), 0);
        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_3D, input.brick_pool_photons);
        shader.set_int(c_str!("brickPoolPhotons"), 1);
        gl::ActiveTexture(gl::TEXTURE2);
        if input.is_directional {
            gl::BindTexture(gl::TEXTURE_2D, input.light_view_map);
        } else {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, input.light_view_map);
        }
        shader.set_int(c_str!("lightViewMap"), 2);
        helpers::bind_3d_image_texture(
            0,
            input.brick_pool_irradiance_last_level,
            gl::WRITE_ONLY,
            gl::RGBA8,
        );
        helpers::bind_image_texture(1, input.node_pool.0, gl::READ_ONLY, gl::R32UI);

        let (viewport_width, viewport_height) = config.viewport_dimensions();
        let local_group_size = if input.is_directional { 32 } else { 12 };
        shader.dispatch_xyz(vec3(
            (viewport_width as f32 / local_group_size as f32).ceil() as u32,
            (viewport_height as f32 / local_group_size as f32).ceil() as u32,
            1,
        ));
        shader.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::env;

    #[test]
    fn it_works() {
        let (_glfw, _window) = test_utils::init_opengl_context();

        // To go from the crate root to the workspace root
        let mut path = PathBuf::from(env::current_dir().unwrap());
        path.pop();
        path.pop();
        env::set_current_dir(path).unwrap();

        // let textures = PhotonsToIrradianceInput {
        //     node_pool: ,
        // };
    }
}
