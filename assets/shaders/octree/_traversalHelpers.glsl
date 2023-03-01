bool withinSecondHalf(float min, float halfNodeSize, float coordinatePosition) {
  // TODO: Is this wrong?
  return coordinatePosition >= min + halfNodeSize;
}

// Each node is divided into 8 subsections/children, for each coordinate the node is divided in two.
// Given the borders of the node, for each coordinate we calculate if the fragment is within the first or second
// half of the node.
bvec3 calculateNodeSubsection(vec3 nodeCoordinates, float halfNodeSize, vec3 fragment_position) {
  bvec3 subsection;
  subsection.x = withinSecondHalf(nodeCoordinates.x, halfNodeSize, fragment_position.x);
  subsection.y = withinSecondHalf(nodeCoordinates.y, halfNodeSize, fragment_position.y);
  subsection.z = withinSecondHalf(nodeCoordinates.z, halfNodeSize, fragment_position.z);
  return subsection;
}

// As we have one pointer for all 2x2x2 children, we calculate the index of the child this voxel fragment falls into
uint calculateNodeIndex(uint tileIndex, bvec3 subsection) {
  return (tileIndex * uint(NODES_PER_TILE)) + 
         uint(subsection[0]) +
         uint(subsection[1]) * 2 +
         uint(subsection[2]) * 4; // binary -> base10, this gives a unique index per subsection. Then add it to the tileIndex
}

vec3 updateNodeCoordinates(
  vec3 currentNodeCoordinates,
  bvec3 subsection,
  float currentHalfNodeSize
) {
  vec3 ret = currentNodeCoordinates;
  if (subsection.x) {
    ret.x += currentHalfNodeSize;
  }
  if (subsection.y) {
    ret.y += currentHalfNodeSize;
  }
  if (subsection.z) {
    ret.z += currentHalfNodeSize;
  }

  return ret;
}
