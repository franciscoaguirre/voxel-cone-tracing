use c_str_macro::c_str;

use crate::{
    config::CONFIG,
    helpers,
    octree::{OctreeData, OctreeTextures},
    rendering::shader::Shader,
};

pub struct AppendBorderVoxelFragmentsPass {
    shader: Shader,
}

impl AppendBorderVoxelFragmentsPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute(
                "assets/shaders/octree/appendBorderVoxelFragments.comp.glsl",
            ),
        }
    }

    pub unsafe fn run(
        &self,
        geometry_data: &OctreeData,
        border_data: &mut OctreeData,
        textures: &OctreeTextures,
    ) {
        self.shader.use_program();
        // Last level
        self.shader
            .set_uint(c_str!("octreeLevel"), CONFIG.octree_levels - 1);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.shader
            .set_uint(c_str!("callOffset"), 0);
        helpers::bind_image_texture(
            0,
            geometry_data.node_data.level_start_indices.0,
            gl::READ_ONLY,
            gl::R32UI,
        );
        helpers::bind_image_texture(
            1,
            border_data.voxel_data.voxel_positions.0,
            gl::WRITE_ONLY,
            gl::RGB10_A2UI,
        );
        helpers::bind_image_texture(2, textures.node_positions.0, gl::READ_ONLY, gl::RGB10_A2UI);

        let next_voxel_fragment_counter = helpers::generate_atomic_counter_buffer();
        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, next_voxel_fragment_counter);

        // Part 1
        for texture_offset in 0..(textures.neighbors.len() / 2) {
            helpers::bind_image_texture(
                3 + texture_offset as u32,
                textures.neighbors[texture_offset as usize].0,
                gl::READ_ONLY,
                gl::R32UI,
            );
        }

        self.shader.set_bool(c_str!("shouldStore"), false);
        self.shader
            .dispatch(geometry_data.voxel_data.number_of_voxel_fragments); // Call first with `shouldStore = false`
        self.shader.wait();

        for texture_offset in 0..(textures.neighbors.len() / 2) {
            helpers::bind_image_texture(
                3 + texture_offset as u32,
                textures.neighbors[(texture_offset + 3) as usize].0,
                gl::READ_ONLY,
                gl::R32UI,
            );
        }
        self.shader
            .set_uint(c_str!("callOffset"), 3);

        self.shader
            .dispatch(geometry_data.voxel_data.number_of_voxel_fragments); // Call first with `shouldStore = false`
        self.shader.wait();

        let number_of_voxel_fragments =
            helpers::get_value_from_atomic_counter(next_voxel_fragment_counter);
        border_data.voxel_data.number_of_voxel_fragments = number_of_voxel_fragments;


        for texture_offset in 0..(textures.neighbors.len() / 2) {
            helpers::bind_image_texture(
                3 + texture_offset as u32,
                textures.neighbors[texture_offset as usize].0,
                gl::READ_ONLY,
                gl::R32UI,
            );
        }
        self.shader
            .set_uint(c_str!("callOffset"), 0);

        self.shader.set_bool(c_str!("shouldStore"), true);
        self.shader.dispatch(
            (number_of_voxel_fragments as f32 / CONFIG.working_group_size as f32).ceil() as u32,
        );
        self.shader.wait();


        for texture_offset in 0..(textures.neighbors.len() / 2) {
            helpers::bind_image_texture(
                3 + texture_offset as u32,
                textures.neighbors[(texture_offset + 3) as usize].0,
                gl::READ_ONLY,
                gl::R32UI,
            );
        }
        self.shader
            .set_uint(c_str!("callOffset"), 3);

        self.shader.set_bool(c_str!("shouldStore"), true);
        self.shader.dispatch(
            (number_of_voxel_fragments as f32 / CONFIG.working_group_size as f32).ceil() as u32,
        );
        self.shader.wait();

        //let values = helpers::get_values_from_texture_buffer(
            //border_data.voxel_data.voxel_positions.1,
            //20,
            //420u32,
        //);
        //let values = values
            //.iter()
            //.map(|&position| {
                //let (x, y, z) = helpers::r32ui_to_rgb10_a2ui(position);
                //let text = format!("({x}, {y}, {z})");
                //text
            //})
            //.collect::<Vec<String>>();
        //dbg!(&values);
    }
}
