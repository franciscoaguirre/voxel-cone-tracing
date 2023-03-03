use std::fmt;

use c_str_macro::c_str;
use cgmath::Matrix4;
use gl::types::*;

use super::common::OCTREE_NODE_POOL;
use crate::config::CONFIG;
use crate::rendering::shader::Shader;
use crate::voxelization::helpers;
use crate::voxelization::octree::common::{
    BRICK_POOL_COLORS_TEXTURE, OCTREE_NODE_POOL_BRICK_POINTERS,
};

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

pub struct RenderOctreeShader {
    shader: Shader,
    voxel_positions_texture: GLuint,
    number_of_voxel_fragments: u32,
}

impl RenderOctreeShader {
    pub fn init(voxel_positions_texture: GLuint, number_of_voxel_fragments: u32) -> Self {
        Self {
            shader: Shader::with_geometry_shader(
                "assets/shaders/octree/visualize.vert.glsl",
                "assets/shaders/octree/visualize.frag.glsl",
                "assets/shaders/octree/visualize.geom.glsl",
            ),
            voxel_positions_texture,
            number_of_voxel_fragments,
        }
    }

    pub unsafe fn run(
        &self,
        model: &Matrix4<f32>,
        view: &Matrix4<f32>,
        projection: &Matrix4<f32>,
        octree_level: u32,
        show_bricks: &ShowBricks,
    ) {
        self.shader.use_program();

        helpers::bind_image_texture(0, OCTREE_NODE_POOL.0, gl::READ_WRITE, gl::R32UI);
        helpers::bind_image_texture(
            1,
            OCTREE_NODE_POOL_BRICK_POINTERS.0,
            gl::READ_WRITE,
            gl::R32UI,
        );
        helpers::bind_3d_image_texture(2, BRICK_POOL_COLORS_TEXTURE, gl::READ_ONLY, gl::RGBA8);
        helpers::bind_image_texture(
            3,
            self.voxel_positions_texture,
            gl::READ_ONLY,
            gl::RGB10_A2UI,
        );

        self.shader.set_uint(c_str!("octreeLevels"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.shader
            .set_uint(c_str!("showBricks"), (*show_bricks).into());

        self.shader.set_mat4(c_str!("projection"), projection);
        self.shader.set_mat4(c_str!("view"), view);
        self.shader.set_mat4(c_str!("model"), model);

        self.shader
            .set_float(c_str!("halfDimension"), 1.0 / CONFIG.voxel_dimension as f32);

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::DrawArrays(gl::POINTS, 0, self.number_of_voxel_fragments as i32);
    }
}
