use std::ffi::c_void;
use std::mem::size_of;

use c_str_macro::c_str;
use cgmath::{Matrix4, Point3};

use crate::config::CONFIG;
use crate::rendering::shader::Shader;
use crate::voxelization::octree::common::{
    OCTREE_LEVEL_START_INDICES, OCTREE_NODE_POOL, OCTREE_NODE_POSITIONS, OCTREE_NODE_POOL_NEIGHBOUR_X
};
use crate::voxelization::voxelize::VOXEL_POSITIONS;
use crate::voxelization::{helpers, octree};

pub struct VisualDebugger {
    voxel_fragments_shader: Shader,
    node_positions_shader: Shader,
    points_shader: Shader,
}

impl VisualDebugger {
    pub fn init() -> Self {
        Self {
            voxel_fragments_shader: Shader::with_geometry_shader(
                "assets/shaders/debug/voxel.vert.glsl",
                "assets/shaders/debug/voxel.frag.glsl",
                "assets/shaders/debug/voxel.geom.glsl",
            ),
            node_positions_shader: Shader::with_geometry_shader(
                "assets/shaders/debug/nodePositions.vert.glsl",
                "assets/shaders/debug/nodePositions.frag.glsl",
                "assets/shaders/debug/nodePositions.geom.glsl",
            ),
            points_shader: Shader::new(
                "assets/shaders/debug/points.vert.glsl",
                "assets/shaders/debug/points.frag.glsl",
            ),
        }
    }

    pub unsafe fn run(
        &self,
        voxel_indices: &Vec<u32>,
        node_indices: &Vec<u32>,
        points: &Vec<Point3<f32>>,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        self.run_voxel_fragments_shader(voxel_indices, projection, view, model);
        self.run_node_positions_shader(node_indices, projection, view, model);
        self.run_points_shader(points, projection, view, model);
    }

    unsafe fn run_node_positions_shader(
        &self,
        node_indices: &Vec<u32>,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        if node_indices.is_empty() {
            return;
        }

        self.node_positions_shader.use_program();

        self.node_positions_shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.node_positions_shader
            .set_uint(c_str!("maxOctreeLevel"), CONFIG.octree_levels);

        self.node_positions_shader
            .set_mat4(c_str!("projection"), &projection);
        self.node_positions_shader.set_mat4(c_str!("view"), &view);
        self.node_positions_shader.set_mat4(c_str!("model"), &model);

        helpers::bind_image_texture(0, OCTREE_NODE_POSITIONS.0, gl::READ_ONLY, gl::RGB10_A2UI);
        helpers::bind_image_texture(1, OCTREE_NODE_POOL.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(2, OCTREE_LEVEL_START_INDICES.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(3, OCTREE_NODE_POOL_NEIGHBOUR_X.0, gl::READ_ONLY, gl::R32UI);

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (node_indices.len() * size_of::<u32>()) as isize,
            &node_indices[0] as *const u32 as *const c_void,
            gl::DYNAMIC_DRAW,
        );
        gl::VertexAttribIPointer(
            0,
            1,
            gl::UNSIGNED_INT,
            size_of::<u32>() as i32,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(0);

        gl::DrawArrays(gl::POINTS, 0, node_indices.len() as i32);
    }

    unsafe fn run_points_shader(
        &self,
        points: &Vec<Point3<f32>>,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        gl::PointSize(10.);

        self.points_shader.use_program();

        self.points_shader
            .set_uint(c_str!("numberOfPoints"), points.len() as u32);
        self.points_shader
            .set_point_vector(c_str!("points"), points.len(), points);

        self.points_shader
            .set_mat4(c_str!("projection"), &projection);
        self.points_shader.set_mat4(c_str!("view"), &view);
        self.points_shader.set_mat4(c_str!("model"), &model);

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::DrawArrays(gl::POINTS, 0, 1);

        gl::PointSize(1.);
    }

    unsafe fn run_voxel_fragments_shader(
        &self,
        voxel_indices: &Vec<u32>,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        if voxel_indices.is_empty() {
            return;
        }

        self.voxel_fragments_shader.use_program();

        self.voxel_fragments_shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.voxel_fragments_shader
            .set_uint(c_str!("octreeLevels"), CONFIG.octree_levels - 1);

        self.voxel_fragments_shader
            .set_mat4(c_str!("projection"), &projection);
        self.voxel_fragments_shader.set_mat4(c_str!("view"), &view);
        self.voxel_fragments_shader
            .set_mat4(c_str!("model"), &model);

        helpers::bind_image_texture(0, VOXEL_POSITIONS.0, gl::READ_ONLY, gl::RGB10_A2UI);
        helpers::bind_image_texture(1, OCTREE_NODE_POOL.0, gl::READ_ONLY, gl::R32UI);

        // let (debug_texture, debug_texture_buffer) =
        //     helpers::generate_texture_buffer(3, gl::R32F, 69_f32);
        // helpers::bind_image_texture(2, debug_texture, gl::WRITE_ONLY, gl::R32F);

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (voxel_indices.len() * size_of::<u32>()) as isize,
            &voxel_indices[0] as *const u32 as *const c_void,
            gl::DYNAMIC_DRAW,
        );
        gl::VertexAttribIPointer(
            0,
            1,
            gl::UNSIGNED_INT,
            size_of::<u32>() as i32,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(0);

        gl::DrawArrays(gl::POINTS, 0, voxel_indices.len() as i32);
    }
}

pub fn r32ui_to_rgb10_a2ui(from: u32) -> (u32, u32, u32) {
    let mask = 0b00000000000000000000001111111111;

    (from & mask, (from >> 10) & mask, (from >> 20) & mask)
}
