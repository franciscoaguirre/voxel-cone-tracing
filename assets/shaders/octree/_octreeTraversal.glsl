// Dependencies:
// - _traversal_helpers
// - uniform (r32ui) nodePool

// NOTE: This file has a lot of duplicated code because we want to traverse
// the octree and do different things while we do it.
// _traversal_helpers aims to reuse code as much as possible.

// Every voxel fragment ends up in a "final" leaf.
// Empty nodes (tree leaves) in higher octree levels don't have
// a voxel fragment.
// This means there's no way to find a node for a voxel fragment
// in a different level than `octreeLevels`.
// (as long as `octreeLevels` < OCTREE_LEVELS)
int traverseOctree(
  vec3 voxelCoordinates, // Should be normalized, i.e. between 0 and 1
  uint octreeLevels
) {
  float currentHalfNodeSize = 0.5; // Node side length normalized is 1, so half of that

  // Start journey in first tile, in node calculated via coordinates
  uint currentTileIndex = 0;

  // Each node's coordinates are the coordinates of the point with lower (x, y, z)
  // within the node.
  vec3 currentNodeCoordinates = vec3(0, 0, 0);

  bvec3 subsection = calculateNodeSubsection(currentNodeCoordinates,
                                               currentHalfNodeSize,
                                               voxelCoordinates.xyz);

  uint currentNodeIndex = calculateNodeIndex(currentTileIndex, subsection);

  currentNodeCoordinates = updateNodeCoordinates(
    currentNodeCoordinates,
    subsection,
    currentHalfNodeSize
  );

  for (uint i = 0; i < octreeLevels; i++)
  {
    currentTileIndex = imageLoad(nodePool, int(currentNodeIndex)).r;

    bvec3 subsection = calculateNodeSubsection(
      currentNodeCoordinates,
      currentHalfNodeSize,
      voxelCoordinates.xyz
    );

    currentNodeIndex = calculateNodeIndex(currentTileIndex, subsection);

    currentNodeCoordinates = updateNodeCoordinates(
      currentNodeCoordinates,
      subsection,
      currentHalfNodeSize
    );

    currentHalfNodeSize /= 2;
  }
  
  return int(currentNodeIndex);
}

int traverseOctreeReturningNodeCoordinates(
  vec3 voxelCoordinates, // Should be normalized, i.e. between 0 and 1
  uint octreeLevels,
  out float half_node_size,
  out vec3 node_coordinates,
  out uint tile_index
) {
  float currentHalfNodeSize = 0.5; // Node side length normalized is 1, so half of that

  // Start journey in first tile, in node calculated via coordinates
  uint currentTileIndex = 0;

  // Each node's coordinates are the coordinates of the point with lower (x, y, z)
  // within the node. We start with the root node at (0, 0, 0).
  vec3 currentNodeCoordinates = vec3(0, 0, 0);

  bvec3 subsection = calculateNodeSubsection(
    currentNodeCoordinates,
    currentHalfNodeSize,
    voxelCoordinates.xyz
  );

  uint currentNodeIndex = calculateNodeIndex(currentTileIndex, subsection);

  // NOTE: We set the return value before updating it just for level 1 because it should be
  // the default one for that level (0, 0, 0).
  // TODO: Maybe this isn't the best? We should look into it.
  node_coordinates = currentNodeCoordinates;

  currentNodeCoordinates = updateNodeCoordinates(
    currentNodeCoordinates,
    subsection,
    currentHalfNodeSize
  );

  for (uint i = 0; i < octreeLevels; i++)
  {
    currentTileIndex = imageLoad(nodePool, int(currentNodeIndex)).r;

    if (currentTileIndex == 0) {
      break;
    }

    bvec3 subsection = calculateNodeSubsection(
      currentNodeCoordinates,
      currentHalfNodeSize,
      voxelCoordinates.xyz
    );

    currentNodeIndex = calculateNodeIndex(currentTileIndex, subsection);

    currentNodeCoordinates = updateNodeCoordinates(
      currentNodeCoordinates,
      subsection,
      currentHalfNodeSize
    );

    currentHalfNodeSize /= 2;

    node_coordinates = currentNodeCoordinates;
  }

  tile_index = currentTileIndex;
  half_node_size = currentHalfNodeSize;

  return int(currentNodeIndex);
}

int traverseOctreeReturningLevel(
  vec3 voxelCoordinates, // Should be normalized, i.e. between 0 and 1
  uint octreeLevels,
  out uint found_on_level
) {
  float currentHalfNodeSize = 0.5; // Node side length normalized is 1, so half of that

  // Start journey in first tile, in node calculated via coordinates
  uint currentTileIndex = 0;

  // Each node's coordinates are the coordinates of the point with lower (x, y, z)
  // within the node.
  vec3 currentNodeCoordinates = vec3(0, 0, 0);

  bvec3 subsection = calculateNodeSubsection(currentNodeCoordinates,
                                               currentHalfNodeSize,
                                               voxelCoordinates.xyz);

  uint currentNodeIndex = calculateNodeIndex(currentTileIndex, subsection);

  currentNodeCoordinates = updateNodeCoordinates(
    currentNodeCoordinates,
    subsection,
    currentHalfNodeSize
  );

  found_on_level = octreeLevels;

  for (uint i = 0; i < octreeLevels; i++)
  {
    currentTileIndex = imageLoad(nodePool, int(currentNodeIndex)).r;
    
    if (currentTileIndex == 0) {
      found_on_level = i;
      break;
    }

    bvec3 subsection = calculateNodeSubsection(
      currentNodeCoordinates,
      currentHalfNodeSize,
      voxelCoordinates.xyz
    );

    currentNodeIndex = calculateNodeIndex(currentTileIndex, subsection);

    currentNodeCoordinates = updateNodeCoordinates(
      currentNodeCoordinates,
      subsection,
      currentHalfNodeSize
    );

    currentHalfNodeSize /= 2;
  }
  
  return int(currentNodeIndex);
}
