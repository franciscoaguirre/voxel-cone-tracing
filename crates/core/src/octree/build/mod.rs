use log;

use super::{Octree, OctreeDataType, OctreeData};
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
        // Root node is in the geometry pool
        self.geometry_data.node_data.nodes_per_level.push(1);

        self.voxels_to_nodes(
            OctreeDataType::Geometry,
            self.allocated_nodes_counter, // Already initialized
        );
        let number_of_nodes = self.number_of_nodes() as u32;

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

        self.builder.leaf_border_transfer_pass.run(
            &self.textures,
            &self.geometry_data.node_data,
            BrickPoolValues::Colors,
        );

        #[cfg(debug_assertions)]
        self.run_mipmap(&self.geometry_data, BrickPoolValues::Colors);
    }

    pub unsafe fn add_dynamic_objects(
        &mut self,
        voxel_positions: BufferTextureV2<u32>,
        number_of_voxel_fragments: u32,
        voxel_colors: BufferTexture,
        voxel_normals: BufferTexture,
    ) {
        // TODO: Need to recreate dynamic data from scratch everytime
        // self.clear_dynamic_data();

        // Set voxel data
        self.dynamic_data.voxel_data.voxel_positions = voxel_positions;
        self.dynamic_data.voxel_data.number_of_voxel_fragments = number_of_voxel_fragments;
        self.dynamic_data.voxel_data.voxel_colors = voxel_colors;
        self.dynamic_data.voxel_data.voxel_normals = voxel_normals;

        // Root level is always static
        self.dynamic_data.node_data.nodes_per_level.push(0);

        self.voxels_to_nodes(
            OctreeDataType::Dynamic,
            self.allocated_nodes_counter,
        );

        // self.builder
        //     .write_leaf_nodes_pass
        //     .run(&self.dynamic_data.voxel_data, &self.textures);

        // self.builder
        //     .process_raw_brick_pool_colors
        //     .run(&self.dynamic_data.node_data, &self.textures);

        // self.builder
        //     .create_alpha_map
        //     .run(&self.textures, &self.dynamic_data.node_data);

        // self.builder.spread_leaf_bricks_pass.run(
        //     &self.textures,
        //     &self.dynamic_data.node_data,
        //     BrickPoolValues::Colors,
        // );

        // self.builder.leaf_border_transfer_pass.run(
        //     &self.textures,
        //     &self.dynamic_data.node_data,
        //     BrickPoolValues::Colors,
        // );

        // #[cfg(debug_assertions)]
        // self.run_mipmap(&self.dynamic_data, BrickPoolValues::Colors);
    }

    pub unsafe fn run_mipmap(&self, octree_data: &OctreeData, brick_pool_values: BrickPoolValues) {
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
                    &octree_data.node_data,
                    level,
                    *direction,
                    brick_pool_values,
                );

                self.builder.mipmap_isotropic_pass.run(
                    &self.textures,
                    &octree_data.node_data,
                    level,
                );

                if level > 0 {
                    self.builder.anisotropic_border_transfer_pass.run(
                        &self.textures,
                        &octree_data.node_data,
                        level,
                        brick_pool_values,
                        *direction,
                    );
                }
            }
        }
    }

    unsafe fn voxels_to_nodes(
        &mut self,
        octree_data_type: OctreeDataType,
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
            OctreeDataType::Dynamic => -1,
            OctreeDataType::Border => unimplemented!(),
        };
        octree_level_start_indices.push(first_level_start);

        let voxel_data = match octree_data_type {
            OctreeDataType::Geometry => &self.geometry_data.voxel_data,
            OctreeDataType::Dynamic => &self.dynamic_data.voxel_data,
            OctreeDataType::Border => unimplemented!(),
        };
        // Not necessary because we default to 0 on textures, and first node will always be 
        // at 0, 0, 0. But just in case...
        // self.builder
        //     .store_node_positions_pass
        //     .run(&self.textures, 0, &voxel_data);

        for octree_level in 1..=config.last_octree_level() {
            let previous_level_node_amount = match octree_data_type {
                OctreeDataType::Geometry => self.geometry_data.node_data.nodes_per_level[octree_level as usize - 1],
                OctreeDataType::Dynamic => 100000,
                OctreeDataType::Border => unimplemented!(),
            };

            log::debug!(
                "{:?} nodes in level {}: {}",
                octree_data_type,
                octree_level - 1,
                previous_level_node_amount,
            );

            // Flag and allocate previous level of octree with nodes for current level
            // of octree
            let flag_nodes_input = FlagNodesInput {
                octree_level: octree_level - 1,
                voxel_data: voxel_data.clone(),
                node_pool: BufferTextureV2::from_texture_and_buffer(self.textures.node_pool),
            };
            self.builder
                .flag_nodes_pass
                .run(flag_nodes_input);
            self.show_nodes(62, 10);
            let allocate_nodes_input = AllocateNodesInput {
                voxel_data: voxel_data.clone(),
                allocated_nodes_counter,
                first_node_in_level,
                first_free_node: self.first_free_node,
                node_pool: BufferTextureV2::from_texture_and_buffer(self.textures.node_pool),
                previous_level_node_amount,
            };
            self.builder.allocate_nodes_pass.run(allocate_nodes_input);

            let non_border_nodes_allocated = helpers::get_value_from_atomic_counter_without_reset(allocated_nodes_counter);
            log::debug!(
                "{octree_data_type:?} non border nodes allocated for {}: {}",
                octree_level,
                non_border_nodes_allocated
            );

            self.builder
                .store_node_positions_pass
                .run(&self.textures, octree_level, &voxel_data);

            self.builder.neighbor_pointers_pass.run(
                &voxel_data,
                &self.textures,
                octree_level,
                self.first_free_node as u32,
                non_border_nodes_allocated,
            );

            self.builder.append_border_voxel_fragments_pass.run(
                &mut self.border_data,
                octree_level,
                self.first_free_node as u32,
                non_border_nodes_allocated,
                &self.textures,
            );

            let flag_nodes_input = FlagNodesInput {
                octree_level: octree_level - 1,
                voxel_data: self.border_data.voxel_data.clone(),
                node_pool: BufferTextureV2::from_texture_and_buffer(self.textures.node_pool),
            };
            self.builder
                .flag_nodes_pass
                .run(flag_nodes_input);
            let allocate_nodes_input = AllocateNodesInput {
                voxel_data: self.border_data.voxel_data.clone(),
                allocated_nodes_counter,
                first_node_in_level,
                first_free_node: self.first_free_node,
                node_pool: BufferTextureV2::from_texture_and_buffer(self.textures.node_pool),
                previous_level_node_amount,
            };
            self.builder.allocate_nodes_pass.run(allocate_nodes_input);

            self.builder
                .store_node_positions_pass
                .run(&self.textures, octree_level, &self.border_data.voxel_data);

            let nodes_allocated = helpers::get_value_from_atomic_counter(allocated_nodes_counter);
            dbg!(&nodes_allocated);
            self.builder.neighbor_pointers_pass.run(
                &voxel_data,
                &self.textures,
                octree_level,
                self.first_free_node as u32,
                nodes_allocated,
            );

            // TODO: Separate node_data and voxel_data top-level.
            // The only mutable thing we need is node_data.
            // If we had node_data separate from voxel_data we could mutably borrow it and
            // immutably borrow voxel_data on a per voxel type basis.
            match octree_data_type {
                OctreeDataType::Geometry => self.geometry_data
                    .node_data
                    .nodes_per_level
                    .push(nodes_allocated),
                OctreeDataType::Dynamic => self.dynamic_data
                    .node_data
                    .nodes_per_level
                    .push(nodes_allocated),
                OctreeDataType::Border => unimplemented!(),
            }

            first_node_in_level = self.first_free_node;
            self.first_free_node += nodes_allocated as i32;

            octree_level_start_indices.push(first_node_in_level);
        }

        self.builder.store_node_positions_pass.run(
            &self.textures,
            config.last_octree_level(),
            &voxel_data,
        );

        // Start of level config.octree_levels(), shouldn't have anything
        // since we go from level 0 to config.octree_levels() - 1.
        // Works as an upper bound to the nodeIDs in config.octree_levels() - 1, the last level.
        octree_level_start_indices.push(self.first_free_node);

        let octree_data = match octree_data_type {
            OctreeDataType::Geometry => &self.geometry_data,
            OctreeDataType::Dynamic => &self.dynamic_data,
            OctreeDataType::Border => unimplemented!(),
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
