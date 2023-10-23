use c_str_macro::c_str;
use cgmath::vec3;
use serde::{Serialize, Deserialize};
use engine::prelude::*;

use crate::{
    config::Config,
    octree::OctreeTextures,
};

pub struct ConeTracer {
    shader: Shader,
    pub toggles: Toggles,
}

impl ConeTracer {
    pub fn init() -> Self {
        Self {
            shader: compile_shaders!("assets/shaders/octree/coneTracing.glsl"),
            toggles: Toggles::default(),
        }
    }

    pub unsafe fn run(
        &self,
        light: &SpotLight,
        half_cone_angle: f32,
        textures: &OctreeTextures,
        geometry_buffers: &Textures<GEOMETRY_BUFFERS>,
        light_maps: (u32, u32, u32),
        quad: &Quad,
        camera: &Camera,
        visual_tests_data: Option<(&str, &Framebuffer<1>, bool)>, // When specified, will write to a framebuffer instead of to screen, and save the image to disk
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
        let light_direction = vec3(
            light.transform.position.x,
            light.transform.position.y,
            light.transform.position.z,
        );
        self.shader.set_vec3(
            c_str!("lightDirection"),
            light_direction.x,
            light_direction.y,
            light_direction.z,
        );
        self.shader.set_float(c_str!("shininess"), 30.0);
        self.shader.set_mat4(
            c_str!("lightViewMatrix"),
            &light.transform.get_view_matrix(),
        );
        self.shader.set_mat4(
            c_str!("lightProjectionMatrix"),
            &light.get_projection_matrix(),
        );
        self.shader
            .set_float(c_str!("halfConeAngle"), half_cone_angle as f32);
        helpers::bind_image_texture(0, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);

        let brick_pool_textures = vec![
            (
                c_str!("brickPoolColorsX"),
                textures.brick_pool_colors[0],
                gl::LINEAR as i32,
            ),
            (
                c_str!("brickPoolColorsXNeg"),
                textures.brick_pool_colors[1],
                gl::LINEAR as i32,
            ),
            (
                c_str!("brickPoolColorsY"),
                textures.brick_pool_colors[2],
                gl::LINEAR as i32,
            ),
            (
                c_str!("brickPoolColorsYNeg"),
                textures.brick_pool_colors[3],
                gl::LINEAR as i32,
            ),
            (
                c_str!("brickPoolColorsZ"),
                textures.brick_pool_colors[4],
                gl::LINEAR as i32,
            ),
            (
                c_str!("brickPoolColorsZNeg"),
                textures.brick_pool_colors[5],
                gl::LINEAR as i32,
            ),
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

        gl::ActiveTexture(gl::TEXTURE0 + texture_counter);
        // gl::BindTexture(gl::TEXTURE_2D, light_maps.2);
        // self.shader
        //     .set_int(c_str!("shadowMap"), texture_counter as i32);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        let quad_vao = quad.get_vao();
        if self.toggles.should_show_final_image_quad() {
            if let Some((_, framebuffer, _)) = visual_tests_data {
                gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer.fbo());
                gl::Enable(gl::DEPTH_TEST);
                gl::ClearColor(0.0, 0.0, 0.0, 0.0);
                gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
            gl::BindVertexArray(quad_vao);
            gl::DrawElements(
                gl::TRIANGLES,
                quad.get_num_indices() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
            if let Some((_, framebuffer, _)) = visual_tests_data {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            }
        }

        if let Some((filename, framebuffer, should_update)) = visual_tests_data {
            let filepath = format!("screenshots/{filename}.png");
            if should_update {
                framebuffer.save_color_attachment_to_file(0, &filepath);
            } else {
                let result = framebuffer.compare_attachment_to_file(0, &filepath)
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
