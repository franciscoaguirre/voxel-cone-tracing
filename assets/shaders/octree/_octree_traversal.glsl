bool within_second_half(uint min, uint half_node_size, uint coordinate_position) {
 return coordinate_position >= min + half_node_size;
}

// Each node is divided into 8 subsections/children, for each coordinate the node is divided in two.
// Given the borders of the node, for each coordinate we calculate if the fragment is within the first or second
// half of the node.
bvec3 calculate_node_subsection(uvec3 node_coordinates, uint half_node_size, uvec3 fragment_position) {
  bvec3 subsection;
  subsection.x = within_second_half(node_coordinates.x, half_node_size, fragment_position.x);
  subsection.y = within_second_half(node_coordinates.y, half_node_size, fragment_position.y);
  subsection.z = within_second_half(node_coordinates.z, half_node_size, fragment_position.z);
  return subsection;
}

// As we have one pointer for all 2x2x2 children, we calculate the index of the child this voxel fragment falls into
uint calculate_node_index(uint tile_index, bvec3 subsection) {
  return (tile_index * uint(NODES_PER_TILE)) + 
         uint(subsection[0]) +
         uint(subsection[1]) * 2 +
         uint(subsection[2]) * 4; // binary -> base10, this gives a unique index per subsection. Then add it to the tile_index
}

uvec3 update_node_coordinates(
  uvec3 current_node_coordinates,
  bvec3 subsection,
  uint current_half_node_size
) {
  uvec3 ret = current_node_coordinates;
  if (subsection.x) {
    ret.x += current_half_node_size;
  }
  if (subsection.y) {
    ret.y += current_half_node_size;
  }
  if (subsection.z) {
    ret.z += current_half_node_size;
  }

  return ret;
}
