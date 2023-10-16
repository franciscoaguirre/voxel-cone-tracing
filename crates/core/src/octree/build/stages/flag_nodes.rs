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

        helpers::bind_image_texture(0, input.voxel_data.voxel_positions.0, gl::READ_WRITE, gl::RGB10_A2);
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

    /// The idea is to generate a testing node pool we can use to test the SVO construction,
    /// the SVO traversal, neighbors, etc.
    unsafe fn get_node_pool() -> BufferTexture {
        let data: Vec<u32> = vec![
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
        ];
        helpers::generate_texture_buffer_with_initial_data(8, gl::R32UI, data)
    }

    /// The idea is to generate a single u32 representing a RGB10_A2UI given three coordinates
    fn combine_bits(first: u32, second: u32, third: u32) -> u32 {
        // Mask to extract the first 10 bits of an integer
        let mask = 0x3FF;
        let offset = 10;

        // Combine the bits of the three integers
        let result = (first & mask) << 2 * offset | (second & mask) << offset | (third & mask);

        result
    }

    /// The idea is to generate a testing voxel fragment list we can use to feed the SVO construction
    unsafe fn get_voxel_data() -> VoxelData {
        VoxelData {
            voxel_positions: helpers::generate_texture_buffer_with_initial_data(
                1,
                gl::RGB10_A2UI,
                vec![
                    // TODO: Add useful testing voxels, probably very simple coordinates
                    combine_bits(124, 238, 128), // (124, 238, 128)
                ],
            ),
            number_of_voxel_fragments: 1,
            voxel_colors: (0, 0), // Irrelevant
            voxel_normals: (0, 0), // Irrelevant
        }        
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
        let voxel_dimension_exponent = 1;

        let flag_value = 1 << 31;

        unsafe {
            // Initialize everything
            Config::initialize(Config::new(voxel_dimension_exponent));
            let flag_nodes_pass = FlagNodesPass::init();
            let voxel_data = get_voxel_data();
            let node_pool = get_node_pool();
            // TODO: Test with multiple levels,
            // we should be able to test this independently from `allocate_nodes`.
            // Although after both are tested, we should write an integration test
            // for the both in sequence.
            let input = FlagNodesInput {
                octree_level: 0,
                voxel_data: &voxel_data,
                node_pool,
            };

            // Run the shader
            flag_nodes_pass.run(input);

            // Verify output
            // TODO: We only verify the first child pointer in the first node is flagged.
            // We should verify more once we have a reasonable voxel fragment list to test with.
            let output = helpers::get_values_from_texture_buffer(node_pool.1, 8, 42_u32);
            assert_eq!(output[0], flag_value);
            /// Why is this returning a bunch of 42 (default) when we initialize it with generate_texture_buffer_with_initial_data and write on it on flagNodes (with my breaking change on flagNodes.comp.glsl). One of the two should write on it
            let output = helpers::get_values_from_texture_buffer(voxel_data.voxel_positions.1, 10, 42_u32);
            assert_eq!(output[0..10], [0_u32; 10]);
        }
    }
}
