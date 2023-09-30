use c_str_macro::c_str;
use cgmath::{vec3, Matrix4};
use gl::types::GLuint;

use crate::{
    config::CONFIG,
    constants::Axis,
    helpers,
    rendering::{framebuffer::Framebuffer, light::SpotLight, model::Model},
};

use super::{
    build::{BrickPoolValues, MipmapAnisotropicPass},
    Octree,
};

mod stages;
pub use stages::*;

impl Octree {
    pub unsafe fn clear_light(&self) {
        self.renderer.clear_bricks_shader.use_program();
        self.renderer
            .clear_bricks_shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        helpers::bind_3d_image_texture(
            0,
            self.textures.brick_pool_photons,
            gl::WRITE_ONLY,
            gl::R32UI,
        );

        let number_of_groups =
            (self.number_of_nodes() as f64 / CONFIG.working_group_size as f64).ceil() as u32;

        self.renderer.clear_bricks_shader.dispatch(number_of_groups);
        self.renderer.clear_bricks_shader.wait();

        self.renderer.clear_bricks_float_shader.use_program();
        self.renderer
            .clear_bricks_float_shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

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
        models: &[&Model],
        light: &SpotLight,
        model: &Matrix4<f32>,
        framebuffer: &Framebuffer,
    ) -> (GLuint, GLuint, GLuint) {
        let (light_view_map, light_view_map_view, shadow_map) =
            self.create_light_view_map(models, light, model, framebuffer);
        self.store_photons(light_view_map, light_view_map_view);
        // Transfer light, to get consistent colors across shared voxels later
        self.border_transfer(light_view_map);

        // TODO: Refactorear todos estos métodos a su propia stage.

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
            CONFIG.brick_pool_resolution as i32,
            CONFIG.brick_pool_resolution as i32,
            CONFIG.brick_pool_resolution as i32,
        );

        self.builder
          .photons_to_irradiance_pass
          .run(&self.textures, light_view_map);

        self.builder.spread_leaf_bricks_pass.run(
            &self.textures,
            &self.geometry_data.node_data,
            BrickPoolValues::Irradiance,
        );

        // The "usual" border transfer in the last level, to make shared voxels consistent
        self.builder.leaf_border_transfer_pass.run(
            &self.textures,
            &self.geometry_data.node_data,
            &self.border_data.node_data,
            BrickPoolValues::Irradiance,
        );

        //self.mipmap_photons(light_view_map);

        (light_view_map, light_view_map_view, shadow_map)
    }

    unsafe fn border_transfer(&self, light_view_map: GLuint) {
        // TODO: performance -- Doing compilation every time?
        let border_transfer = BorderTransferPass::init(light_view_map);

        for axis in Axis::all_axis().iter() {
            border_transfer.run(
                &self.textures,
                CONFIG.last_octree_level,
                &self.geometry_data.node_data,
                *axis,
            );
        }
    }

    unsafe fn mipmap_photons(&self, light_view_map: GLuint) {
        // TODO: performance -- Doing compilation every time?
        self.run_mipmap(BrickPoolValues::Irradiance);
    }

    unsafe fn store_photons(&self, light_view_map: GLuint, light_view_map_view: GLuint) {
        self.renderer.store_photons_shader.use_program();

        self.renderer
            .store_photons_shader
            .set_uint(c_str!("octreeLevel"), CONFIG.octree_levels - 1);
        self.renderer
            .store_photons_shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, light_view_map);
        self.renderer
            .store_photons_shader
            .set_int(c_str!("lightViewMap"), 0);

        helpers::bind_image_texture(0, self.textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_3d_image_texture(
            1,
            self.textures.brick_pool_photons,
            gl::READ_WRITE,
            gl::R32UI,
        );
        let total_photons = helpers::generate_texture_buffer(1, gl::R32UI, 0u32);
        helpers::bind_image_texture(2, total_photons.0, gl::READ_WRITE, gl::R32UI);

        self.renderer.store_photons_shader.dispatch_xyz(vec3(
            (CONFIG.viewport_width as f32 / 32 as f32).ceil() as u32,
            (CONFIG.viewport_height as f32 / 32 as f32).ceil() as u32,
            1,
        ));
        self.renderer.store_photons_shader.wait();

        let total_photons_values =
            helpers::get_values_from_texture_buffer(total_photons.1, 1, 69u32);
        dbg!(&total_photons_values[0]);
    }

    unsafe fn create_light_view_map(
        &self,
        models: &[&Model],
        light: &SpotLight,
        model: &Matrix4<f32>,
        framebuffer: &Framebuffer,
    ) -> (GLuint, GLuint, GLuint) {
        let projection = light.get_projection_matrix();

        gl::CullFace(gl::FRONT);
        let geometry_buffers = light.transform.take_photo(
            models,
            &projection,
            model,
            framebuffer,
            Some(self.renderer.light_view_map_shader),
        );
        gl::CullFace(gl::BACK);

        (
            geometry_buffers.raw_positions(),
            geometry_buffers.positions(),
            geometry_buffers.normals(), // We use the normals texture for the shadow map
        )
    }
}
