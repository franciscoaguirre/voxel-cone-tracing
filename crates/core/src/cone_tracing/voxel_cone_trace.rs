use std::collections::HashMap;

use c_str_macro::c_str;
use cgmath::vec3;
use serde::{Serialize, Deserialize};
use engine::prelude::*;

use super::ConeParameters;

use crate::{
    config::Config,
    octree::OctreeTextures,
};

pub struct ConeTracer {
    shader: Shader,
    pub toggles: Toggles,
    framebuffer: Framebuffer<1>,
    max_color_shader: Shader,
    post_processing_shader: Shader,
    processed_framebuffer: Framebuffer<1>,
}

pub struct VisualTestsParameters<'a> {
    pub screenshot_name: &'a str,
    pub should_update: bool,
}

impl ConeTracer {
    pub fn init() -> Self {
        Self {
            shader: compile_shaders!("assets/shaders/octree/coneTracing.glsl"),
            toggles: Toggles::default(),
            framebuffer: unsafe { Framebuffer::<1>::new() },
            max_color_shader: compile_compute!("assets/shaders/octree/getMaxColor.comp.glsl"),
            post_processing_shader: compile_shaders!("assets/shaders/octree/postProcessing.glsl"),
            processed_framebuffer: unsafe { Framebuffer::<1>::new() },
        }
    }

    pub unsafe fn run(
        &self,
        light: &Light,
        textures: &OctreeTextures,
        geometry_buffers: &Textures<GEOMETRY_BUFFERS>,
        light_maps: &Textures<LIGHT_MAP_BUFFERS>,
        quad: &Quad,
        camera: &Camera,
        parameters: &HashMap<&str, ConeParameters>,
        visual_tests_parameters: Option<&VisualTestsParameters>,
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
                light.transform().position.x,
                light.transform().position.y,
                light.transform().position.z,
            );
        } else {
            self.shader.set_vec3(
                c_str!("pointLight.position"),
                light.transform().position.x,
                light.transform().position.y,
                light.transform().position.z,
            );
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
        self.shader.bind_image_texture(0, textures.node_pool, TextureAccess::ReadOnly);

        let brick_pool_textures = vec![
            // Irradiance textures
            (
                "brickPoolIrradianceX",
                textures.brick_pool_irradiance[0],
            ),
            (
                "brickPoolIrradianceXNeg",
                textures.brick_pool_irradiance[1],
            ),
            (
                "brickPoolIrradianceY",
                textures.brick_pool_irradiance[2],
            ),
            (
                "brickPoolIrradianceYNeg",
                textures.brick_pool_irradiance[3],
            ),
            (
                "brickPoolIrradianceZ",
                textures.brick_pool_irradiance[4],
            ),
            (
                "brickPoolIrradianceZNeg",
                textures.brick_pool_irradiance[5],
            ),
        ];

        for &(texture_name, texture) in brick_pool_textures.iter() {
            self.shader.bind_3d_texture(texture_name, texture, false);
        }

        let g_buffer_textures = vec![
            ("gBufferColors", geometry_buffers[3]),
            ("gBufferPositions", geometry_buffers[0]),
            ("gBufferNormals", geometry_buffers[2]),
            ("gBufferSpeculars", geometry_buffers[4]),
        ];

        for &(texture_name, texture) in g_buffer_textures.iter() {
            self.shader.bind_texture(texture_name, texture, false);
        }

        self.shader.set_bool(c_str!("isDirectional"), light.is_directional());

        if self.toggles.should_show_final_image_quad() {
            self.create_image(quad); // Loads it in the framebuffer
            let max_color_norm = self.calculate_max_color_norm();
            self.run_post_processing(quad, max_color_norm); // Runs post processing effects on the framebuffer, stores in final framebuffer
            self.render_to_screen(quad); // Renders the framebuffer to the screen
        }

        if let Some(VisualTestsParameters {
            screenshot_name,
            should_update
        }) = visual_tests_parameters {
            let filepath = format!("screenshots/{screenshot_name}.png");
            if *should_update {
                self.processed_framebuffer.save_color_attachment_to_file(0, &filepath);
            } else {
                let result = self.processed_framebuffer.compare_attachment_to_file(0, &filepath)
                    .expect("Image not found for comparing. Generate it with `--update-sceenshots` first.");
                assert!(
                    result,
                    "Generated image is not the same as the one in `screenshots`.
                    Make sure to update the screenshot with `--update-screenshots` if this was intended.
                    If not, make sure to fix what you broke :)",
                );
            }
        }
    }

    unsafe fn create_image(&self, quad: &Quad) {
        self.framebuffer.bind();
        quad.draw(true);
        self.framebuffer.unbind();
    }

    unsafe fn calculate_max_color_norm(&self) -> f32 {
        self.max_color_shader.use_program();
        self.max_color_shader.bind_texture("inputTexture", self.framebuffer.textures()[0], false);
        let max_color_texture = BufferTextureV2::from_data(&vec![0u32]);
        self.max_color_shader.bind_image_texture(0, max_color_texture, TextureAccess::ReadWrite);
        let config = Config::instance();
        let (viewport_width, viewport_height) = config.viewport_dimensions();
        self.max_color_shader.dispatch_xyz(vec3(
            viewport_width as u32,
            viewport_height as u32,
            1,
        ));
        self.max_color_shader.wait();
        let max_color_norm = max_color_texture.data()[0];
        let normalized_max_color_norm = max_color_norm as f32 / 1000.0; // This same value is used in the shader
        normalized_max_color_norm
    }

    unsafe fn run_post_processing(&self, quad: &Quad, max_color_norm: f32) {
        // Set uniforms
        self.post_processing_shader.use_program();
        // TODO: Maybe it's not false
        self.post_processing_shader.bind_texture("inputTexture", self.framebuffer.textures()[0], false);
        self.post_processing_shader.set_float(c_str!("maxNorm"), max_color_norm);

        // Framebuffer
        self.processed_framebuffer.bind();
        quad.draw(true);
        self.processed_framebuffer.unbind();
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
