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
            nodes_queried: helpers::generate_texture_buffer(1000, gl::R32UI, 69u32),
            cone_angle: 0.263599,
            vao,
        }
    }

    // self.transform.position.x = 0.5;
    // self.transform.position.y = 0.5;
    // self.transform.position.z = 0.43;

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

        let nodes_queried_counter = helpers::generate_atomic_counter_buffer();
        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, nodes_queried_counter);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_3D, textures.brick_pool_colors[0]);
        self.shader.set_int(c_str!("brickPoolColors"), 0 as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_3D, textures.brick_pool_photons);
        self.shader.set_int(c_str!("brickPoolPhotons"), 1 as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

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
        let values_set = HashSet::from_iter(values.iter().cloned());
        let total_nodes_queried =
            helpers::get_value_from_atomic_counter(nodes_queried_counter) as usize;

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
