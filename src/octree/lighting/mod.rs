use c_str_macro::c_str;
use cgmath::{vec3, Matrix4};
use gl::types::GLuint;

use crate::{
    config::CONFIG,
    helpers,
    rendering::{light::SpotLight, model::Model, shader::Shader},
};

use super::Octree;

mod stages;
use stages::*;

impl Octree {
    pub unsafe fn inject_light(
        &self,
        models: &[&Model],
        light: &SpotLight,
        model: &Matrix4<f32>,
    ) -> GLuint {
        let (light_view_map, light_view_map_view) =
            Self::create_light_view_map(models, light, model);
        self.store_photons(light_view_map, light_view_map_view);
        self.mipmap_photons(light_view_map);

        light_view_map_view
    }

    unsafe fn mipmap_photons(&self, light_view_map: GLuint) {
        let mipmap_centers = MipmapCentersPass::init(light_view_map);
        let mipmap_faces = MipmapFacesPass::init(light_view_map);
        let mipmap_corners = MipmapCornersPass::init(light_view_map);
        let mipmap_edges = MipmapEdgesPass::init(light_view_map);
        let border_transfer = BorderTransferPass::init(light_view_map);

        border_transfer.run(
            &self.textures,
            CONFIG.octree_levels - 1,
            &self.nodes_per_level,
        );

        for level in (0..CONFIG.octree_levels - 1).rev() {
            mipmap_centers.run(&self.textures, level);
            mipmap_faces.run(&self.textures, level);
            mipmap_corners.run(&self.textures, level);
            mipmap_edges.run(&self.textures, level);

            // TODO: Fix in higher levels
            if level > 0 {
                border_transfer.run(&self.textures, level, &self.nodes_per_level);
            }
        }
    }

    unsafe fn store_photons(&self, light_view_map: GLuint, light_view_map_view: GLuint) {
        let shader = Shader::new_compute("assets/shaders/octree/storePhotons.comp.glsl");

        shader.use_program();

        shader.set_uint(c_str!("octreeLevel"), CONFIG.octree_levels - 1);
        shader.set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, light_view_map);
        shader.set_int(c_str!("lightViewMap"), 0);

        helpers::bind_image_texture(0, self.textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_3d_image_texture(
            1,
            self.textures.brick_pool_photons,
            gl::READ_WRITE,
            gl::R32UI,
        );
        helpers::bind_image_texture(2, light_view_map_view, gl::WRITE_ONLY, gl::RGBA8);

        shader.dispatch_xyz(vec3(
            (CONFIG.viewport_width as f32 / 32 as f32).ceil() as u32,
            (CONFIG.viewport_height as f32 / 32 as f32).ceil() as u32,
            1,
        ));
        shader.wait();
    }

    unsafe fn create_light_view_map(
        models: &[&Model],
        light: &SpotLight,
        model: &Matrix4<f32>,
    ) -> (GLuint, GLuint) {
        let projection = light.get_projection_matrix();

        let (light_view_map, light_view_map_view, _) =
            light.transform.take_photo(models, &projection, model);

        (light_view_map, light_view_map_view)
    }
}
