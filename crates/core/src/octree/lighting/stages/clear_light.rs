use c_str_macro::c_str;
use cgmath::vec3;
use gl::types::GLuint;
use engine::prelude::*;

use crate::{
    config::Config,
    octree::OctreeTextures,
};

/// Shader pass for clearing the photons from the SVO.
/// Used to reset the structure for recomputing light,
/// for example, when the light source moves.
pub struct ClearLight {
    uint_shader: Shader,
    float_shader: Shader,
}

impl ClearLight {
    pub fn init() -> Self {
        Self {
            uint_shader: compile_compute!("assets/shaders/octree/clearBricks.comp.glsl"),
            float_shader: compile_compute!(
                "assets/shaders/octree/clearBricksFloat.comp.glsl",
            ),
        }
    }
}

pub struct ClearLightInput {
    pub brick_pool_photons: Texture3D,
    pub brick_pool_irradiance: [Texture3D; 6],
    pub number_of_nodes: usize,
}

impl ShaderPass for ClearLight {
    type Input<'a> = ClearLightInput;

    unsafe fn run(&self, input: Self::Input<'_>) {
        let config = Config::instance();

        self.uint_shader.use_program();
        self.uint_shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

        helpers::bind_3d_image_texture(
            0,
            input.brick_pool_photons,
            gl::WRITE_ONLY,
            gl::R32UI,
        );

        let number_of_groups =
            (input.number_of_nodes as f64 / config.working_group_size as f64).ceil() as u32;

        self.uint_shader.dispatch(number_of_groups);
        self.uint_shader.wait();

        self.float_shader.use_program();
        self.float_shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

        for texture_number in 0..6 {
            helpers::bind_3d_image_texture(
                1,
                input.brick_pool_irradiance[texture_number as usize],
                gl::WRITE_ONLY,
                gl::RGBA8,
            );

            self.float_shader
                .dispatch(number_of_groups);
            self.float_shader.wait();
        }
    }
}
