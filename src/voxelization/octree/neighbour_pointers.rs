use c_str_macro::c_str;
use gl::types::*;

use crate::{
    constants::{OCTREE_LEVELS, VOXEL_DIMENSION},
    rendering::shader::Shader,
    voxelization::helpers::bind_image_texture,
};

pub struct NeighbourPointersPass {
    shader: Shader,
    node_pool_texture: GLuint,
    voxel_positions_texture: GLuint,
    node_pool_neighbours_x_texture: GLuint,
    node_pool_neighbours_x_negative_texture: GLuint,
    node_pool_neighbours_y_texture: GLuint,
    node_pool_neighbours_y_negative_texture: GLuint,
    node_pool_neighbours_z_texture: GLuint,
    node_pool_neighbours_z_negative_texture: GLuint,
}

impl NeighbourPointersPass {
    pub fn init(
        voxel_positions_texture: GLuint,
        node_pool_texture: GLuint,
        node_pool_neighbours_x_texture: GLuint,
        node_pool_neighbours_x_negative_texture: GLuint,
        node_pool_neighbours_y_texture: GLuint,
        node_pool_neighbours_y_negative_texture: GLuint,
        node_pool_neighbours_z_texture: GLuint,
        node_pool_neighbours_z_negative_texture: GLuint,
    ) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/neighbour_pointers.comp.glsl"),
            node_pool_texture,
            voxel_positions_texture,
            node_pool_neighbours_x_texture,
            node_pool_neighbours_x_negative_texture,
            node_pool_neighbours_y_texture,
            node_pool_neighbours_y_negative_texture,
            node_pool_neighbours_z_texture,
            node_pool_neighbours_z_negative_texture,
        }
    }

    pub unsafe fn run(&self, current_octree_level: u32) {
        self.shader.use_program();

        // Set uniforms
        self.shader
            .set_uint(c_str!("voxel_dimension"), VOXEL_DIMENSION as u32);
        self.shader
            .set_uint(c_str!("current_octree_level"), current_octree_level);

        // Bind images
        bind_image_texture(0, self.node_pool_texture, gl::READ_WRITE, gl::R32UI);
        bind_image_texture(1, self.voxel_positions_texture, gl::READ_WRITE, gl::R32UI);

        bind_image_texture(
            2,
            self.node_pool_neighbours_x_texture,
            gl::READ_WRITE,
            gl::R32UI,
        );
        bind_image_texture(
            3,
            self.node_pool_neighbours_x_negative_texture,
            gl::READ_WRITE,
            gl::R32UI,
        );
        bind_image_texture(
            4,
            self.node_pool_neighbours_y_texture,
            gl::READ_WRITE,
            gl::R32UI,
        );
        bind_image_texture(
            5,
            self.node_pool_neighbours_y_negative_texture,
            gl::READ_WRITE,
            gl::R32UI,
        );
        bind_image_texture(
            6,
            self.node_pool_neighbours_z_texture,
            gl::READ_WRITE,
            gl::R32UI,
        );
        bind_image_texture(
            7,
            self.node_pool_neighbours_z_negative_texture,
            gl::READ_WRITE,
            gl::R32UI,
        );

        self.shader.dispatch(65_535); // TODO: Calculate number of groups
        self.shader.wait();
    }
}
