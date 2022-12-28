// Dependencies:
// - _traversal_helpers

// NOTE: This file has a lot of duplicated code because we want to traverse
// the octree and do different things while we do it.
// _traversal_helpers aims to reuse code as much as possible.

// Every voxel fragment ends up in a "final" leaf.
// Empty nodes (tree leaves) in higher octree levels don't have
// a voxel fragment.
// This means there's no way to find a node for a voxel fragment
// in a different level than `max_octree_levels`.
// (as long as `max_octree_levels` < OCTREE_LEVELS)
int traverse_octree(
  vec3 voxel_coordinates, // Should be normalized, i.e. between 0 and 1
  int max_octree_levels,
  uimageBuffer node_pool
) {
  float current_half_node_size = 0.5; // Node side length normalized is 1, so half of that

  // Start journey in first tile, in node calculated via coordinates
  uint current_tile_index = 0;

  // Each node's coordinates are the coordinates of the point with lower (x, y, z)
  // within the node.
  vec3 current_node_coordinates = vec3(0, 0, 0);

  bvec3 subsection = calculate_node_subsection(current_node_coordinates,
                                               current_half_node_size,
                                               voxel_coordinates.xyz);

  uint current_node_index = calculate_node_index(current_tile_index, subsection);

  current_node_coordinates = update_node_coordinates(
    current_node_coordinates,
    subsection,
    current_half_node_size
  );

  for (uint i = 0; i < max_octree_levels; i++)
  {
    current_tile_index = imageLoad(node_pool, int(current_node_index)).r;

    bvec3 subsection = calculate_node_subsection(
      current_node_coordinates,
      current_half_node_size,
      voxel_coordinates.xyz
    );

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

int traverse_octree_returning_node_coordinates(
  vec3 voxel_coordinates, // Should be normalized, i.e. between 0 and 1
  int max_octree_levels,
  uimageBuffer node_pool,
  out float half_node_size,
  out vec3 node_coordinates,
  out uint tile_index
) {
  float current_half_node_size = 0.5; // Node side length normalized is 1, so half of that

  // Start journey in first tile, in node calculated via coordinates
  uint current_tile_index = 0;

  // Each node's coordinates are the coordinates of the point with lower (x, y, z)
  // within the node. We start with the root node at (0, 0, 0).
  vec3 current_node_coordinates = vec3(0, 0, 0);

  bvec3 subsection = calculate_node_subsection(
    current_node_coordinates,
    current_half_node_size,
    voxel_coordinates.xyz
  );

  uint current_node_index = calculate_node_index(current_tile_index, subsection);

  // NOTE: We set the return value before updating it just for level 1 because it should be
  // the default one for that level (0, 0, 0).
  // TODO: Maybe this isn't the best? We should look into it.
  node_coordinates = current_node_coordinates;

  current_node_coordinates = update_node_coordinates(
    current_node_coordinates,
    subsection,
    current_half_node_size
  );

  for (uint i = 0; i < max_octree_levels; i++)
  {
    current_tile_index = imageLoad(node_pool, int(current_node_index)).r;

    if (current_tile_index == 0) {
      break;
    }

    bvec3 subsection = calculate_node_subsection(
      current_node_coordinates,
      current_half_node_size,
      voxel_coordinates.xyz
    );

    current_node_index = calculate_node_index(current_tile_index, subsection);

    current_node_coordinates = update_node_coordinates(
      current_node_coordinates,
      subsection,
      current_half_node_size
    );

    current_half_node_size /= 2;

    node_coordinates = current_node_coordinates;
  }

  tile_index = current_tile_index;
  half_node_size = current_half_node_size;

  return int(current_node_index);
}

int traverse_octree_returning_level(
  vec3 voxel_coordinates, // Should be normalized, i.e. between 0 and 1
  int max_octree_levels,
  uimageBuffer node_pool,
  out uint found_on_level
) {
  float current_half_node_size = 0.5; // Node side length normalized is 1, so half of that

  // Start journey in first tile, in node calculated via coordinates
  uint current_tile_index = 0;

  // Each node's coordinates are the coordinates of the point with lower (x, y, z)
  // within the node.
  vec3 current_node_coordinates = vec3(0, 0, 0);

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

    if (current_tile_index == 0) {
        found_on_level = i;
        break;
    }

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
