use std::{ffi::c_void, fmt, mem::size_of};

use c_str_macro::c_str;
use cgmath::Matrix4;

use crate::{config::CONFIG, helpers};

use super::Octree;

#[derive(Debug, Clone, Copy)]
pub enum ShowBricks {
    DontShow,
    ShowZ0,
    ShowZ1,
    ShowZ2,
}

impl ShowBricks {
    pub fn next(self) -> Self {
        match self {
            Self::DontShow => Self::ShowZ0,
            Self::ShowZ0 => Self::ShowZ1,
            Self::ShowZ1 => Self::ShowZ2,
            Self::ShowZ2 => Self::DontShow,
        }
    }
}

impl Into<u32> for ShowBricks {
    fn into(self) -> u32 {
        match self {
            Self::DontShow => 0,
            Self::ShowZ0 => 1,
            Self::ShowZ1 => 2,
            Self::ShowZ2 => 3,
        }
    }
}

impl fmt::Display for ShowBricks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DontShow => write!(f, "Not showing"),
            Self::ShowZ0 => write!(f, "Showing z=0"),
            Self::ShowZ1 => write!(f, "Showing z=1"),
            Self::ShowZ2 => write!(f, "Showing z=2"),
        }
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
            .set_uint(c_str!("showBricks"), self.renderer.show_bricks.into());

        self.renderer
            .shader
            .set_mat4(c_str!("projection"), projection);
        self.renderer.shader.set_mat4(c_str!("view"), view);
        self.renderer.shader.set_mat4(c_str!("model"), model);

        self.renderer
            .shader
            .set_float(c_str!("halfDimension"), 1.0 / CONFIG.voxel_dimension as f32);

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::DrawArrays(
            gl::POINTS,
            0,
            self.voxel_data.number_of_voxel_fragments as i32,
        );
    }

    pub fn toggle_show_bricks(&mut self) {
        self.renderer.show_bricks = self.renderer.show_bricks.next();
    }

    pub unsafe fn run_node_positions_shader(
        &self,
        node_indices: &Vec<u32>,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        if node_indices.is_empty() {
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
}
