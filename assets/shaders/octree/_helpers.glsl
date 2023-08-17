const float fourThirds = 1.33333333;
const float twoThirds = 0.6666666;

vec4 convR32UIToVec4(uint val) {
    return vec4(
        float((val & 0x000000FF)),
        float((val & 0x0000FF00) >> 8U),
        float((val & 0x00FF0000) >> 16U),
        float((val & 0xFF000000) >> 24U)
    );
}

uint convVec4ToR32UI(vec4 val) {
    return (uint(val.w) & 0x000000FF) << 24U |
           (uint(val.z) & 0x000000FF) << 16U |
           (uint(val.y) & 0x000000FF) << 8U |
           (uint(val.x) & 0x000000FF);
}

uint vec3ToUintXYZ10(uvec3 val) {
    return (uint(val.z) & 0x000003FF)   << 20U
            |(uint(val.y) & 0x000003FF) << 10U 
            |(uint(val.x) & 0x000003FF);
}

uvec3 uintXYZ10ToVec3(uint val) {
    return uvec3(uint((val & 0x000003FF)),
                 uint((val & 0x000FFC00) >> 10U), 
                 uint((val & 0x3FF00000) >> 20U));
}

float calculateHalfNodeSize(uint octreeLevel) {
    return 0.5 / float(pow(2.0, float(octreeLevel)));
}

// Returns the node quarter the queryCoordinate is on
// Goes from 0 to 2
uint findQuarter(float min, float quarterNodeSize, float queryCoordinate) {
    if (queryCoordinate < min + quarterNodeSize) {
        return 0;
    } else if (queryCoordinate < min + quarterNodeSize * 3) {
        return 1;
    } else {
        return 2;
    }
}

// Returns voxel ID from a brick, goes from 0 to 3^3 - 1
uvec3 calculateBrickVoxel(vec3 nodeCoordinates, float halfNodeSize, vec3 queryCoordinates) {
    float quarterNodeSize = halfNodeSize / 2.0;
    uint xOffset = findQuarter(nodeCoordinates.x, quarterNodeSize, queryCoordinates.x);
    uint yOffset = findQuarter(nodeCoordinates.y, quarterNodeSize, queryCoordinates.y);
    //uint yOffset = 1;
    uint zOffset = findQuarter(nodeCoordinates.z, quarterNodeSize, queryCoordinates.z);
    return uvec3(xOffset, yOffset, zOffset);
}

uint findHalf(float min, float halfNodeSize, float queryCoordinate) {
    if (queryCoordinate < min + halfNodeSize) {
        return 0;
    } else {
        return 2;
    }
}

uvec3 calculateNodeVoxel(vec3 nodeCoordinates, float halfNodeSize, vec3 queryCoordinates) {
    uint xOffset = findHalf(nodeCoordinates.x, halfNodeSize, queryCoordinates.x);
    uint yOffset = findHalf(nodeCoordinates.y, halfNodeSize, queryCoordinates.y);
    //uint yOffset = 1;
    uint zOffset = findHalf(nodeCoordinates.z, halfNodeSize, queryCoordinates.z);
    return uvec3(0, 0, 0);
}

vec3 calculateNormalizedBrickVoxel(vec3 nodeCoordinates, float halfNodeSize, vec3 queryCoordinates) {
    vec3 voxelCoordinates = (queryCoordinates - nodeCoordinates) / (halfNodeSize * 2);
    return voxelCoordinates;
}

vec3 normalizedFromIntCoordinates(uvec3 intCoordinates, float factor) {
  vec3 centerVoxel = vec3(intCoordinates) + vec3(0.5);
  return centerVoxel / factor;
}

float zFromPlaneAndPoint(vec2 point, vec4 plane, float defaultValue) {
  if (plane.z == 0.0) {
    return defaultValue;
  }
  return (point.x * plane.x + point.y * plane.y + plane.w) / -plane.z;
}

bool isOutsideRange(vec3 val, vec3 lowerBound, vec3 higherBound) {
  bvec3 isGreater = greaterThan(val, higherBound);
  bvec3 isLess = lessThan(val, lowerBound);

  return any(isGreater) || any(isLess);
}

bool isOutsideRange(ivec3 val, ivec3 lowerBound, ivec3 higherBound) {
  bvec3 isGreater = greaterThan(val, higherBound);
  bvec3 isLess = lessThan(val, lowerBound);

  return any(isGreater) || any(isLess);
}

bool isOutsideRange(int val, int lowerBound, int higherBound) {
  bool isGreater = val > higherBound;
  bool isLess = val < lowerBound;

  return isGreater || isLess;
}
