use c_str_macro::c_str;

use crate::{
    config::CONFIG,
    constants::{CHILDREN_PER_NODE, WORKING_GROUP_SIZE},
    helpers,
    octree::{OctreeTextures, VoxelData},
    rendering::shader::Shader,
};

pub struct WriteLeafNodesPass {
    shader: Shader,
}

impl WriteLeafNodesPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/writeLeafNodes.comp.glsl"),
        }
    }

    pub unsafe fn run(
        &self,
        voxel_data: &VoxelData,
        textures: &OctreeTextures,
        nodes_per_level: &[u32],
    ) {
        self.shader.use_program();
        let octree_level = CONFIG.octree_levels - 1;

        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.shader.set_uint(c_str!("octreeLevel"), octree_level); // Last level
        self.shader.set_uint(
            c_str!("number_of_voxel_fragments"),
            voxel_data.number_of_voxel_fragments,
        );

        helpers::bind_image_texture(
            0,
            voxel_data.voxel_positions,
            gl::READ_WRITE,
            gl::RGB10_A2UI,
        );
        helpers::bind_image_texture(1, voxel_data.voxel_colors, gl::READ_WRITE, gl::RGBA8);
        helpers::bind_image_texture(2, textures.brick_pointers.0, gl::READ_WRITE, gl::R32UI);
        helpers::bind_3d_image_texture(3, textures.brick_pool_colors, gl::WRITE_ONLY, gl::RGBA8);
        helpers::bind_image_texture(4, textures.node_pool.0, gl::READ_WRITE, gl::R32UI);

        let tiles_in_level = nodes_per_level[octree_level as usize];
        let nodes_in_level = tiles_in_level * CHILDREN_PER_NODE;
        let groups_count = (nodes_in_level as f32 / WORKING_GROUP_SIZE as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
