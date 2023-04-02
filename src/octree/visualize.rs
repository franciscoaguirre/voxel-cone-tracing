use std::{ffi::c_void, mem::size_of};

use c_str_macro::c_str;
use cgmath::Matrix4;

use crate::{config::CONFIG, helpers};

use super::Octree;

#[derive(Debug, Clone, Copy, Default)]
pub struct BricksToShow {
    show_z0: bool,
    show_z1: bool,
    show_z2: bool,
}

impl BricksToShow {
    pub fn toggle_z0(&mut self) {
        self.show_z0 = !self.show_z0;
    }

    pub fn toggle_z1(&mut self) {
        self.show_z1 = !self.show_z1;
    }

    pub fn toggle_z2(&mut self) {
        self.show_z2 = !self.show_z2;
    }

    pub fn at_least_one(&self) -> bool {
        self.show_z0 || self.show_z1 || self.show_z2
    }

    pub fn z0(&self) -> bool {
        self.show_z0
    }

    pub fn z1(&self) -> bool {
        self.show_z1
    }

    pub fn z2(&self) -> bool {
        self.show_z2
    }
}

impl Into<u32> for BricksToShow {
    fn into(self) -> u32 {
        self.show_z0 as u32 | (self.show_z1 as u32) << 1 | (self.show_z2 as u32) << 2 as u32
    }
}

impl Octree {
    pub unsafe fn render(
        &self,
        model: &Matrix4<f32>,
        view: &Matrix4<f32>,
        projection: &Matrix4<f32>,
        octree_level: u32,
    ) {
        self.renderer.shader.use_program();

        helpers::bind_image_texture(0, self.textures.node_pool.0, gl::READ_WRITE, gl::R32UI);
        helpers::bind_image_texture(1, self.textures.brick_pointers.0, gl::READ_WRITE, gl::R32UI);
        helpers::bind_3d_image_texture(
            2,
            self.textures.brick_pool_colors,
            gl::READ_ONLY,
            gl::RGBA8,
        );
        helpers::bind_image_texture(
            3,
            self.voxel_data.voxel_positions,
            gl::READ_ONLY,
            gl::RGB10_A2UI,
        );

        self.renderer
            .shader
            .set_uint(c_str!("octreeLevels"), octree_level);
        self.renderer
            .shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        self.renderer
            .shader
            .set_mat4(c_str!("projection"), projection);
        self.renderer.shader.set_mat4(c_str!("view"), view);
        self.renderer.shader.set_mat4(c_str!("model"), model);

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::DrawArrays(
            gl::POINTS,
            0,
            self.voxel_data.number_of_voxel_fragments as i32,
        );
    }

    pub fn set_bricks_to_show(&mut self, bricks_to_show: BricksToShow) {
        self.renderer.bricks_to_show = bricks_to_show;
    }

    pub unsafe fn set_node_indices(&mut self, node_indices: &Vec<u32>) {
        if node_indices.is_empty() {
            return;
        }

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

        self.renderer.node_count = node_indices.len() as u32;
        self.renderer.vao = vao;
    }

    pub unsafe fn run_node_positions_shader(
        &self,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        if self.renderer.node_count == 0 {
            return;
        }

        self.renderer.node_positions_shader.use_program();

        self.renderer
            .node_positions_shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.renderer
            .node_positions_shader
            .set_uint(c_str!("maxOctreeLevel"), CONFIG.octree_levels);

        self.renderer
            .node_positions_shader
            .set_mat4(c_str!("projection"), &projection);
        self.renderer
            .node_positions_shader
            .set_mat4(c_str!("view"), &view);
        self.renderer
            .node_positions_shader
            .set_mat4(c_str!("model"), &model);

        helpers::bind_image_texture(
            0,
            self.textures.node_positions.0,
            gl::READ_ONLY,
            gl::RGB10_A2UI,
        );
        helpers::bind_image_texture(
            1,
            self.textures.level_start_indices.0,
            gl::READ_ONLY,
            gl::R32UI,
        );

        gl::BindVertexArray(self.renderer.vao);
        gl::DrawArrays(gl::POINTS, 0, self.renderer.node_count as i32);
    }

    pub unsafe fn run_node_neighbors_shader(
        &self,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        if self.renderer.node_count == 0 {
            return;
        }

        self.renderer.node_neighbors_shader.use_program();

        self.renderer
            .node_neighbors_shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.renderer
            .node_neighbors_shader
            .set_uint(c_str!("maxOctreeLevel"), CONFIG.octree_levels);

        self.renderer
            .node_neighbors_shader
            .set_mat4(c_str!("projection"), &projection);
        self.renderer
            .node_neighbors_shader
            .set_mat4(c_str!("view"), &view);
        self.renderer
            .node_neighbors_shader
            .set_mat4(c_str!("model"), &model);

        helpers::bind_image_texture(
            0,
            self.textures.node_positions.0,
            gl::READ_ONLY,
            gl::RGB10_A2UI,
        );
        helpers::bind_image_texture(1, self.textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(
            2,
            self.textures.level_start_indices.0,
            gl::READ_ONLY,
            gl::R32UI,
        );

        for texture_offset in 0..self.textures.neighbors.len() {
            helpers::bind_image_texture(
                3 + texture_offset as u32,
                self.textures.neighbors[texture_offset as usize].0,
                gl::READ_ONLY,
                gl::R32UI,
            );
        }

        gl::BindVertexArray(self.renderer.vao);
        gl::DrawArrays(gl::POINTS, 0, self.renderer.node_count as i32);
    }

    pub unsafe fn run_node_bricks_shader(
        &self,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        if self.renderer.node_count == 0 {
            return;
        }

        self.renderer.node_bricks_shader.use_program();

        self.renderer
            .node_bricks_shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.renderer
            .node_bricks_shader
            .set_uint(c_str!("maxOctreeLevel"), CONFIG.octree_levels);
        self.renderer
            .node_bricks_shader
            .set_uint(c_str!("bricksToShow"), self.renderer.bricks_to_show.into());

        self.renderer
            .node_bricks_shader
            .set_mat4(c_str!("projection"), &projection);
        self.renderer
            .node_bricks_shader
            .set_mat4(c_str!("view"), &view);
        self.renderer
            .node_bricks_shader
            .set_mat4(c_str!("model"), &model);

        helpers::bind_image_texture(
            0,
            self.textures.node_positions.0,
            gl::READ_ONLY,
            gl::RGB10_A2UI,
        );
        helpers::bind_image_texture(
            1,
            self.textures.level_start_indices.0,
            gl::READ_ONLY,
            gl::R32UI,
        );
        helpers::bind_image_texture(2, self.textures.brick_pointers.0, gl::READ_WRITE, gl::R32UI);
        helpers::bind_3d_image_texture(
            3,
            self.textures.brick_pool_colors,
            gl::READ_ONLY,
            gl::RGBA8,
        );

        gl::BindVertexArray(self.renderer.vao);
        gl::DrawArrays(gl::POINTS, 0, self.renderer.node_count as i32);
    }
}
