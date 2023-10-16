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
}

pub struct FlagNodesInput<'a> {
    pub octree_level: u32,
    pub voxel_data: &'a VoxelData,
    pub node_pool: BufferTexture,
}

impl ShaderPass for FlagNodesPass {
    type Input<'a> = FlagNodesInput<'a>;

    unsafe fn run<'a>(&self, input: Self::Input<'a>) {
        self.shader.use_program();

        let config = Config::instance();

        self.shader.set_uint(
            c_str!("numberOfVoxelFragments"),
            input.voxel_data.number_of_voxel_fragments,
        );
        self.shader.set_uint(c_str!("octreeLevel"), input.octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

        helpers::bind_image_texture(0, input.voxel_data.voxel_positions.0, gl::READ_ONLY, gl::RGB10_A2);
        helpers::bind_image_texture(1, input.node_pool.0, gl::READ_WRITE, gl::R32UI);

        let groups_count = (input.voxel_data.number_of_voxel_fragments as f32
            / config.working_group_size as f32)
            .ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::env;

    #[test]
    fn flag_nodes_works() {
        let (_glfw, _window) = test_utils::init_opengl_context();

        // To go from the crate root to the workspace root
        let mut path = PathBuf::from(env::current_dir().unwrap());
        path.pop();
        path.pop();
        env::set_current_dir(path).unwrap();

        unsafe {
            Config::initialize(Config::new(1));
            let flag_nodes_pass = FlagNodesPass::init();
            let voxel_data = VoxelData {
                voxel_positions: helpers::generate_texture_buffer_with_initial_data(
                    1,
                    gl::RGB10_A2UI,
                    vec![
                        134461564_u32,
                    ],
                ),
                number_of_voxel_fragments: 1,
                voxel_colors: (0, 0), // Irrelevant
                voxel_normals: (0, 0), // Irrelevant
            };
            let node_pool = helpers::generate_texture_buffer(8, gl::R32UI, 0_u32);
            let input = FlagNodesInput {
                octree_level: 0,
                voxel_data: &voxel_data,
                node_pool,
            };
            flag_nodes_pass.run(input);
        }
    }
}
