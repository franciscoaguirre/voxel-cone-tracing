use log;

use super::{Octree, OctreeDataType};
use crate::{
    config::Config,
    constants::{Direction, Sign, Axis},
};
use engine::prelude::*;

mod stages;

pub use stages::*;

#[derive(Debug, Clone, Copy)]
pub enum BrickPoolValues {
    Colors,
    Normals,
    Irradiance,
}

impl Octree {
    pub unsafe fn build(&mut self) {
        let allocated_nodes_counter = helpers::generate_atomic_counter_buffer();

        // Root node is in the geometry pool, not the border one.
        self.geometry_data.node_data.nodes_per_level.push(1);
        self.border_data.node_data.nodes_per_level.push(0);

        let mut first_free_node = 1; // Index of first free node (unallocated) in the octree

        self.voxels_to_nodes(
            OctreeDataType::Geometry,
            &mut first_free_node,
            allocated_nodes_counter,
        );
        let number_of_nodes = self.number_of_nodes() as u32;

        self.builder.append_border_voxel_fragments_pass.run(
            &self.geometry_data,
            &mut self.border_data,
            &self.textures,
        );

        self.voxels_to_nodes(
            OctreeDataType::Border,
            &mut first_free_node,
            allocated_nodes_counter,
        );

        self.builder
            .write_leaf_nodes_pass
            .run(&self.geometry_data.voxel_data, &self.textures);

        self.builder
            .process_raw_brick_pool_colors
            .run(&self.geometry_data.node_data, &self.textures);

        self.builder
            .create_alpha_map
            .run(&self.textures, &self.geometry_data.node_data);

        self.builder.spread_leaf_bricks_pass.run(
            &self.textures,
            &self.geometry_data.node_data,
            BrickPoolValues::Colors,
        );

        // self.builder.spread_leaf_bricks_pass.run(
        //     &self.textures,
        //     &self.geometry_data.node_data,
        //     BrickPoolValues::Normals,
        // );

        self.builder.leaf_border_transfer_pass.run(
            &self.textures,
            &self.geometry_data.node_data,
            &self.border_data.node_data,
            BrickPoolValues::Colors,
        );

        #[cfg(debug_assertions)]
        self.run_mipmap(BrickPoolValues::Colors);
    }

    pub unsafe fn run_mipmap(&self, brick_pool_values: BrickPoolValues) {
        let all_directions = vec![
            Direction::new(Axis::X, Sign::Pos),
            Direction::new(Axis::X, Sign::Neg),
            Direction::new(Axis::Y, Sign::Pos),
            Direction::new(Axis::Y, Sign::Neg),
            Direction::new(Axis::Z, Sign::Pos),
            Direction::new(Axis::Z, Sign::Neg),
        ];

        let config = Config::instance();

        for level in (0..config.octree_levels() - 1).rev() {
            for direction in all_directions.iter() {
                self.builder.mipmap_anisotropic_pass.run(
                    &self.textures,
                    &self.geometry_data.node_data,
                    level,
                    *direction,
                    brick_pool_values,
                );

                self.builder.mipmap_isotropic_pass.run(
                    &self.textures,
                    &self.geometry_data.node_data,
                    level,
                );

                if level > 0 {
                    self.builder.anisotropic_border_transfer_pass.run(
                        &self.textures,
                        &self.geometry_data.node_data,
                        &self.border_data.node_data,
                        level,
                        brick_pool_values,
                        *direction,
                    );
                    // self.builder.border_transfer_pass.run(
                    //     &self.textures,
                    //     &self.geometry_data.node_data,
                    //     level,
                    //     BrickPoolValues::Normals,
                    // );
                }
            }
        }
    }

    unsafe fn voxels_to_nodes(
        &mut self,
        octree_data_type: OctreeDataType,
        first_free_node: &mut i32,
        allocated_nodes_counter: u32,
    ) {
        log::trace!("Voxels to nodes called for: {:?}", octree_data_type);

        let config = Config::instance();

        let mut octree_level_start_indices = Vec::with_capacity(config.octree_levels() as usize);
        let mut first_node_in_level = 0; // Index of first node in a given octree level

        // First level of nodes only has root.
        // If the octree_data doesn't have root, this should have no meaning.
        let first_level_start = match octree_data_type {
            OctreeDataType::Geometry => 0,
            OctreeDataType::Border => -1,
        };
        octree_level_start_indices.push(first_level_start);

        let voxel_data = match octree_data_type {
            OctreeDataType::Geometry => &self.geometry_data.voxel_data,
            OctreeDataType::Border => &self.border_data.voxel_data,
        };

        for octree_level in 0..config.octree_levels() - 1 {
            let flag_nodes_input = FlagNodesInput {
                octree_level,
                voxel_data: &voxel_data,
                node_pool: self.textures.node_pool,
            };
            self.builder
                .flag_nodes_pass
                .run(flag_nodes_input);
            self.builder.allocate_nodes_pass.run(
                &self.geometry_data.voxel_data, // TODO: Pass in the actual number of nodes
                &self.textures,
                allocated_nodes_counter,
                first_node_in_level,
                *first_free_node,
            );
            if let OctreeDataType::Border = octree_data_type {
                let geometry_level_start_indices = helpers::get_values_from_texture_buffer(
                    self.geometry_data.node_data.level_start_indices.1,
                    (config.octree_levels() + 1) as usize,
                    0u32,
                );
                let geometry_first_node_in_level =
                    geometry_level_start_indices[octree_level as usize];

                self.builder.allocate_nodes_pass.run(
                    &self.geometry_data.voxel_data, // TODO: Pass in the actual number of nodes
                    &self.textures,
                    allocated_nodes_counter,
                    geometry_first_node_in_level as i32,
                    *first_free_node,
                );
            }

            let nodes_allocated = helpers::get_value_from_atomic_counter(allocated_nodes_counter);
            log::debug!(
                "{octree_data_type:?} nodes allocated for {}: {}",
                octree_level + 1,
                nodes_allocated
            );

            // TODO: Separate node_data and voxel_data top-level.
            // The only mutable thing we need is node_data.
            // If we had node_data separate from voxel_data we could mutably borrow it and
            // immutably borrow voxel_data on a per voxel type basis.
            match octree_data_type {
                OctreeDataType::Geometry => self
                    .geometry_data
                    .node_data
                    .nodes_per_level
                    .push(nodes_allocated),
                OctreeDataType::Border => self
                    .border_data
                    .node_data
                    .nodes_per_level
                    .push(nodes_allocated),
            }
            first_node_in_level = *first_free_node;
            // TODO: Corroborar empíricamente
            // first_node_in_level +=
            //     octree_data.node_data.nodes_per_level[octree_level as usize] as i32;
            *first_free_node += nodes_allocated as i32;

            octree_level_start_indices.push(first_node_in_level);

            self.builder
                .store_node_positions_pass
                .run(&self.textures, octree_level, &voxel_data);

            match octree_data_type {
                OctreeDataType::Geometry => self.builder.neighbor_pointers_pass.run(
                    &self.geometry_data.voxel_data,
                    &self.geometry_data.node_data,
                    &self.textures,
                    octree_level + 1,
                ),
                OctreeDataType::Border => {
                    self.builder.neighbor_pointers_pass.run(
                        &self.geometry_data.voxel_data,
                        &self.geometry_data.node_data,
                        &self.textures,
                        octree_level + 1,
                    );
                    self.builder.neighbor_pointers_pass.run(
                        &voxel_data,
                        &self.border_data.node_data,
                        &self.textures,
                        octree_level + 1,
                    );
                }
            }
        }

        //let output = helpers::get_values_from_texture_buffer(self.geometry_data.voxel_data.voxel_positions.1, 10, 42_u32);
        //let lol: Vec<[u32; 3]> = output[0..10].into_iter().map(|m| unscramble_position_from_int(m)).collect();
        //dbg!(lol);

        self.builder.store_node_positions_pass.run(
            &self.textures,
            config.octree_levels() - 1, // Last level
            &voxel_data,
        );

        // Start of level config.octree_levels(), shouldn't have anything
        // since we go from level 0 to config.octree_levels() - 1.
        // Works as an upper bound to the nodeIDs in config.octree_levels() - 1, the last level.
        octree_level_start_indices.push(*first_free_node);

        let octree_data = match octree_data_type {
            OctreeDataType::Geometry => &self.geometry_data,
            OctreeDataType::Border => &self.border_data,
        };

        helpers::fill_texture_buffer_with_data(
            octree_data.node_data.level_start_indices.1,
            &octree_level_start_indices,
            gl::STATIC_DRAW,
        );

        log::debug!(
            "{octree_data_type:?} nodes_per_level: {:?}",
            &octree_data.node_data.nodes_per_level
        );
        log::debug!(
            "{octree_data_type:?} level_start_indices: {:?}",
            &octree_level_start_indices
        );
    }
}
