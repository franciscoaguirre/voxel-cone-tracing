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
        let input = ClearLightInput {
            brick_pool_photons: self.textures.brick_pool_photons,
            brick_pool_irradiance: self.textures.brick_pool_irradiance,
            number_of_nodes: self.number_of_nodes(),
        };
        self.builder
            .clear_light
            .run(input);
    }

    pub unsafe fn inject_light(
        &self,
        objects: &mut [Object],
        light: &Light,
        scene_aabb: &Aabb,
    ) -> (GLuint, GLuint, GLuint) {
        let (light_view_map, light_view_map_view, shadow_map) =
            self.create_light_view_map(objects, light, scene_aabb);

        let store_photons_input = StorePhotonsInput {
            light_view_map,
            node_pool: self.textures.node_pool,
            brick_pool_photons: self.textures.brick_pool_photons,
            is_directional: light.is_directional(),
        };
        self.builder
            .store_photons
            .run(store_photons_input);

        self.border_transfer(light_view_map);

        let config = Config::instance();

        self.copy_alpha_to_irradiance();

        let photons_to_irradiance_input = PhotonsToIrradianceInput {
            node_pool: self.textures.node_pool,
            brick_pool_colors_last_level: self.textures.brick_pool_colors[0],
            brick_pool_photons: self.textures.brick_pool_photons,
            brick_pool_irradiance_last_level: self.textures.brick_pool_irradiance[0],
            light_view_map,
            is_directional: light.is_directional(),
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

        self.run_mipmap(BrickPoolValues::Irradiance);

        (light_view_map, light_view_map_view, shadow_map)
    }

    #[inline]
    unsafe fn copy_alpha_to_irradiance(&self) {
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

    unsafe fn create_light_view_map(
        &self,
        objects: &mut [Object],
        light: &Light,
        scene_aabb: &Aabb,
    ) -> (GLuint, GLuint, GLuint) {
        let config = Config::instance();

        gl::CullFace(gl::FRONT);
        let light_map_buffers = light.take_photo(
            objects,
            scene_aabb,
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
