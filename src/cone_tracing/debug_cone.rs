use std::collections::HashSet;

use c_str_macro::c_str;
use cgmath::{vec3, Matrix4, Vector3};
use gl::types::GLuint;

use crate::{
    config::CONFIG,
    helpers,
    menu::DebugNode,
    octree::OctreeTextures,
    rendering::{
        shader::{compile_shaders, Shader},
        transform::Transform,
    },
    types::BufferTexture,
};

pub struct DebugCone {
    pub transform: Transform,
    pub cone_angle: f32,
    shader: Shader,
    direction: Vector3<f32>,
    previous_values: HashSet<u32>,
    nodes_queried: BufferTexture,
    nodes_queried_counter: GLuint,
    sampled_color_texture: BufferTexture,
    vao: GLuint,
}

impl DebugCone {
    pub unsafe fn new() -> Self {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);

        let mut transform = Transform::default();
        transform.movement_speed = 0.25;

        Self {
            shader: compile_shaders!("assets/shaders/debug/debugConeTracing.glsl", debug = true),
            transform,
            direction: vec3(0.0, 0.0, 1.0),
            previous_values: HashSet::new(),
            nodes_queried: helpers::generate_texture_buffer4(
                1000,
                gl::R32UI,
                69u32,
                gl::DYNAMIC_READ,
            ),
            sampled_color_texture: helpers::generate_texture_buffer4(
                5,
                gl::R32F,
                69f32,
                gl::DYNAMIC_READ,
            ),
            nodes_queried_counter: helpers::generate_atomic_counter_buffer1(gl::DYNAMIC_READ),
            cone_angle: 0.263599,
            vao,
        }
    }

    pub unsafe fn run(
        &mut self,
        textures: &OctreeTextures,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        selected_debug_nodes: &mut Vec<DebugNode>,
    ) {
        self.shader.use_program();

        gl::BindVertexArray(self.vao);

        helpers::bind_image_texture(0, self.nodes_queried.0, gl::WRITE_ONLY, gl::R32UI);
        helpers::bind_image_texture(1, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(2, self.sampled_color_texture.0, gl::WRITE_ONLY, gl::R32F);

        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, self.nodes_queried_counter);

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

        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.shader
            .set_uint(c_str!("maxOctreeLevel"), CONFIG.octree_levels - 1);
        self.shader.set_mat4(c_str!("projection"), projection);
        self.shader.set_mat4(c_str!("view"), view);
        self.shader.set_vec3(
            c_str!("position"),
            self.transform.position.x,
            self.transform.position.y,
            self.transform.position.z,
        );
        self.shader.set_vec3(
            c_str!("axis"),
            self.direction.x,
            self.direction.y,
            self.direction.z,
        );
        self.shader
            .set_float(c_str!("coneAngle"), self.cone_angle as f32);

        // Add more points if we want more debug cones
        gl::DrawArrays(gl::POINTS, 0, 1);

        let values = helpers::get_values_from_texture_buffer(self.nodes_queried.1, 1000, 42u32);
        let sampled_color =
            helpers::get_values_from_texture_buffer(self.sampled_color_texture.1, 5, 32f32);
        dbg!(sampled_color);

        let total_nodes_queried =
            helpers::get_value_from_atomic_counter(self.nodes_queried_counter) as usize;
        let values_set = HashSet::from_iter(values[..total_nodes_queried].iter().cloned());

        if self.previous_values != values_set {
            dbg!(&values[..total_nodes_queried]);
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
