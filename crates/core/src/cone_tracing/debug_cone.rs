use engine::prelude::*;
use std::collections::HashSet;
use std::fmt;

use c_str_macro::c_str;
use cgmath::{point3, vec3, Matrix4, Vector2, Vector3};
use colored::{customcolors, Colorize};

use gl::types::GLuint;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DebugNode {
    index: u32,
    text: String,
}

impl DebugNode {
    pub fn new(index: u32, text: String) -> Self {
        Self { index, text }
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

impl fmt::Display for DebugNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.index, self.text)
    }
}

use crate::{config::Config, octree::OctreeTextures};

use super::ConeParameters;

pub struct DebugCone {
    pub transform: Transform,
    pub parameters: ConeParameters,
    pub point_to_light: bool,
    shader: Shader,
    direction: Vector3<f32>,
    previous_values: HashSet<u32>,
    nodes_queried: BufferTexture,
    nodes_queried_counter: GLuint,
    sampled_colors_texture: BufferTexture,
    vao: GLuint,
}

pub struct VoxelData {
    color: (f32, f32, f32, f32),
    octree_level: f32,
}
impl fmt::Display for VoxelData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = customcolors::CustomColor {
            r: (self.color.0 * 255.0) as u8,
            g: (self.color.1 * 255.0) as u8,
            b: (self.color.2 * 255.0) as u8,
        };
        write!(
            f,
            "{} {}, octree_level: {}",
            "color:".custom_color(color),
            format!(
                "({}, {}, {}, {})",
                self.color.0, self.color.1, self.color.2, self.color.3
            ),
            self.octree_level,
        )
    }
}

impl DebugCone {
    pub unsafe fn new() -> Self {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);

        let mut transform = Transform::default();
        transform.movement_speed = 0.25;
        transform.position = point3(0.5, 0.5, 0.43);

        Self {
            shader: compile_shaders!("assets/shaders/debug/debugConeTracing.glsl", debug = true),
            transform,
            parameters: ConeParameters {
                max_distance: 1.0,
                cone_angle_in_degrees: 30f32.to_radians(),
            },
            direction: vec3(0.0, 1.0, 0.0),
            previous_values: HashSet::new(),
            nodes_queried: helpers::generate_texture_buffer_with_hint(
                1000,
                gl::R32UI,
                69u32,
                gl::DYNAMIC_READ,
            ),
            sampled_colors_texture: helpers::generate_texture_buffer_with_hint(
                100,
                gl::R32F,
                69f32,
                gl::DYNAMIC_READ,
            ),
            nodes_queried_counter: helpers::generate_atomic_counter_buffer1(),
            point_to_light: false,
            vao,
        }
    }

    pub unsafe fn run(
        &mut self,
        textures: &OctreeTextures,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        selected_debug_nodes: &mut Vec<DebugNode>,
        geometry_buffers: &Textures<GEOMETRY_BUFFERS>,
        geometry_buffer_coordinates: &Vector2<f32>,
        light: &Light,
    ) {
        helpers::clear_texture_buffer(self.sampled_colors_texture.1, 100, 42f32, gl::DYNAMIC_READ);
        self.shader.use_program();

        gl::BindVertexArray(self.vao);

        helpers::bind_image_texture(0, self.nodes_queried.0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(1, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(2, self.sampled_colors_texture.0, gl::WRITE_ONLY, gl::R32F);

        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, self.nodes_queried_counter);

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

        // Unbind any texture leftover
        gl::BindTexture(gl::TEXTURE_2D, 0);

        self.shader.set_vec2(
            c_str!("gBufferQueryCoordinates"),
            geometry_buffer_coordinates.x,
            geometry_buffer_coordinates.y,
        );

        let config = Config::instance();

        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.shader
            .set_uint(c_str!("maxOctreeLevel"), config.octree_levels() - 1);
        self.shader.set_mat4(c_str!("projection"), projection);
        self.shader.set_mat4(c_str!("view"), view);
        self.shader.set_vec3(
            c_str!("position"),
            self.transform.position.x,
            self.transform.position.y,
            self.transform.position.z,
        );
        self.shader
            .set_bool(c_str!("pointToLight"), self.point_to_light);
        self.shader.set_vec3(
            c_str!("lightPosition"),
            light.transform().position.x,
            light.transform().position.y,
            light.transform().position.z,
        );
        self.shader.set_vec3(
            c_str!("axis"),
            self.direction.x,
            self.direction.y,
            self.direction.z,
        );
        self.parameters.set_uniforms("parameters", &self.shader);

        let number_of_cones = 1; // For now
        gl::DrawArrays(gl::POINTS, 0, number_of_cones);

        let values = helpers::get_values_from_texture_buffer(self.nodes_queried.1, 1000, 42u32);
        let sampled_colors =
            helpers::get_values_from_texture_buffer(self.sampled_colors_texture.1, 100, 32f32);
        // dbg!(&sampled_colors[0..5]);
        // pretty_print_data(&sampled_colors[5..]);

        let total_nodes_queried =
            helpers::get_value_from_atomic_counter(self.nodes_queried_counter) as usize;
        let values_set = HashSet::from_iter(values[..total_nodes_queried].iter().cloned());

        if self.previous_values != values_set {
            // dbg!(&values[..total_nodes_queried]);
            let set_vector: Vec<_> = values_set.iter().cloned().collect();
            *selected_debug_nodes = (&set_vector[..])
                .iter()
                .map(|&index| DebugNode::new(index, "picked by cone".to_string()))
                .collect();
            self.previous_values = values_set;
        }

        gl::BindVertexArray(0);
    }
}

const AMOUNT_OF_VALUES: usize = 5;

fn pretty_print_data(sampled_colors: &[f32]) {
    for chunk in sampled_colors.chunks_exact(AMOUNT_OF_VALUES) {
        let voxel_data = VoxelData {
            color: (chunk[0], chunk[1], chunk[2], chunk[3]),
            octree_level: chunk[4],
        };
        if voxel_data.octree_level != 42.0 {
            println!("{}", voxel_data);
        }
    }
}
