// Dependencies:
// - _traversal_helpers

int traverse_octree(
  uvec3 voxel_coordinates,
  int voxel_dimension,
  int max_octree_levels,
  uimageBuffer node_pool
) {
  uint current_half_node_size = voxel_dimension / 2;

  // Start journey in first tile, in node calculated via coordinates
  uint current_tile_index = 0;

  // Each node's coordinates are the coordinates of the point with lower (x, y, z)
  // within the node.
  uvec3 current_node_coordinates = uvec3(0, 0, 0);

  bvec3 subsection = calculate_node_subsection(current_node_coordinates,
                                               current_half_node_size,
                                               voxel_coordinates.xyz);

  uint current_node_index = calculate_node_index(current_tile_index, subsection);

  current_node_coordinates = update_node_coordinates(
    current_node_coordinates,
    subsection,
    current_half_node_size
  );

  current_half_node_size /= 2;

  for (uint i = 0; i < max_octree_levels; i++)
  {
      current_tile_index = imageLoad(node_pool, int(current_node_index)).r;

      bvec3 subsection = calculate_node_subsection(current_node_coordinates,
                                                   current_half_node_size,
                                                   voxel_coordinates.xyz);

      current_node_index = calculate_node_index(current_tile_index, subsection);

      current_node_coordinates = update_node_coordinates(
        current_node_coordinates,
        subsection,
        current_half_node_size
      );

      current_half_node_size /= 2;
  }
  
  return int(current_node_index);
}