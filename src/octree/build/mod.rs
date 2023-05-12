use log;

use super::{Octree, OctreeDataType};
use crate::{config::CONFIG, helpers};

mod stages;

use stages::*;

pub enum BrickPoolValues {
    Colors,
    Normals,
}

struct ShaderPasses {
    neighbor_pointers_pass: NeighborPointersPass,
    flag_nodes_pass: FlagNodesPass,
    allocate_nodes_pass: AllocateNodesPass,
    store_node_positions_pass: StoreNodePositions,
    write_leaf_nodes_pass: WriteLeafNodesPass,
    spread_leaf_bricks_pass: SpreadLeafBricksPass,
    border_transfer_pass: BorderTransferPass,
    mipmap_center_pass: MipmapCenterPass,
    mipmap_faces_pass: MipmapFacesPass,
    mipmap_corners_pass: MipmapCornersPass,
    mipmap_edges_pass: MipmapEdgesPass,
    append_border_voxel_fragments_pass: AppendBorderVoxelFragmentsPass,
}

impl Octree {
    pub unsafe fn build(&mut self) {
        let allocated_nodes_counter = helpers::generate_atomic_counter_buffer();

        // Root node is in the geometry pool, not the border one.
        self.geometry_data.node_data.nodes_per_level.push(1);
        self.border_data.node_data.nodes_per_level.push(0);

        let shader_passes = ShaderPasses {
            neighbor_pointers_pass: NeighborPointersPass::init(),
            flag_nodes_pass: FlagNodesPass::init(),
            allocate_nodes_pass: AllocateNodesPass::init(),
            store_node_positions_pass: StoreNodePositions::init(),
            write_leaf_nodes_pass: WriteLeafNodesPass::init(),
            spread_leaf_bricks_pass: SpreadLeafBricksPass::init(),
            border_transfer_pass: BorderTransferPass::init(),
            mipmap_center_pass: MipmapCenterPass::init(),
            mipmap_faces_pass: MipmapFacesPass::init(),
            mipmap_corners_pass: MipmapCornersPass::init(),
            mipmap_edges_pass: MipmapEdgesPass::init(),
            append_border_voxel_fragments_pass: AppendBorderVoxelFragmentsPass::init(),
        };

        let mut first_free_node = 1; // Index of first free node (unallocated) in the octree

        self.voxels_to_nodes(
            OctreeDataType::Geometry,
            &shader_passes,
            &mut first_free_node,
            allocated_nodes_counter,
        );

        shader_passes.append_border_voxel_fragments_pass.run(
            &self.geometry_data,
            &mut self.border_data,
            &self.textures,
        );

        // We reset first_node_in_level since it's used to know the parent to subdivide
        // We don't want to subdivide the last level, we want to start from scratch.
        // We don't reset first_free_node because it specifies where to add new nodes,
        // and we do want to add new nodes to the end of our buffer.
        // We also don't reset our counter because it's already in 0 when the for loop ends,
        // it resets everytime we get its value.
        // Append nodes to the end of the structure for representing the border voxels.
        // We have to go through all levels of our current structure to possibly subdivide nodes.
        self.voxels_to_nodes(
            OctreeDataType::Border,
            &shader_passes,
            &mut first_free_node,
            allocated_nodes_counter,
        );

        self.show_nodes(0, 8);
        self.show_nodes(2914, 8);
        self.show_nodes(11289, 8);
        self.show_nodes(11297, 8);

        shader_passes
            .write_leaf_nodes_pass
            .run(&self.geometry_data.voxel_data, &self.textures);
        shader_passes.spread_leaf_bricks_pass.run(
            &self.textures,
            &self.geometry_data.node_data,
            BrickPoolValues::Colors,
        );
        shader_passes.spread_leaf_bricks_pass.run(
            &self.textures,
            &self.geometry_data.node_data,
            BrickPoolValues::Normals,
        );

        shader_passes.border_transfer_pass.run(
            &self.textures,
            &self.geometry_data.node_data,
            CONFIG.octree_levels - 1,
            BrickPoolValues::Colors,
        );

        shader_passes.border_transfer_pass.run(
            &self.textures,
            &self.geometry_data.node_data,
            CONFIG.octree_levels - 1,
            BrickPoolValues::Normals,
        );

        for level in (0..CONFIG.octree_levels - 1).rev() {
            shader_passes.mipmap_center_pass.run(
                &self.textures,
                &self.geometry_data.node_data,
                level,
                BrickPoolValues::Colors,
            );
            shader_passes.mipmap_faces_pass.run(
                &self.textures,
                &self.geometry_data.node_data,
                level,
                BrickPoolValues::Colors,
            );
            shader_passes.mipmap_corners_pass.run(
                &self.textures,
                &self.geometry_data.node_data,
                level,
                BrickPoolValues::Colors,
            );
            shader_passes.mipmap_edges_pass.run(
                &self.textures,
                &self.geometry_data.node_data,
                level,
                BrickPoolValues::Colors,
            );

            shader_passes.mipmap_center_pass.run(
                &self.textures,
                &self.geometry_data.node_data,
                level,
                BrickPoolValues::Normals,
            );
            shader_passes.mipmap_faces_pass.run(
                &self.textures,
                &self.geometry_data.node_data,
                level,
                BrickPoolValues::Normals,
            );
            shader_passes.mipmap_corners_pass.run(
                &self.textures,
                &self.geometry_data.node_data,
                level,
                BrickPoolValues::Normals,
            );
            shader_passes.mipmap_edges_pass.run(
                &self.textures,
                &self.geometry_data.node_data,
                level,
                BrickPoolValues::Normals,
            );

            if level > 0 {
                shader_passes.border_transfer_pass.run(
                    &self.textures,
                    &self.geometry_data.node_data,
                    level,
                    BrickPoolValues::Colors,
                );
                shader_passes.border_transfer_pass.run(
                    &self.textures,
                    &self.geometry_data.node_data,
                    level,
                    BrickPoolValues::Normals,
                );
            }
        }
    }

    unsafe fn voxels_to_nodes(
        &mut self,
        octree_data_type: OctreeDataType,
        shader_passes: &ShaderPasses,
        first_free_node: &mut i32,
        allocated_nodes_counter: u32,
    ) {
        log::trace!("Voxels to nodes called for: {:?}", octree_data_type);

        let octree_data = match octree_data_type {
            OctreeDataType::Geometry => &mut self.geometry_data,
            OctreeDataType::Border => &mut self.border_data,
        };

        let mut octree_level_start_indices = Vec::with_capacity(CONFIG.octree_levels as usize);
        let mut first_node_in_level = 0; // Index of first node in a given octree level

        // First level of nodes only has root.
        // If the octree_data doesn't have root, this should have no meaning.
        let first_level_start = if octree_data
            .node_data
            .nodes_per_level
            .first()
            .filter(|&&nodes_on_first_level| nodes_on_first_level > 0)
            .is_some()
        {
            0
        } else {
            -1
        };
        octree_level_start_indices.push(first_level_start);

        for octree_level in 0..CONFIG.octree_levels - 1 {
            shader_passes.flag_nodes_pass.run(
                &octree_data.voxel_data,
                &self.textures,
                octree_level,
            );
            shader_passes.allocate_nodes_pass.run(
                &octree_data.voxel_data,
                &self.textures,
                allocated_nodes_counter,
                first_node_in_level,
                *first_free_node,
            );

            let nodes_allocated = helpers::get_value_from_atomic_counter(allocated_nodes_counter);
            log::info!(
                "{octree_data_type:?} nodes allocated for {}: {}",
                octree_level + 1,
                nodes_allocated
            );

            octree_data.node_data.nodes_per_level.push(nodes_allocated);
            first_node_in_level = *first_free_node;
            // TODO: Corroborar empíricamente
            // first_node_in_level +=
            //     octree_data.node_data.nodes_per_level[octree_level as usize] as i32;
            *first_free_node += nodes_allocated as i32;

            dbg!(&first_node_in_level);
            dbg!(&first_free_node);

            octree_level_start_indices.push(first_node_in_level);

            shader_passes.store_node_positions_pass.run(
                &self.textures,
                octree_level,
                &octree_data.voxel_data,
            );
            shader_passes.neighbor_pointers_pass.run(
                &octree_data.voxel_data,
                &octree_data.node_data,
                &self.textures,
                octree_level + 1,
            );
        }

        shader_passes.store_node_positions_pass.run(
            &self.textures,
            CONFIG.octree_levels - 1, // Last level
            &octree_data.voxel_data,
        );

        // Start of level CONFIG.octree_levels, shouldn't have anything
        // since we go from level 0 to CONFIG.octree_levels - 1.
        // Works as an upper bound to the nodeIDs in CONFIG.octree_levels - 1, the last level.
        octree_level_start_indices.push(*first_free_node);

        helpers::fill_texture_buffer_with_data(
            octree_data.node_data.level_start_indices.1,
            &octree_level_start_indices,
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
