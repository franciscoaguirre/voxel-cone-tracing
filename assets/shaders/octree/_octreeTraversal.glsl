// Dependencies:
// - _constants
// - _traversalHelpers
// - uniform (r32ui) nodePool

int traverseOctree(
  vec3 queryCoordinates, // Should be normalized, i.e. between 0 and 1
  uint octreeLevels, // Will stop and return the node at this level
  out vec3 currentNodeCoordinates,
  out float currentHalfNodeSize
) {
  uint currentNodeID = 0; // Start journey in first node
  currentHalfNodeSize = 0.5; // Node side length normalized is 1, so half of that
  currentNodeCoordinates = vec3(0, 0, 0); // Node coordinates are centered on the corner with lower (x, y, z)

  for (uint i = 0; i < octreeLevels; i++)
  {
    uint childLocalID = calculateChildLocalID(currentNodeCoordinates, currentHalfNodeSize, queryCoordinates.xyz);
    uint childGlobalID = currentNodeID * CHILDREN_PER_NODE + childLocalID;
    currentNodeID = imageLoad(nodePool, int(childGlobalID)).r;
    if (currentNodeID == 0) {
      return NODE_NOT_FOUND;
    }
    currentNodeCoordinates = updateNodeCoordinates(currentNodeCoordinates, childLocalID, currentHalfNodeSize);
    currentHalfNodeSize /= 2;
  }
  
  return int(currentNodeID);
}
