use c_str_macro::c_str;

use crate::{
    config::CONFIG,
    constants::{Axis, Direction, Sign},
    helpers,
    octree::{build::BrickPoolValues, NodeData, OctreeTextures},
    rendering::shader::Shader,
};

pub struct BorderTransferPass {
    shader: Shader,
}

impl BorderTransferPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/borderTransfer.comp.glsl"),
        }
    }

    /// Runs the border transfer pass.
    /// This runs border transfer for geometry and border nodes in sequence
    /// so as to generate cohesive values.
    pub unsafe fn run(
        &self,
        textures: &OctreeTextures,
        geometry_node_data: &NodeData,
        border_node_data: &NodeData,
        octree_level: u32,
        brick_pool_values: BrickPoolValues,
        direction: Direction,
    ) {
        self.shader.use_program();

        self.shader.set_uint(c_str!("octreeLevel"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        // We want the direction to be the same for both runs (geometry and border)
        self.shader
            .set_int(c_str!("direction.axis"), direction.axis.into());
        self.shader
            .set_int(c_str!("direction.sign"), direction.sign.into());

        let mut neighbors_texture_number = match direction.axis {
            Axis::X => 0,
            Axis::Y => 2,
            Axis::Z => 4,
        };
        neighbors_texture_number = match direction.sign {
            Sign::Pos => neighbors_texture_number,
            Sign::Neg => neighbors_texture_number + 1,
        };

        match brick_pool_values {
            BrickPoolValues::Colors => helpers::bind_3d_image_texture(
                1,
                textures.brick_pool_colors[neighbors_texture_number],
                gl::READ_WRITE,
                gl::RGBA8,
            ),
            BrickPoolValues::Normals => helpers::bind_3d_image_texture(
                1,
                textures.brick_pool_normals,
                gl::READ_WRITE,
                gl::RGBA32F,
            ),
        }
        let other_axis = if direction.axis == Axis::X {
            vec![Axis::Y, Axis::Z]
        } else if direction.axis == Axis::Y {
            vec![Axis::X, Axis::Z]
        } else {
            vec![Axis::X, Axis::Y]
        };

        // First run for geometry nodes
        helpers::bind_image_texture(
            2,
            geometry_node_data.level_start_indices.0,
            gl::READ_ONLY,
            gl::R32UI,
        );
        let nodes_in_level = geometry_node_data.nodes_per_level[octree_level as usize];
        let groups_count = (nodes_in_level as f32 / CONFIG.working_group_size as f32).ceil() as u32;
        for axis in other_axis.iter() {
            self.shader.set_uint(c_str!("axis"), (*axis).into());
            let mut neighbors_texture_number = match axis {
                Axis::X => 0,
                Axis::Y => 2,
                Axis::Z => 4,
            };
            helpers::bind_image_texture(
                0,
                textures.neighbors[neighbors_texture_number].0,
                gl::READ_ONLY,
                gl::R32UI,
            );
            self.shader.dispatch(groups_count);
            self.shader.wait();
        }
        let mut neighbors_texture_number = match direction.axis {
            Axis::X => 0,
            Axis::Y => 2,
            Axis::Z => 4,
        };
        neighbors_texture_number = match direction.sign {
            Sign::Pos => neighbors_texture_number,
            Sign::Neg => neighbors_texture_number + 1,
        };
        helpers::bind_image_texture(
            0,
            textures.neighbors[neighbors_texture_number].0,
            gl::READ_ONLY,
            gl::R32UI,
        );
        self.shader.set_uint(c_str!("axis"), direction.axis.into());
        self.shader.dispatch(groups_count);
        self.shader.wait();

        // Second run for border nodes
        // helpers::bind_image_texture(
        //     2,
        //     border_node_data.level_start_indices.0,
        //     gl::READ_ONLY,
        //     gl::R32UI,
        // );
        // let nodes_in_level = border_node_data.nodes_per_level[octree_level as usize];
        // let groups_count = (nodes_in_level as f32 / CONFIG.working_group_size as f32).ceil() as u32;
        // for axis in other_axis.iter() {
        //     self.shader.set_uint(c_str!("axis"), (*axis).into());
        //     let mut neighbors_texture_number = match axis {
        //         Axis::X => 0,
        //         Axis::Y => 2,
        //         Axis::Z => 4,
        //     };
        //     neighbors_texture_number = match direction.sign {
        //         Sign::Pos => neighbors_texture_number,
        //         Sign::Neg => neighbors_texture_number + 1,
        //     };
        //     helpers::bind_image_texture(
        //         0,
        //         textures.neighbors[neighbors_texture_number].0,
        //         gl::READ_ONLY,
        //         gl::R32UI,
        //     );
        //     self.shader.dispatch(groups_count);
        //     self.shader.wait();
        // }
        // let mut neighbors_texture_number = match direction.axis {
        //     Axis::X => 0,
        //     Axis::Y => 2,
        //     Axis::Z => 4,
        // };
        // neighbors_texture_number = match direction.sign {
        //     Sign::Pos => neighbors_texture_number,
        //     Sign::Neg => neighbors_texture_number + 1,
        // };
        // helpers::bind_image_texture(
        //     0,
        //     textures.neighbors[neighbors_texture_number].0,
        //     gl::READ_ONLY,
        //     gl::R32UI,
        // );
        // self.shader.set_uint(c_str!("axis"), direction.axis.into());
        // self.shader.dispatch(groups_count);
        // self.shader.wait();
    }
}
