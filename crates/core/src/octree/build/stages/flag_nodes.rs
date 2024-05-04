use c_str_macro::c_str;
use engine::prelude::*;

use crate::{config::Config, octree::OctreeTextures};

#[derive(Pausable)]
pub struct FlagNodesPass {
    shader: Shader,
    paused: bool,
    pause_next_frame: bool,
}

const SHADER_PATH: &'static str = "assets/shaders/octree/flagNodes.comp.glsl";

impl FlagNodesPass {
    pub fn new() -> Self {
        Self {
            shader: compile_compute!(SHADER_PATH),
            paused: false,
            pause_next_frame: false,
        }
    }
}

impl System for FlagNodesPass {
    unsafe fn setup(&mut self, assets: &mut AssetRegistry) {
        // Textures.
        assets.register_texture("nodePool", 0);

        // Uniforms.
        assets.register_uniform(self.get_info().name, "octreeLevel", Uniform::Uint(0));
    }

    unsafe fn update(&mut self, inputs: SystemInputs) {
        self.shader.use_program();
        let config = Config::instance();
        let Uniform::Uint(number_of_voxel_fragments) = *inputs
            .assets
            .get_uniform("SVOVoxelizer", "numberOfVoxelFragments")
            .unwrap()
        else {
            unreachable!()
        };
        self.shader
            .set_uint(c_str!("numberOfVoxelFragments"), number_of_voxel_fragments);
        let Uniform::Uint(octree_level) = *inputs
            .assets
            .get_uniform(self.get_info().name, "octreeLevel")
            .unwrap()
        else {
            unreachable!()
        };
        self.shader.set_uint(c_str!("octreeLevel"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        helpers::bind_image_texture(
            0,
            *inputs.assets.get_texture("voxelPositions").unwrap(),
            gl::READ_ONLY,
            gl::RGB10_A2,
        );
        helpers::bind_image_texture(
            1,
            *inputs.assets.get_texture("nodePool").unwrap(),
            gl::READ_WRITE,
            gl::R32UI,
        );
        let groups_count =
            (number_of_voxel_fragments as f32 / config.working_group_size as f32).ceil() as u32;
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }

    unsafe fn post_update(&mut self, _assets: &mut AssetRegistry) {}

    fn get_info(&self) -> SystemInfo {
        SystemInfo {
            name: "FlagNodesPass",
        }
    }
}

impl PausableSystem for FlagNodesPass {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;

    struct TestCase {
        pub input: FlagNodesInput,
        pub expected_output: Vec<u32>,
        pub description: String,
    }

    #[test]
    fn flag_nodes_works() {
        let (_glfw, _window) = test_utils::init_opengl_context();

        let voxel_dimension_exponent = 4;

        unsafe {
            let test_data = test_cases();

            // Initialize everything
            Config::initialize_test_sensitive(Config::new(voxel_dimension_exponent), true);
            let flag_nodes_pass = FlagNodesPass::new();

            for TestCase {
                input,
                expected_output,
                description,
            } in test_data.iter()
            {
                // Run the shader
                flag_nodes_pass.run(input.clone());

                // Verify output
                let output = input.node_pool.data();
                println!("Running test: {description}");
                assert_eq!(output, *expected_output);
                println!("Passed");
            }
        }
    }

    unsafe fn test_cases() -> std::vec::Vec<TestCase> {
        // Flag left in node's child pointers to notify the child they point to
        // should be allocated, in `allocate_nodes`
        #[allow(non_snake_case)]
        let F = 1 << 31;
        vec![
            TestCase {
                description: String::from(
                    "Base case, empty node pool, voxel_position on initial coordinate",
                ),
                input: FlagNodesInput {
                    octree_level: 0,
                    voxel_data: BufferTextureV2::from_data(vec![helpers::rgb10_a2ui_to_r32ui(
                        0, 0, 0,
                    )])
                    .into(),
                    node_pool: BufferTextureV2::from_data(vec![
                        // Level 0
                        0, 0, 0, 0, 0, 0, 0, 0, // Level 1
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ]),
                },
                expected_output: vec![
                    // Level 0
                    F, 0, 0, 0, 0, 0, 0, 0, // Level 1
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            },
            TestCase {
                description: String::from(
                    "First level already complete, flag subnode for x = 1, z = y = 0",
                ),
                input: FlagNodesInput {
                    octree_level: 1,
                    voxel_data: BufferTextureV2::from_data(vec![helpers::rgb10_a2ui_to_r32ui(
                        4, 2, 0,
                    )])
                    .into(),
                    node_pool: BufferTextureV2::from_data(vec![
                        // Level 0
                        1, 0, 0, 0, 0, 0, 0, 0, // Level 1
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ]),
                },
                expected_output: vec![
                    // Level 0
                    1, 0, 0, 0, 0, 0, 0, 0, // Level 1
                    0, F, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            },
            TestCase {
                description: String::from(
                    "First level already complete, flag subnode for x = y = z = 1",
                ),
                input: FlagNodesInput {
                    octree_level: 1,
                    voxel_data: BufferTextureV2::from_data(vec![helpers::rgb10_a2ui_to_r32ui(
                        4, 4, 4,
                    )])
                    .into(),
                    node_pool: BufferTextureV2::from_data(vec![
                        // Level 0
                        1, 0, 0, 0, 0, 0, 0, 0, // Level 1
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ]),
                },
                expected_output: vec![
                    // Level 0
                    1, 0, 0, 0, 0, 0, 0, 0, // Level 1
                    0, 0, 0, 0, 0, 0, 0, F, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            },
            TestCase {
                description: String::from(
                    "First two levels already complete, flag subnode for x = 0, z = y = 1",
                ),
                input: FlagNodesInput {
                    octree_level: 2,
                    voxel_data: BufferTextureV2::from_data(vec![helpers::rgb10_a2ui_to_r32ui(
                        4, 6, 6,
                    )])
                    .into(),
                    node_pool: BufferTextureV2::from_data(vec![
                        // Level 0
                        1, 0, 0, 0, 0, 0, 0, 0, // Level 1
                        0, 0, 0, 0, 0, 0, 0, 2, // Level 2
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ]),
                },
                expected_output: vec![
                    // Level 0
                    1, 0, 0, 0, 0, 0, 0, 0, // Level 1
                    0, 0, 0, 0, 0, 0, 0, 2, // Level 2
                    0, 0, 0, 0, 0, 0, F, 0,
                ],
            },
            TestCase {
                description: String::from("Empty node pool, multiple voxel positions"),
                input: FlagNodesInput {
                    octree_level: 0,
                    voxel_data: BufferTextureV2::from_data(vec![
                        helpers::rgb10_a2ui_to_r32ui(0, 0, 0),
                        helpers::rgb10_a2ui_to_r32ui(8, 10, 6),
                        helpers::rgb10_a2ui_to_r32ui(8, 7, 10),
                        helpers::rgb10_a2ui_to_r32ui(8, 6, 10),
                    ])
                    .into(),
                    node_pool: BufferTextureV2::from_data(vec![
                        // Level 0
                        0, 0, 0, 0, 0, 0, 0, 0, // Level 1
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ]),
                },
                expected_output: vec![
                    // Level 0
                    F, 0, 0, F, 0, F, 0, 0, // Level 1
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            },
            TestCase {
                description: String::from("First level already complete, flag many subnodes"),
                input: FlagNodesInput {
                    octree_level: 1,
                    voxel_data: BufferTextureV2::from_data(vec![
                        helpers::rgb10_a2ui_to_r32ui(0, 0, 0),
                        helpers::rgb10_a2ui_to_r32ui(8, 10, 6),
                        helpers::rgb10_a2ui_to_r32ui(8, 7, 10),
                        helpers::rgb10_a2ui_to_r32ui(8, 6, 10),
                    ])
                    .into(),
                    node_pool: BufferTextureV2::from_data(vec![
                        // Level 0
                        1, 0, 0, 2, 0, 3, 0, 0, // Level 1
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ]),
                },
                expected_output: vec![
                    // Level 0
                    1, 0, 0, 2, 0, 3, 0, 0, // Level 1
                    F, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, F, 0, 0, 0, 0, 0, F, 0, 0, 0, 0, 0,
                ],
            },
            TestCase {
                description: String::from("First two levels already complete, flag many subnodes"),
                input: FlagNodesInput {
                    octree_level: 2,
                    voxel_data: BufferTextureV2::from_data(vec![
                        helpers::rgb10_a2ui_to_r32ui(0, 0, 0),
                        helpers::rgb10_a2ui_to_r32ui(8, 10, 6),
                        helpers::rgb10_a2ui_to_r32ui(8, 7, 10),
                        helpers::rgb10_a2ui_to_r32ui(8, 7, 9),
                    ])
                    .into(),
                    node_pool: BufferTextureV2::from_data(vec![
                        // Level 0
                        1, 0, 0, 2, 0, 3, 0, 0, // Level 1
                        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0,
                        // Level 2
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ]),
                },
                expected_output: vec![
                    // Level 0
                    1, 0, 0, 2, 0, 3, 0, 0, // Level 1
                    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0,
                    // Level 2
                    F, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, F, 0, 0, 0, F, 0, 0, 0, F, 0,
                ],
            },
        ]
    }
}
