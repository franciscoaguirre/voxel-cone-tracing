use c_str_macro::c_str;

use crate::{
    config::CONFIG,
    constants::Sign,
    constants::{Axis, Direction},
    helpers,
    octree::{build::BrickPoolValues, NodeData, OctreeTextures},
    rendering::shader::Shader,
};

pub struct MipmapEdgesPass {
    shader: Shader,
}

impl MipmapEdgesPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/mipmapEdges.comp.glsl"),
        }
    }

    pub unsafe fn run(
        &self,
        textures: &OctreeTextures,
        node_data: &NodeData,
        level: u32,
        direction: Direction,
    ) {
        let mut neighbors_texture_number = match direction.axis {
            Axis::X => 0,
            Axis::Y => 2,
            Axis::Z => 4,
        };
        // Sign 1 means positive sign, and also means we need the positive sign neighbors texture
        neighbors_texture_number = match direction.sign {
            Sign::Pos => neighbors_texture_number,
            Sign::Neg => neighbors_texture_number + 1,
        };

        self.shader.use_program();

        self.shader.set_uint(c_str!("octreeLevel"), level);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.shader
            .set_int(c_str!("direction.axis"), direction.axis.into());
        self.shader
            .set_int(c_str!("direction.sign"), direction.sign.into());

        helpers::bind_image_texture(0, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        // Set directional mipmap children's color texture
        helpers::bind_3d_image_texture(
            1,
            textures.brick_pool_colors[0],
            gl::WRITE_ONLY,
            gl::RGBA8,
        );
        helpers::bind_image_texture(
            3,
            textures.neighbors[0].0,
            gl::READ_ONLY,
            gl::R32UI,
        );
        helpers::bind_image_texture(2, node_data.level_start_indices.0, gl::READ_ONLY, gl::R32UI);

        let last_level = CONFIG.octree_levels - 1;
        let read_texture_index = if level == last_level - 1 {
            0
        } else {
            0
        };
        helpers::bind_3d_image_texture(
            4,
            textures.brick_pool_colors[read_texture_index],
            gl::READ_ONLY,
            gl::RGBA8,
        );

        let nodes_in_level = node_data.nodes_per_level[level as usize];
        let groups_count = (nodes_in_level as f32 / CONFIG.working_group_size as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
