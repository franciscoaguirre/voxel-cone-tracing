use c_str_macro::c_str;
use cgmath::vec3;
use gl::types::GLuint;
use engine::prelude::*;

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
}

pub struct PhotonsToIrradianceInput {
    pub node_pool: BufferTexture,
    pub brick_pool_colors_last_level: Texture3D,
    pub brick_pool_photons: Texture3D,
    pub brick_pool_irradiance_last_level: Texture3D,
    pub light_view_map: Texture2D,
}

impl ShaderPass for PhotonsToIrradiance {
    type Input = PhotonsToIrradianceInput;

    unsafe fn run(&self, input: Self::Input) {
        self.shader.use_program();

        let config = Config::instance();

        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.shader
            .set_uint(c_str!("octreeLevel"), config.last_octree_level());

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_3D, input.brick_pool_colors_last_level);
        self.shader.set_int(c_str!("brickPoolColors"), 0);

        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_3D, input.brick_pool_photons);
        self.shader.set_int(c_str!("brickPoolPhotons"), 1);

        gl::ActiveTexture(gl::TEXTURE2);
        gl::BindTexture(gl::TEXTURE_2D, input.light_view_map);
        self.shader.set_int(c_str!("lightViewMap"), 2);

        helpers::bind_3d_image_texture(
            0,
            input.brick_pool_irradiance_last_level,
            gl::WRITE_ONLY,
            gl::RGBA8,
        );
        helpers::bind_image_texture(1, input.node_pool.0, gl::READ_ONLY, gl::R32UI);

        let (viewport_width, viewport_height) = config.viewport_dimensions();

        self.shader.dispatch_xyz(vec3(
            (viewport_width as f32 / 32 as f32).ceil() as u32,
            (viewport_height as f32 / 32 as f32).ceil() as u32,
            1,
        ));
        self.shader.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
