use c_str_macro::c_str;
use cgmath::{vec3, Matrix4};
use gl::types::GLuint;
use engine::prelude::*;

use crate::{
    config::Config,
    constants::Axis,
};

use super::{
    build::{BrickPoolValues, MipmapAnisotropicPass},
    Octree,
};

mod stages;
pub use stages::*;

impl Octree {
    pub unsafe fn clear_light(&self) {
        let config = Config::instance();

        self.renderer.clear_bricks_shader.use_program();
        self.renderer
            .clear_bricks_shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

        helpers::bind_3d_image_texture(
            0,
            self.textures.brick_pool_photons,
            gl::WRITE_ONLY,
            gl::R32UI,
        );

        let number_of_groups =
            (self.number_of_nodes() as f64 / config.working_group_size as f64).ceil() as u32;

        self.renderer.clear_bricks_shader.dispatch(number_of_groups);
        self.renderer.clear_bricks_shader.wait();

        self.renderer.clear_bricks_float_shader.use_program();
        self.renderer
            .clear_bricks_float_shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

        for texture_number in 0..6 {
            helpers::bind_3d_image_texture(
                1,
                self.textures.brick_pool_irradiance[texture_number as usize],
                gl::WRITE_ONLY,
                gl::RGBA8,
            );

            self.renderer
                .clear_bricks_float_shader
                .dispatch(number_of_groups);
            self.renderer.clear_bricks_float_shader.wait();
        }
    }

    pub unsafe fn inject_light(
        &self,
        objects: &mut [Object],
        light: &SpotLight,
        scene_aabb: &Aabb,
        framebuffer: &LightFramebuffer,
    ) -> (GLuint, GLuint, GLuint) {
        let (light_view_map, light_view_map_view, shadow_map) =
            self.create_light_view_map(objects, light, scene_aabb, framebuffer);

        let store_photons_input = StorePhotonsInput {
            light_view_map,
            node_pool: self.textures.node_pool,
            brick_pool_photons: self.textures.brick_pool_photons,
        };
        self.builder
            .store_photons
            .run(store_photons_input);

        self.border_transfer(light_view_map);

        // TODO: Refactorear todos estos métodos a su propia stage.

        let config = Config::instance();

        gl::CopyImageSubData(
            self.textures.brick_pool_alpha,
            gl::TEXTURE_3D,
            0,
            0,
            0,
            0,
            self.textures.brick_pool_irradiance[0],
            gl::TEXTURE_3D,
            0,
            0,
            0,
            0,
            config.brick_pool_resolution as i32,
            config.brick_pool_resolution as i32,
            config.brick_pool_resolution as i32,
        );

        let photons_to_irradiance_input = PhotonsToIrradianceInput {
            node_pool: self.textures.node_pool,
            brick_pool_colors_last_level: self.textures.brick_pool_colors[0],
            brick_pool_photons: self.textures.brick_pool_photons,
            brick_pool_irradiance_last_level: self.textures.brick_pool_irradiance[0],
            light_view_map,
        };
        self.builder
            .photons_to_irradiance_pass
            .run(photons_to_irradiance_input);

        self.builder.spread_leaf_bricks_pass.run(
            &self.textures,
            &self.geometry_data.node_data,
            BrickPoolValues::Irradiance,
        );

        self.builder.leaf_border_transfer_pass.run(
            &self.textures,
            &self.geometry_data.node_data,
            &self.border_data.node_data,
            BrickPoolValues::Irradiance,
        );

        self.mipmap_photons(light_view_map);

        (light_view_map, light_view_map_view, shadow_map)
    }

    unsafe fn border_transfer(&self, light_view_map: GLuint) {
        let border_transfer = BorderTransferPass::init(light_view_map);

        let config = Config::instance();

        for axis in Axis::all_axis().iter() {
            border_transfer.run(
                &self.textures,
                config.last_octree_level(),
                &self.geometry_data.node_data,
                *axis,
            );
        }
    }

    unsafe fn mipmap_photons(&self, light_view_map: GLuint) {
        let mipmap = MipmapAnisotropicPass::init();
        self.run_mipmap(BrickPoolValues::Irradiance);
    }

    unsafe fn create_light_view_map(
        &self,
        objects: &mut [Object],
        light: &SpotLight,
        scene_aabb: &Aabb,
        framebuffer: &LightFramebuffer,
    ) -> (GLuint, GLuint, GLuint) {
        let projection = light.get_projection_matrix();

        let config = Config::instance();

        gl::CullFace(gl::FRONT);
        let light_map_buffers = light.transform.take_photo(
            objects,
            &projection,
            scene_aabb,
            framebuffer,
            Some(self.renderer.light_view_map_shader),
            config.voxel_dimension(),
        );
        gl::CullFace(gl::BACK);

        (
            light_map_buffers[0],
            light_map_buffers[1],
            light_map_buffers[2],
        )
    }
}
