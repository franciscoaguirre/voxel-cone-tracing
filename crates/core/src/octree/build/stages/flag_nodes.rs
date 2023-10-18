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

#[derive(Clone)]
pub struct FlagNodesInput {
    pub octree_level: u32,
    pub voxel_data: VoxelData,
    pub node_pool: BufferTextureV2<u32>,
}

impl ShaderPass for FlagNodesPass {
    type Input<'a> = FlagNodesInput;

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

        helpers::bind_image_texture(0, input.voxel_data.voxel_positions.texture(), gl::READ_ONLY, gl::RGB10_A2);
        helpers::bind_image_texture(1, input.node_pool.texture(), gl::READ_WRITE, gl::R32UI);

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

    struct TestCase {
        pub input: FlagNodesInput,
        pub expected_output: Vec<u32>,
    }

    #[test]
    fn flag_nodes_works() {
        let (_glfw, _window) = test_utils::init_opengl_context();

        // To go from the crate root to the workspace root
        let mut path = PathBuf::from(env::current_dir().unwrap());
        path.pop();
        path.pop();
        env::set_current_dir(path).unwrap();

        // TODO: Make this small enough to understand,
        // but good enough to test what we want.
        let voxel_dimension_exponent = 3;

        // Flag left in node's child pointers to notify the child they point to
        // should be allocated, in `allocate_nodes`
        #[allow(non_snake_case)]
        let F = 1 << 31;

        unsafe {
            let test_data = vec![
                TestCase {
                    input: FlagNodesInput {
                        octree_level: 0,
                        voxel_data: BufferTextureV2::from_data(vec![
                            helpers::rgb10_a2ui_to_r32ui(0, 0, 0),
                        ]).into(),
                        node_pool: BufferTextureV2::from_data(vec![
                            // Level 0
                            0, 0, 0, 0, 0, 0, 0, 0,
                            // Level 1
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                        ]),
                    },
                    expected_output: vec![
                        // Level 0
                        F, 0, 0, 0, 0, 0, 0, 0,
                        // Level 1
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                },
                TestCase {
                    input: FlagNodesInput {
                        octree_level: 1,
                        voxel_data: BufferTextureV2::from_data(vec![
                            helpers::rgb10_a2ui_to_r32ui(0, 0, 0),
                        ]).into(),
                        node_pool: BufferTextureV2::from_data(vec![
                            // Level 0
                            1, 0, 0, 0, 0, 0, 0, 0,
                            // Level 1
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                        ]),
                    },
                    expected_output: vec![
                        // Level 0
                        1, 0, 0, 0, 0, 0, 0, 0,
                        // Level 1
                        F, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                },
            ];

            // Initialize everything
            Config::initialize(Config::new(voxel_dimension_exponent));
            let flag_nodes_pass = FlagNodesPass::init();

            for TestCase { input, expected_output } in test_data.iter() {
                // Run the shader
                flag_nodes_pass.run(input.clone());

                // Verify output
                let output = input.node_pool.data();
                assert_eq!(output, *expected_output);
            }
        }
    }
}
