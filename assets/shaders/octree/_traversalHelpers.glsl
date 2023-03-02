bool withinSecondHalf(float min, float halfNodeSize, float coordinatePosition) {
  // TODO: Is this wrong?
  return coordinatePosition >= min + halfNodeSize;
}

// Returns child local ID from 0 to 7.
uint calculateChildLocalID(vec3 nodeCoordinates, float halfNodeSize, vec3 fragment_position) {
  uint xOffset = uint(withinSecondHalf(nodeCoordinates.x, halfNodeSize, fragment_position.x)); // 0 or 1
  uint yOffset = uint(withinSecondHalf(nodeCoordinates.y, halfNodeSize, fragment_position.y)); // 0 or 1
  uint zOffset = uint(withinSecondHalf(nodeCoordinates.z, halfNodeSize, fragment_position.z)); // 0 or 1
  return xOffset + yOffset * 2 + zOffset * 4;
}

vec3 updateNodeCoordinates(
  vec3 currentNodeCoordinates,
  uint childLocalID,
  float currentHalfNodeSize
) {
  vec3 ret = currentNodeCoordinates;
  if (bool(childLocalID & 1)) {
    ret.x += currentHalfNodeSize;
  }
  if (bool(childLocalID & (1 << 1))) {
    ret.y += currentHalfNodeSize;
  }
  if (bool(childLocalID & (1 << 2))) {
    ret.z += currentHalfNodeSize;
  }

  return ret;
}
