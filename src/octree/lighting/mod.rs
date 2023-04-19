use c_str_macro::c_str;
use cgmath::{vec3, Matrix4};
use gl::types::GLuint;

use crate::{
    config::CONFIG,
    helpers,
    rendering::{model::Model, shader::Shader},
};

use super::Octree;

mod stages;
use stages::*;

impl Octree {
    pub unsafe fn inject_light(
        &self,
        models: &[&Model],
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) -> GLuint {
        let (light_view_map, light_view_map_view) =
            Self::create_light_view_map(models, projection, view, model);
        self.store_photons(light_view_map);
        self.mipmap_photons(light_view_map);

        light_view_map_view
    }

    unsafe fn mipmap_photons(&self, light_view_map: GLuint) {
        let mipmap_centers = MipmapCentersPass::init(light_view_map);
        let mipmap_faces = MipmapFacesPass::init(light_view_map);
        let mipmap_corners = MipmapCornersPass::init(light_view_map);
        let mipmap_edges = MipmapEdgesPass::init(light_view_map);
        let border_transfer = BorderTransferPass::init(light_view_map);

        border_transfer.run(&self.textures, CONFIG.octree_levels - 1);

        for level in (0..CONFIG.octree_levels - 1).rev() {
            mipmap_centers.run(&self.textures, level);
            mipmap_faces.run(&self.textures, level);
            mipmap_corners.run(&self.textures, level);
            mipmap_edges.run(&self.textures, level);

            // TODO: Fix in higher levels
            if level > 0 {
                border_transfer.run(&self.textures, level);
            }
        }

        // border_transfer.run(&self.textures, 0);
    }

    unsafe fn store_photons(&self, light_view_map: GLuint) {
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

        shader.dispatch_xyz(vec3(
            (CONFIG.viewport_width as f32 / 32 as f32).ceil() as u32,
            (CONFIG.viewport_height as f32 / 32 as f32).ceil() as u32,
            1,
        ));
        shader.wait();
    }

    unsafe fn create_light_view_map(
        models: &[&Model],
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) -> (GLuint, GLuint) {
        // TODO: Compile beforehand once we have dynamic lights/objects
        let shader = Shader::new(
            "assets/shaders/octree/lightViewMap.vert.glsl",
            "assets/shaders/octree/lightViewMap.frag.glsl",
        );

        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        shader.use_program();
        shader.set_mat4(c_str!("projection"), &projection);
        shader.set_mat4(c_str!("view"), &view);
        shader.set_mat4(c_str!("model"), &model);
        shader.set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        let mut fbo = 0;
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut light_view_map_textures = [0; 2]; // First one is rgb10_a2ui, second rgba8 for viewing
        gl::GenTextures(2, light_view_map_textures.as_mut_ptr());

        gl::BindTexture(gl::TEXTURE_2D, light_view_map_textures[0]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB10_A2UI as i32,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
            0,
            gl::RGBA_INTEGER,
            gl::UNSIGNED_INT_2_10_10_10_REV,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::BindTexture(gl::TEXTURE_2D, light_view_map_textures[1]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        let mut rbo = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
        );
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo,
        );
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            light_view_map_textures[0],
            0,
        );
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT1,
            gl::TEXTURE_2D,
            light_view_map_textures[1],
            0,
        );

        gl::DrawBuffers(2, [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1].as_ptr());

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER: Framebuffer is not complete!");
        }

        gl::Enable(gl::DEPTH_TEST);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        for model in models {
            model.draw(&shader);
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        (light_view_map_textures[0], light_view_map_textures[1])
    }
}
