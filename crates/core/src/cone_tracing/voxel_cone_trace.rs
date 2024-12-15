use std::collections::HashMap;

use c_str_macro::c_str;
use engine::prelude::*;
use serde::{Deserialize, Serialize};

use super::ConeParameters;

use crate::{config::Config, octree::OctreeTextures};

pub struct ConeTracer {
    shader: Shader,
    pub toggles: Toggles,
    framebuffer: Framebuffer<1>,
    post_processing_shader: Shader,
    processed_framebuffer: Framebuffer<1>,
}

impl ConeTracer {
    pub fn init() -> Self {
        Self {
            shader: compile_shaders!("assets/shaders/octree/coneTracing.glsl"),
            toggles: Toggles::default(),
            framebuffer: unsafe { Framebuffer::<1>::new_floating_point() },
            post_processing_shader: compile_shaders!("assets/shaders/octree/postProcessing.glsl"),
            processed_framebuffer: unsafe { Framebuffer::<1>::new() },
        }
    }

    pub unsafe fn run(
        &self,
        light: &Light,
        textures: &OctreeTextures,
        geometry_buffers: &Textures<GEOMETRY_BUFFERS>,
        light_maps: (u32, u32, u32),
        quad: &Quad,
        camera: &Camera,
        parameters: &HashMap<&str, ConeParameters>,
        screenshot_folder: Option<String>,
    ) {
        self.shader.use_program();

        let config = Config::instance();

        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.shader
            .set_uint(c_str!("maxOctreeLevel"), config.last_octree_level());
        self.shader
            .set_bool(c_str!("shouldShowColor"), self.toggles.should_show_color());
        self.shader.set_bool(
            c_str!("shouldShowDirect"),
            self.toggles.should_show_direct(),
        );
        self.shader.set_bool(
            c_str!("shouldShowIndirect"),
            self.toggles.should_show_indirect(),
        );
        self.shader.set_bool(
            c_str!("shouldShowIndirectSpecular"),
            self.toggles.should_show_indirect_specular(),
        );
        self.shader.set_bool(
            c_str!("shouldShowAmbientOcclusion"),
            self.toggles.should_show_ambient_occlusion(),
        );
        self.shader.set_vec3(
            c_str!("eyePosition"),
            camera.transform.position.x,
            camera.transform.position.y,
            camera.transform.position.z,
        );
        if light.is_directional() {
            self.shader.set_vec3(
                c_str!("directionalLight.direction"),
                light.transform().get_forward().x,
                light.transform().get_forward().y,
                light.transform().get_forward().z,
            );
            self.shader
                .set_float(c_str!("directionalLight.intensity"), light.intensity());
        } else {
            self.shader.set_vec3(
                c_str!("pointLight.position"),
                light.transform().position.x,
                light.transform().position.y,
                light.transform().position.z,
            );
            self.shader
                .set_float(c_str!("pointLight.intensity"), light.intensity());
        }
        self.shader.set_vec3(
            c_str!("pointLight.color"),
            1.0, // White
            1.0,
            1.0,
        );
        self.shader.set_float(c_str!("shininess"), 30.0); // TODO: This should be decided per material
        for (key, value) in parameters.iter() {
            value.set_uniforms(&key, &self.shader);
        }
        helpers::bind_image_texture(0, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);

        let brick_pool_textures = vec![
            (
                c_str!("brickPoolNormals"),
                textures.brick_pool_normals,
                gl::NEAREST as i32,
            ),
            // Irradiance textures
            (
                c_str!("brickPoolIrradianceX"),
                textures.brick_pool_irradiance[0],
                gl::LINEAR as i32,
            ),
            (
                c_str!("brickPoolIrradianceXNeg"),
                textures.brick_pool_irradiance[1],
                gl::LINEAR as i32,
            ),
            (
                c_str!("brickPoolIrradianceY"),
                textures.brick_pool_irradiance[2],
                gl::LINEAR as i32,
            ),
            (
                c_str!("brickPoolIrradianceYNeg"),
                textures.brick_pool_irradiance[3],
                gl::LINEAR as i32,
            ),
            (
                c_str!("brickPoolIrradianceZ"),
                textures.brick_pool_irradiance[4],
                gl::LINEAR as i32,
            ),
            (
                c_str!("brickPoolIrradianceZNeg"),
                textures.brick_pool_irradiance[5],
                gl::LINEAR as i32,
            ),
        ];

        let mut texture_counter = 0;

        for &(texture_name, texture, sample_interpolation) in brick_pool_textures.iter() {
            gl::ActiveTexture(gl::TEXTURE0 + texture_counter);
            gl::BindTexture(gl::TEXTURE_3D, texture);
            self.shader.set_int(texture_name, texture_counter as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, sample_interpolation);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, sample_interpolation);
            texture_counter += 1;
        }

        let g_buffer_textures = vec![
            (c_str!("gBufferColors"), geometry_buffers[3]),
            (c_str!("gBufferPositions"), geometry_buffers[0]),
            (c_str!("gBufferNormals"), geometry_buffers[2]),
            (c_str!("gBufferSpeculars"), geometry_buffers[4]),
        ];

        for &(texture_name, texture) in g_buffer_textures.iter() {
            gl::ActiveTexture(gl::TEXTURE0 + texture_counter);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            self.shader.set_int(texture_name, texture_counter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            texture_counter += 1;
        }

        self.shader
            .set_bool(c_str!("isDirectional"), light.is_directional());

        if self.toggles.should_show_final_image_quad() {
            self.create_image(quad); // Loads it in the framebuffer
            self.run_post_processing(quad); // Runs post processing effects on the framebuffer, stores in final framebuffer
            self.render_to_screen(quad); // Renders the framebuffer to the screen
        }

        if let Some(folder) = screenshot_folder {
            let filepath = format!("{folder}/screenshot.png");
            self.processed_framebuffer
                .save_color_attachment_to_file(0, &filepath);
        }
    }

    unsafe fn create_image(&self, quad: &Quad) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.fbo());
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::BindVertexArray(quad.get_vao());
        gl::DrawElements(
            gl::TRIANGLES,
            quad.get_num_indices() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        gl::BindVertexArray(0);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    unsafe fn run_post_processing(&self, quad: &Quad) {
        // Set uniforms
        self.post_processing_shader.use_program();
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, self.framebuffer.textures()[0]);
        self.post_processing_shader
            .set_int(c_str!("inputTexture"), 0);

        self.post_processing_shader
            .set_float(c_str!("exposure"), 0.8);

        // Framebuffer
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.processed_framebuffer.fbo());
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        // Draw using quad
        gl::BindVertexArray(quad.get_vao());
        gl::DrawElements(
            gl::TRIANGLES,
            quad.get_num_indices() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        gl::BindVertexArray(0);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    unsafe fn render_to_screen(&self, quad: &Quad) {
        quad.render(self.processed_framebuffer.textures()[0]);
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Toggles {
    should_show_color: bool,
    should_show_direct: bool,
    should_show_indirect: bool,
    should_show_indirect_specular: bool,
    should_show_ambient_occlusion: bool,
}

impl Toggles {
    pub fn should_show_color(&self) -> bool {
        self.should_show_color
    }

    pub fn toggle_color(&mut self) {
        self.should_show_color = !self.should_show_color;
    }

    pub fn should_show_direct(&self) -> bool {
        self.should_show_direct
    }

    pub fn toggle_direct(&mut self) {
        self.should_show_direct = !self.should_show_direct;
    }

    pub fn should_show_indirect(&self) -> bool {
        self.should_show_indirect
    }

    pub fn toggle_indirect(&mut self) {
        self.should_show_indirect = !self.should_show_indirect;
    }

    pub fn should_show_indirect_specular(&self) -> bool {
        self.should_show_indirect_specular
    }

    pub fn toggle_indirect_specular(&mut self) {
        self.should_show_indirect_specular = !self.should_show_indirect_specular;
    }

    pub fn should_show_ambient_occlusion(&self) -> bool {
        self.should_show_ambient_occlusion
    }

    pub fn toggle_ambient_occlusion(&mut self) {
        self.should_show_ambient_occlusion = !self.should_show_ambient_occlusion;
    }

    pub fn should_show_final_image_quad(&self) -> bool {
        self.should_show_color
            || self.should_show_direct
            || self.should_show_indirect
            || self.should_show_indirect_specular
            || self.should_show_ambient_occlusion
    }
}
