use c_str_macro::c_str;
use cgmath::{vec3, Matrix4};
use engine::prelude::*;
use gl::types::*;

use crate::config::Config;

#[derive(Pausable)]
pub struct VoxelVisualizer {
    shader: Shader,
    vao: GLuint,
    paused: bool,
}

impl VoxelVisualizer {
    pub fn new() -> Self {
        Self {
            shader: compile_shaders!(
                "assets/shaders/voxel_fragment/renderVoxel.vert.glsl",
                "assets/shaders/voxel_fragment/renderVoxel.frag.glsl",
                "assets/shaders/voxel_fragment/renderVoxel.geom.glsl",
            ),
            vao: unsafe {
                let mut vao = 0;
                gl::GenVertexArrays(1, &mut vao);

                vao
            },
            paused: false,
        }
    }
}

impl System for VoxelVisualizer {
    unsafe fn setup(&mut self, _assets: &mut AssetRegistry) {}

    unsafe fn update(&mut self, inputs: SystemInputs) {
        gl::Enable(gl::DEPTH_TEST);

        self.shader.use_program();

        let voxel_positions_texture = *inputs.assets.get_texture("voxel_positions").unwrap();
        let voxel_colors_texture = *inputs.assets.get_texture("voxel_colors").unwrap();
        let Uniform::Uint(number_of_voxel_fragments) = *inputs
            .assets
            .get_uniform("SVOVoxelizer", "number_of_voxel_fragments")
            .unwrap()
        else {
            panic!("number_of_voxel_fragments should be a uint")
        };

        gl::BindImageTexture(
            0,
            voxel_positions_texture,
            0,
            gl::TRUE,
            0,
            gl::READ_ONLY,
            gl::RGB10_A2UI,
        );

        gl::BindImageTexture(
            1,
            voxel_colors_texture,
            0,
            gl::TRUE,
            0,
            gl::READ_ONLY,
            gl::RGBA8,
        );

        let camera = inputs.scene.cameras[inputs.scene.active_camera.unwrap_or(0)].borrow();

        self.shader
            .set_mat4(c_str!("projection"), &camera.get_projection_matrix());
        self.shader
            .set_mat4(c_str!("view"), &camera.transform.get_view_matrix());
        self.shader.set_mat4(
            c_str!("model"),
            &Matrix4::<f32>::from_translation(vec3(0., 0., 0.)),
        );

        let config = Config::instance();
        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.shader.set_float(
            c_str!("halfDimension"),
            1.0 / config.voxel_dimension() as f32,
        );

        gl::BindVertexArray(self.vao);
        gl::DrawArrays(gl::POINTS, 0, number_of_voxel_fragments as i32);
    }

    fn get_info(&self) -> SystemInfo {
        SystemInfo {
            name: "SVOVoxelVisualizer",
        }
    }
}
