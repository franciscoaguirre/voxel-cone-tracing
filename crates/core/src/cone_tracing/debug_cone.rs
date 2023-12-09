use std::collections::HashSet;
use std::fmt;
use engine::prelude::*;

use c_str_macro::c_str;
use cgmath::{point3, vec3, Matrix4, Vector3, Vector2};
use colored::{customcolors, Colorize};

use gl::types::GLuint;

use crate::{
    config::Config,
    menu::DebugNode,
    octree::OctreeTextures,
};

use super::ConeParameters;

pub struct DebugCone {
    pub transform: Transform,
    pub parameters: ConeParameters,
    pub point_to_light: bool,
    shader: Shader,
    direction: Vector3<f32>,
    previous_values: HashSet<u32>,
    nodes_queried: BufferTextureV2<u32>,
    nodes_queried_counter: AtomicCounter,
    sampled_colors_texture: BufferTextureV2<f32>,
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
            nodes_queried: BufferTextureV2::from_data_and_hint(&vec![0u32; 1000], UsageHint::DynamicRead),
            sampled_colors_texture: BufferTextureV2::from_data_and_hint(&vec![0f32; 100], UsageHint::DynamicRead),
            nodes_queried_counter: AtomicCounter::new(),
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
        self.sampled_colors_texture.fill_with(&vec![0f32; 100], true);
        self.shader.use_program();

        gl::BindVertexArray(self.vao);

        self.shader.bind_image_texture(0, self.nodes_queried, TextureAccess::WriteOnly);
        self.shader.bind_image_texture(1, textures.node_pool, TextureAccess::ReadOnly);
        self.shader.bind_image_texture(2, self.sampled_colors_texture, TextureAccess::WriteOnly);

        self.nodes_queried_counter.bind(0);

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
            self.shader.bind_texture(texture_name, texture, true);
        }

        // TODO: Remove
        // Unbind any texture leftover
        gl::BindTexture(gl::TEXTURE_2D, 0);

        self.shader
            .set_vec2(
                c_str!("gBufferQueryCoordinates"),
                geometry_buffer_coordinates.x,
                geometry_buffer_coordinates.y
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
        self.shader.set_bool(
            c_str!("pointToLight"),
            self.point_to_light
        );
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

        let values = self.nodes_queried.data();
        let sampled_colors = self.sampled_colors_texture.data();
        // dbg!(&sampled_colors[0..5]);
        // pretty_print_data(&sampled_colors[5..]);

        let total_nodes_queried = self.nodes_queried_counter.value() as usize;
        self.nodes_queried_counter.reset();
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
