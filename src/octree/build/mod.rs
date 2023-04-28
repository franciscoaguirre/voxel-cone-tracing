use log::{debug, info};

use super::Octree;
use crate::{config::CONFIG, helpers};

mod stages;

use stages::*;

pub enum BrickPoolValues {
    Colors,
    Normals,
}

impl Octree {
    pub unsafe fn build(&mut self) {
        let allocated_nodes_counter = helpers::generate_atomic_counter_buffer();

        self.nodes_per_level.push(1);

        let neighbor_pointers_pass = NeighborPointersPass::init();
        let flag_nodes_pass = FlagNodesPass::init();
        let allocate_nodes_pass = AllocateNodesPass::init();
        let store_node_positions_pass = StoreNodePositions::init();
        let write_leaf_nodes_pass = WriteLeafNodesPass::init();
        let spread_leaf_bricks_pass = SpreadLeafBricksPass::init();
        let border_transfer_pass = BorderTransferPass::init();
        let mipmap_center_pass = MipmapCenterPass::init();
        let mipmap_faces_pass = MipmapFacesPass::init();
        let mipmap_corners_pass = MipmapCornersPass::init();
        let mipmap_edges_pass = MipmapEdgesPass::init();

        let mut octree_level_start_indices = Vec::with_capacity(CONFIG.octree_levels as usize);
        let mut first_node_in_level = 0; // Index of first node in a given octree level
        let mut first_free_node = 1; // Index of first free node (unallocated) in the octree

        octree_level_start_indices.push(first_node_in_level);

        for octree_level in 0..CONFIG.octree_levels {
            flag_nodes_pass.run(&self.voxel_data, &self.textures, octree_level);
            allocate_nodes_pass.run(
                &self.voxel_data,
                &self.textures,
                allocated_nodes_counter,
                first_node_in_level,
                first_free_node,
            );

            let nodes_allocated = helpers::get_value_from_atomic_counter(allocated_nodes_counter);
            info!(
                "Nodes allocated for level {}: {}",
                octree_level + 1,
                nodes_allocated
            );

            self.nodes_per_level.push(nodes_allocated);
            first_node_in_level += self.nodes_per_level[octree_level as usize] as i32;
            first_free_node += nodes_allocated as i32;

            octree_level_start_indices.push(first_node_in_level);

            neighbor_pointers_pass.run(&self.voxel_data, &self.textures, octree_level + 1);
        }

        // // TODO: Could maybe be done in the loop above
        for octree_level in 0..CONFIG.octree_levels {
            store_node_positions_pass.run(
                &self.textures,
                octree_level,
                self.voxel_data.number_of_voxel_fragments,
            );
        }

        helpers::fill_texture_buffer_with_data(
            self.textures.level_start_indices.1,
            &octree_level_start_indices,
        );

        debug!(
            "Octree level_start_indices: {:?}",
            &octree_level_start_indices
        );

        self.show_nodes(0, 8);

        // let all_nodes_allocated: u32 = self.nodes_per_level.iter().sum();

        write_leaf_nodes_pass.run(&self.voxel_data, &self.textures);
        spread_leaf_bricks_pass.run(
            &self.textures,
            &self.nodes_per_level,
            BrickPoolValues::Colors,
        );
        spread_leaf_bricks_pass.run(
            &self.textures,
            &self.nodes_per_level,
            BrickPoolValues::Normals,
        );

        border_transfer_pass.run(
            &self.textures,
            &self.nodes_per_level,
            BrickPoolValues::Colors,
        );

        border_transfer_pass.run(
            &self.textures,
            &self.nodes_per_level,
            BrickPoolValues::Normals,
        );

        for level in (0..CONFIG.octree_levels - 1).rev() {
            mipmap_center_pass.run(
                &self.textures,
                &self.nodes_per_level,
                level,
                BrickPoolValues::Colors,
            );
            mipmap_faces_pass.run(
                &self.textures,
                &self.nodes_per_level,
                level,
                BrickPoolValues::Colors,
            );
            mipmap_corners_pass.run(
                &self.textures,
                &self.nodes_per_level,
                level,
                BrickPoolValues::Colors,
            );
            mipmap_edges_pass.run(
                &self.textures,
                &self.nodes_per_level,
                level,
                BrickPoolValues::Colors,
            );

            mipmap_center_pass.run(
                &self.textures,
                &self.nodes_per_level,
                level,
                BrickPoolValues::Normals,
            );
            mipmap_faces_pass.run(
                &self.textures,
                &self.nodes_per_level,
                level,
                BrickPoolValues::Normals,
            );
            mipmap_corners_pass.run(
                &self.textures,
                &self.nodes_per_level,
                level,
                BrickPoolValues::Normals,
            );
            mipmap_edges_pass.run(
                &self.textures,
                &self.nodes_per_level,
                level,
                BrickPoolValues::Normals,
            );

            if level > 0 {
                border_transfer_pass.run(
                    &self.textures,
                    &self.nodes_per_level,
                    BrickPoolValues::Colors,
                );
                border_transfer_pass.run(
                    &self.textures,
                    &self.nodes_per_level,
                    BrickPoolValues::Normals,
                );
            }
        }
    }
}
