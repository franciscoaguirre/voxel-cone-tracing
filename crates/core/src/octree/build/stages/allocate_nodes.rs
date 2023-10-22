use c_str_macro::c_str;
use engine::prelude::*;

use crate::{
    config::Config,
    octree::{OctreeTextures, VoxelData},
};

pub struct AllocateNodesPass {
    shader: Shader,
}

#[derive(Clone)]
pub struct AllocateNodesInput {
    pub voxel_data: VoxelData,
    pub allocated_nodes_counter: u32,
    pub first_node_in_level: i32,
    pub first_free_node: i32,
    pub node_pool: BufferTextureV2<u32>,
}

impl AllocateNodesPass {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/allocateNodes.comp.glsl"),
        }
    }
}

impl ShaderPass for AllocateNodesPass {
    type Input<'a> = AllocateNodesInput;

    unsafe fn run<'a>(&self, input: Self::Input<'a>) {
        self.shader.use_program();

        self.shader
            .set_int(c_str!("firstNodeInLevel"), input.first_node_in_level);
        self.shader
            .set_int(c_str!("firstFreeNode"), input.first_free_node);
        gl::BindImageTexture(
            0,
            input.node_pool.texture(),
            0,
            gl::FALSE,
            0,
            gl::READ_WRITE,
            gl::R32UI,
        );
        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, input.allocated_nodes_counter);

        let config = Config::instance();

        // TODO: The number of nodes should be `nodes_per_level` but for that
        // `voxel_data` and `node_data` need to be the top level split instead of
        // `geometry_data` and `border_data`
        // TODO: Should move to its own function since we use it all over the place
        let groups_count = (input.voxel_data.number_of_voxel_fragments as f32
            / config.working_group_size as f32)
            .ceil() as u32;

        // TODO: Could send even less threads
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
        pub input: AllocateNodesInput,
        pub expected_output: Vec<u32>,
        pub description: String,
    }

    #[test]
    fn allocate_nodes_works() {
        let (_glfw, _window) = test_utils::init_opengl_context();

        // To go from the crate root to the workspace root
        // This is stateful, so not a good testing practice, doing it on first test ran for now
        let mut path = PathBuf::from(env::current_dir().unwrap());
        path.pop();
        path.pop();
        env::set_current_dir(path).unwrap();

        let voxel_dimension_exponent = 4;

        unsafe {
            let test_data = test_cases();

            // Initialize everything
            Config::initialize_test_sensitive(Config::new(voxel_dimension_exponent), true);
            let allocate_nodes_pass = AllocateNodesPass::init();
            let mut allocations: Vec<u32> = vec!();

            for TestCase { input, expected_output, description } in test_data.iter() {
                // Run the shader
                allocate_nodes_pass.run(input.clone());

                // Verify output
                let output = input.node_pool.data();
                println!("Running test: {description}");
                // Both the for and the later assert are because the allocated node number is non
                // deterministic, as it depends on each thread calling to the allocation counter
                for (index, &item) in expected_output.iter().enumerate() {
                    if item != u32::MAX {
                        assert_eq!(output[index], item);
                    } else {
                        allocations.push(output[index].clone());
                    }
                }
                let expected_allocations: Vec<u32> = (input.first_free_node as u32..input.first_free_node as u32 + allocations.len() as u32 ).collect();
                allocations.sort();
                assert_eq!(*allocations, *expected_allocations);
                allocations.clear();
                println!("Passed");
            }
        }
    }

    unsafe fn test_cases() -> std::vec::Vec<TestCase> {
        let F = 1 << 31;
        vec![
            TestCase {
                description: String::from("Base case, empty node pool"),
                input: AllocateNodesInput {
                    voxel_data: BufferTextureV2::from_data(vec![
                       helpers::rgb10_a2ui_to_r32ui(0, 0, 0),
                    ]).into(),
                    node_pool: BufferTextureV2::from_data(vec![
                      // Level 0
                      F, 0, 0, 0, 0, 0, 0, 0,
                      // Level 1
                      0, 0, 0, 0, 0, 0, 0, 0,
                      0, 0, 0, 0, 0, 0, 0, 0,
                    ]),
                    allocated_nodes_counter: helpers::generate_atomic_counter_buffer(),
                    first_node_in_level: 0,
                    first_free_node: 1,
                },
                expected_output: vec![
                    // Level 0
                    1, 0, 0, 0, 0, 0, 0, 0,
                    // Level 1
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                ],
            },
            TestCase {
                description: String::from("Base case, empty node pool"),
                input: AllocateNodesInput {
                    voxel_data: BufferTextureV2::from_data(vec![
                       helpers::rgb10_a2ui_to_r32ui(4, 4, 4),
                    ]).into(),
                    node_pool: BufferTextureV2::from_data(vec![
                      // Level 0
                      1, 0, 0, 0, 0, 0, 0, 0,
                      // Level 1
                      0, 0, 0, 0, 0, 0, 0, F,
                      0, 0, 0, 0, 0, 0, 0, 0,
                    ]),
                    allocated_nodes_counter: helpers::generate_atomic_counter_buffer(),
                    first_node_in_level: 1,
                    first_free_node: 2,
                },
                expected_output: vec![
                    // Level 0
                    1, 0, 0, 0, 0, 0, 0, 0,
                    // Level 1
                    0, 0, 0, 0, 0, 0, 0, 2,
                    0, 0, 0, 0, 0, 0, 0, 0,
                ],
            },
            TestCase {
                description: String::from("First two levels already complete, allocate many subnodes"),
                input: AllocateNodesInput {
                    voxel_data: BufferTextureV2::from_data(vec![
                       helpers::rgb10_a2ui_to_r32ui(0, 0, 0),
                       helpers::rgb10_a2ui_to_r32ui(8, 10, 6),
                       helpers::rgb10_a2ui_to_r32ui(8, 7, 10),
                       helpers::rgb10_a2ui_to_r32ui(8, 7, 9),
                    ]).into(),
                    node_pool: BufferTextureV2::from_data(vec![
                      // Level 0
                      1, 0, 0, 2, 0, 3, 0, 0,
                      // Level 1
                      4, 0, 0, 0, 0, 0, 0, 0,
                      0, 0, 0, 0, 5, 0, 0, 0,
                      0, 0, 6, 0, 0, 0, 0, 0,
                      // Level 2
                      F, 0, 0, 0, 0, 0, 0, 0,
                      0, 0, 0, 0, 0, 0, F, 0,
                      0, 0, F, 0, 0, 0, F, 0,
                    ],
                    ),
                    allocated_nodes_counter: helpers::generate_atomic_counter_buffer(),
                    first_node_in_level: 4,
                    first_free_node: 7,
                },
                expected_output: vec![
                  // Level 0
                  1, 0, 0, 2, 0, 3, 0, 0,
                  // Level 1
                  4, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 5, 0, 0, 0,
                  0, 0, 6, 0, 0, 0, 0, 0,
                  // Level 2
                  u32::MAX, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, u32::MAX, 0,
                  0, 0, u32::MAX, 0, 0, 0, u32::MAX, 0,
                ],
            },
        ]
    }
}
