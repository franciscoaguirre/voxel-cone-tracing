use c_str_macro::c_str;
use cgmath::{vec3, Matrix4};
use gl::types::GLuint;

use crate::{
    config::CONFIG,
    constants::Axis,
    helpers,
    rendering::{framebuffer::Framebuffer, light::SpotLight, model::Model},
};

use super::Octree;

mod stages;
use stages::*;

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
        self.mipmap_photons(light_view_map);

        (light_view_map, light_view_map_view, shadow_map)
    }

    unsafe fn mipmap_photons(&self, light_view_map: GLuint) {
        let mipmap_centers = MipmapCentersPass::init(light_view_map);
        let mipmap_faces = MipmapFacesPass::init(light_view_map);
        let mipmap_corners = MipmapCornersPass::init(light_view_map);
        let mipmap_edges = MipmapEdgesPass::init(light_view_map);
        let border_transfer = BorderTransferPass::init(light_view_map);

        let all_axis = vec![Axis::X, Axis::Y, Axis::Z];
        for axis in all_axis.iter() {
            border_transfer.run(
                &self.textures,
                CONFIG.octree_levels - 1,
                &self.geometry_data.node_data,
                *axis,
            );
            border_transfer.run(
                &self.textures,
                CONFIG.octree_levels - 1,
                &self.border_data.node_data,
                *axis,
            );
         }

        for level in (0..CONFIG.octree_levels - 1).rev() {
            mipmap_centers.run(&self.textures, level);
            mipmap_faces.run(&self.textures, level);
            mipmap_corners.run(&self.textures, level);
            mipmap_edges.run(&self.textures, level);

            if level > 0 {
               for axis in all_axis.iter() {
                   border_transfer.run(
                       &self.textures,
                       level,
                       &self.geometry_data.node_data,
                       *axis,
                   );
                   border_transfer.run(
                       &self.textures,
                       level,
                       &self.border_data.node_data,
                       *axis,
                   );
               }
            }
        }
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
        helpers::bind_image_texture(2, light_view_map_view, gl::WRITE_ONLY, gl::RGBA8);
        let total_photons = helpers::generate_texture_buffer(1, gl::R32UI, 0u32);
        helpers::bind_image_texture(3, total_photons.0, gl::READ_WRITE, gl::R32UI);

        self.renderer.store_photons_shader.dispatch_xyz(vec3(
            (CONFIG.viewport_width as f32 / 32 as f32).ceil() as u32,
            (CONFIG.viewport_height as f32 / 32 as f32).ceil() as u32,
            1,
        ));
        self.renderer.store_photons_shader.wait();

        let total_photons_values =
            helpers::get_values_from_texture_buffer(total_photons.1, 1, 69u32);
        // dbg!(&total_photons_values[0]);
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
        let (light_view_map, light_view_map_view, shadow_map, _) = light.transform.take_photo(
            models,
            &projection,
            model,
            framebuffer,
            Some(self.renderer.light_view_map_shader),
        );
        gl::CullFace(gl::BACK);

        (light_view_map, light_view_map_view, shadow_map)
    }
}
