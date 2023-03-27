use c_str_macro::c_str;
use cgmath::Matrix4;
use gl::types::GLuint;

use crate::{
    config::CONFIG,
    rendering::{model::Model, shader::Shader},
};

use super::Octree;

impl Octree {
    pub unsafe fn inject_light(
        &self,
        models: &[&Model],
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) -> GLuint {
        let shader = Shader::new(
            "assets/shaders/octree/lightViewMap.vert.glsl",
            "assets/shaders/octree/lightViewMap.frag.glsl",
        );

        shader.use_program();
        shader.set_mat4(c_str!("projection"), &projection);
        shader.set_mat4(c_str!("view"), &view);
        shader.set_mat4(c_str!("model"), &model);

        shader.set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        shader.set_uint(c_str!("octreeLevel"), CONFIG.octree_levels - 1); // Last level

        let mut fbo = 0;
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
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

        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            texture,
            0,
        );

        let mut rbo = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
        );
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo,
        );
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER: Framebuffer is not complete!");
        }

        for model in models {
            model.draw(&shader);
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        texture
    }
}
