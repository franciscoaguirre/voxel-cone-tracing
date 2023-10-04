use c_str_macro::c_str;
use engine::prelude::*;

use crate::{
    config::Config,
    octree::{OctreeTextures, VoxelData},
};

pub struct FlagNodesPass {
    shader: Shader,
}

const SHADER_PATH: &'static str = "assets/shaders/octree/flagNodes.comp.glsl";

impl FlagNodesPass {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!(SHADER_PATH),
        }
    }

    pub unsafe fn run(&self, voxel_data: &VoxelData, textures: &OctreeTextures, octree_level: u32) {
        self.run_minimal(voxel_data, textures.node_pool, octree_level);
    }

    unsafe fn run_minimal(&self, voxel_data: &VoxelData, node_pool: BufferTexture, octree_level: u32) {
        self.shader.use_program();

        let config = Config::instance();
        self.shader.set_uint(
            c_str!("numberOfVoxelFragments"),
            voxel_data.number_of_voxel_fragments,
        );
        self.shader.set_uint(c_str!("octreeLevel"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

        helpers::bind_image_texture(0, voxel_data.voxel_positions.0, gl::READ_ONLY, gl::RGB10_A2);
        helpers::bind_image_texture(1, node_pool.0, gl::READ_WRITE, gl::R32UI);

        let groups_count = (voxel_data.number_of_voxel_fragments as f32
            / config.working_group_size as f32)
            .ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}

#[cfg(test)]
mod tests {
    use engine::prelude::*;
    use super::*;
    use std::path::PathBuf;
    use std::env;
    use std::{
        mem::size_of,
    };
    use gl::types::GLuint;

    #[test]
    fn flag_nodes_works() {

        let (_glfw, _window) = test_utils::init_opengl_context();

        // To go from the crate root to the workspace root
        let mut path = PathBuf::from(env::current_dir().unwrap());
        path.pop();
        path.pop();
        env::set_current_dir(path).unwrap();

        let shader = FlagNodesPass::init();

        let number_of_voxel_fragments = 2;
        unsafe {
            let voxel_data = VoxelData {
                voxel_positions: helpers::generate_texture_buffer(
                                     size_of::<GLuint>() * number_of_voxel_fragments as usize,
                                     gl::R32UI,
                                     0u32,
                                 ),
                number_of_voxel_fragments: number_of_voxel_fragments,
                voxel_colors: (0, 0),
                voxel_normals: (0, 0),
            };
            let node_pool = helpers::generate_texture_buffer(10000, gl::R32UI, 0u32);

            shader.run_minimal(&voxel_data, node_pool, 0); // Fails on this line 
            let values = helpers::get_values_from_texture_buffer(node_pool.1, 1, 0_u32);
            assert_eq!(values, [2]);
        }
    }
}
