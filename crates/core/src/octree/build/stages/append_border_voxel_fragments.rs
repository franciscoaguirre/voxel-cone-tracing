use c_str_macro::c_str;
use engine::prelude::*;

use crate::{
    config::Config,
    octree::{OctreeData, OctreeTextures},
};

pub struct AppendBorderVoxelFragmentsPass {
    shader: Shader,
}

impl AppendBorderVoxelFragmentsPass {
    pub fn init() -> Self {
        Self {
            shader: compile_compute!("assets/shaders/octree/appendBorderVoxelFragments.comp.glsl",),
        }
    }

    pub unsafe fn run(
        &self,
        geometry_data: &OctreeData,
        border_data: &mut OctreeData,
        octree_level: u32,
        level_start: u32,
        nodes_in_current_level: u32,
        textures: &OctreeTextures,
    ) {
        log::debug!(
            "{}, {}, {}",
            octree_level,
            level_start,
            nodes_in_current_level
        );
        let config = Config::instance();
        self.shader.use_program();
        self.shader
            .set_uint(c_str!("maxOctreeLevel"), config.last_octree_level());
        self.shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.shader
            .set_uint(c_str!("levelStart"), level_start);
        self.shader
            .set_uint(c_str!("nextLevelStart"), level_start + nodes_in_current_level);
        self.shader
            .set_uint(c_str!("octreeLevel"), octree_level);


        helpers::bind_image_texture(
            0,
            border_data.voxel_data.voxel_positions.texture(),
            gl::WRITE_ONLY,
            gl::RGB10_A2UI,
        );
        helpers::bind_image_texture(1, textures.node_positions.0, gl::READ_ONLY, gl::RGB10_A2UI);

        let next_voxel_fragment_counter = helpers::generate_atomic_counter_buffer();
        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, next_voxel_fragment_counter);

        let (debug_texture, debug_texture_buffer) = helpers::generate_texture_buffer(20, gl::R32F, 42f32);
        helpers::bind_image_texture(5, debug_texture, gl::WRITE_ONLY, gl::R32F);

        self.run_pass(nodes_in_current_level, textures);

        let values = helpers::get_values_from_texture_buffer(debug_texture_buffer, 20, 0f32);
        // dbg!(&values);

        // Get the number of voxel fragments
        let number_of_voxel_fragments =
            helpers::get_value_from_atomic_counter(next_voxel_fragment_counter);
        border_data.voxel_data.number_of_voxel_fragments = number_of_voxel_fragments;
        log::debug!(
            "border voxel fragments for {}: {}",
            octree_level,
            number_of_voxel_fragments
        );
    }

    unsafe fn run_pass(&self, nodes_in_current_level: u32, textures: &OctreeTextures) {
        self.run_one(0, 2, nodes_in_current_level, textures);
        self.run_one(1, 2, nodes_in_current_level, textures);
        self.run_one(2, 4, nodes_in_current_level, textures);
        self.run_one(3, 4, nodes_in_current_level, textures);
        self.run_one(4, 0, nodes_in_current_level, textures);
        self.run_one(5, 0, nodes_in_current_level, textures);
    }

    unsafe fn run_one(
        &self,
        base_texture: usize,
        side_texture: usize,
        nodes_in_current_level: u32,
        textures: &OctreeTextures,
    ) {
        self.shader
            .set_uint(c_str!("offsetTexture"), base_texture as u32);
        self.shader
            .set_uint(c_str!("sideOffsetTexture"), side_texture as u32);
        helpers::bind_image_texture(
            2 as u32,
            textures.neighbors[base_texture].0,
            gl::READ_ONLY,
            gl::R32UI,
        );
        helpers::bind_image_texture(
            3 as u32,
            textures.neighbors[side_texture].0,
            gl::READ_ONLY,
            gl::R32UI,
        );
        helpers::bind_image_texture(
            4 as u32,
            textures.neighbors[side_texture + 1].0,
            gl::READ_ONLY,
            gl::R32UI,
        );

        let config = Config::instance();

        let groups_count =
            (nodes_in_current_level as f32 / config.working_group_size as f32).ceil() as u32;
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
