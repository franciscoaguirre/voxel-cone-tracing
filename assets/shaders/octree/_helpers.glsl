const float fourThirds = 1.33333333;
const float twoThirds = 0.6666666;

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

uint findOctreeLevel(uint nodeID, readonly uimageBuffer levelStartIndices, uint maxOctreeLevel) {
    uint octreeLevel = 0;
    bool foundLevel = false;

    for (uint level = 0; level < maxOctreeLevel; level++) {
        uint levelStartIndex = imageLoad(levelStartIndices, int(level)).r;

        if (levelStartIndex > nodeID) {
            octreeLevel = level - 1;
            foundLevel = true;
            break;
        }

        if (!foundLevel) {
            octreeLevel = maxOctreeLevel - 1;
        }
    }

    return octreeLevel;
}

float calculateHalfNodeSize(uint octreeLevel) {
    return 0.5 / float(pow(2.0, float(octreeLevel)));
}

// Returns the node quarter the queryCoordinate is on
// Goes from 0 to 2
uint findQuarter(float min, float quarterNodeSize, float queryCoordinate) {
    bool withinFirstHalf = queryCoordinate < min + quarterNodeSize * 2;
    if (withinFirstHalf) {
        bool withinFirstQuarter = queryCoordinate < min + quarterNodeSize;
        if (withinFirstQuarter) {
            return 0;
        } else {
            return 1;
        }
    } else {
        bool withinThirdQuarter = queryCoordinate < min + quarterNodeSize * 3;
        if (withinThirdQuarter) {
            return 1;
        } else {
            return 2;
        }
    }
}

// Returns voxel ID from a brick, goes from 0 to 3^3 - 1
uvec3 calculateBrickVoxel(vec3 nodeCoordinates, float halfNodeSize, vec3 queryCoordinates) {
    float quarterNodeSize = halfNodeSize / 2.0;
    uint xOffset = findQuarter(nodeCoordinates.x, quarterNodeSize, queryCoordinates.x);
    uint yOffset = findQuarter(nodeCoordinates.y, quarterNodeSize, queryCoordinates.y);
    uint zOffset = findQuarter(nodeCoordinates.z, quarterNodeSize, queryCoordinates.z);
    return uvec3(xOffset, yOffset, zOffset);
}

float findQuarter2(float min, float halfNodeSize, float queryCoordinate, float voxelCoordinate) {
    float quarterNodeSize = halfNodeSize / 2.0;
    bool withinFirstHalf = queryCoordinate < min + halfNodeSize;
    if (withinFirstHalf) {
        bool withinFirstQuarter = queryCoordinate < min + quarterNodeSize;
        if (withinFirstQuarter) {
            // Maps first quarter to a third of the brick (that is the actual size on the texture)
            return voxelCoordinate * fourThirds;
        } else {
            // Maps second quarter to the first half of the second third of the brick (that is the actual size on the texture)
            return ((voxelCoordinate - quarterNodeSize) * twoThirds) + quarterNodeSize;
        }
    } else {
        bool withinThirdQuarter = queryCoordinate < min + quarterNodeSize * 3;
        if (withinThirdQuarter) {
            // Maps third quarter to the second half of the second third of the brick (that is the actual size on the texture)
            return ((voxelCoordinate - halfNodeSize) * twoThirds) + halfNodeSize;
        } else {
            // Maps last quarter to the last third of the brick (that is the actual size on the texture)
            return ((voxelCoordinate - (halfNodeSize + quarterNodeSize)) * fourThirds) + halfNodeSize + quarterNodeSize;
        }
    }
}

vec3 calculateNormalizedBrickVoxel(vec3 nodeCoordinates, float halfNodeSize, vec3 queryCoordinates) {
    vec3 voxelCoordinates = queryCoordinates - nodeCoordinates;
    vec3 offset;
    offset.x = findQuarter2(nodeCoordinates.x, halfNodeSize, queryCoordinates.x, voxelCoordinates.x);
    offset.y = findQuarter2(nodeCoordinates.y, halfNodeSize, queryCoordinates.y, voxelCoordinates.y);
    offset.z = findQuarter2(nodeCoordinates.z, halfNodeSize, queryCoordinates.z, voxelCoordinates.z);
    return clamp(offset / (halfNodeSize * 2.0), 0.0, 1.0);
}

ivec3 calculateBrickCoordinates(int nodeID) {
    ivec3 coordinates = ivec3(0);
    int brickPoolResolution = 384;
    int brickPoolResolutionBricks = brickPoolResolution / 3;
    coordinates.x = nodeID % brickPoolResolutionBricks;
    coordinates.y = (nodeID / brickPoolResolutionBricks) % brickPoolResolutionBricks;
    coordinates.z = nodeID / (brickPoolResolutionBricks * brickPoolResolutionBricks);
    coordinates *= 3;
    return coordinates;
}
