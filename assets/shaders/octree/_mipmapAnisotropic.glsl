// Requires:
// - _constants
// - _brickCoordinates
// - uniform uimageBuffer nodePool
// - uniform uimageBuffer directionalNeighbors
// - uniform image3D brickPoolValues

uniform int signn;
uniform int axis;

const float gaussianWeights[3] = { 1, 0.5, 0.25 };
//const float gaussianWeights[3] = { 1.0, 1.0, 1.0 };
vec4 adjacentVoxels[3][3][3];
int childNodeIDs[] = {0, 0, 0, 0, 0, 0, 0, 0};
int neighborChildNodeIDs[] = {0, 0, 0, 0, 0, 0, 0, 0};

const int X_AXIS = 0;
const int Y_AXIS = 1;
const int Z_AXIS = 2;

struct Direction {
    int axis;
    int sign;
};


void loadChildNodeIDs(int nodeID) {
    for (int i = 0; i < 8; i++) {
        int childNodeID = int(imageLoad(nodePool, nodeID * CHILDREN_PER_NODE + i).r);
        childNodeIDs[i] = childNodeID;
    }
    memoryBarrier();
}

void loadNeighborChildNodeIDs(int nodeID) {
    for (int i = 0; i < 8; i++) {
        int childNodeID = int(imageLoad(nodePool, nodeID * CHILDREN_PER_NODE + i).r);
        neighborChildNodeIDs[i] = childNodeID;
    }
    memoryBarrier();
}

void setup(int nodeID) {
  loadChildNodeIDs(nodeID);
  int neighborID = int(imageLoad(directionalNeighbors, nodeID).r);
  loadNeighborChildNodeIDs(neighborID);
  for(int x = 0; x < 3; x++) {
    for(int y = 0; y < 3; y++) { 
      for(int z = 0; z < 3; z++) {
        adjacentVoxels[x][y][z] = vec4(0);
      }
    }
  }
}

void accumulate(inout vec4 color, vec4 voxelColor) {
    color.rgb = color.rgb * color.a + (1 - color.a) * voxelColor.rgb * voxelColor.a;
    color.a += (1 - color.a) * voxelColor.a;
}

vec4 calculateDirectionalValue(ivec3 initialPosition, Direction direction) {
    ivec3 position = initialPosition;
    vec4 accumulatedColor = vec4(0);
    // "stepp" because "step" is a reserved keyword
    ivec3 stepp;

    if (direction.axis == X_AXIS) {
      stepp = ivec3(direction.sign, 0, 0);
    } else if (direction.axis == Y_AXIS) {
      stepp = ivec3(0, direction.sign, 0);
    } else if (direction.axis == Z_AXIS) {
      stepp = ivec3(0, 0, direction.sign);
    }

    for (int l = 0; l < 3; l++) {
        vec4 color = adjacentVoxels[position.x][position.y][position.z];
        // The center voxel has twice the width of the other ones
        if (l != 1) {
          color *= 0.5;
        }
        // Accumulate changes directly the value of accumulatedColor
        accumulate(accumulatedColor, color);
        position += stepp;
    }

    return accumulatedColor;
}

vec4 getNeighborColor(ivec3 position, int axis) {
    ivec3 clampedPosition = position;
    if (axis == X_AXIS) {
        if (position.x == -1) {
            clampedPosition.x = 4;
        } else if (position.x == 5) {
            clampedPosition.x = 0;
        }
    } else if (axis == Y_AXIS) {
        if (position.y == -1) {
            clampedPosition.y = 4;
        } else if (position.y == 5) {
            clampedPosition.y = 0;
        }
    } else if (axis == Z_AXIS) {
        if (position.z == -1) {
            clampedPosition.z = 4;
        } else if (position.z == 5) {
            clampedPosition.z = 0;
        }
    }
    ivec3 childOffset = ivec3(round(vec3(clampedPosition) / 4.0));
    int childIndex = childOffset.x + childOffset.y * 2 + childOffset.z * 4;
    ivec3 localPositionInChild = clampedPosition - 2 * childOffset;

    ivec3 childBrickAddress = calculateBrickCoordinates(neighborChildNodeIDs[childIndex]);
    return imageLoad(brickPoolChildrenColors, childBrickAddress + localPositionInChild);
}

vec4 getColor(ivec3 position) {
    ivec3 childOffset = ivec3(round(vec3(position) / 4.0));
    int childIndex = childOffset.x + childOffset.y * 2 + childOffset.z * 4;
    ivec3 localPositionInChild = position - 2 * childOffset;

    ivec3 childBrickAddress = calculateBrickCoordinates(childNodeIDs[childIndex]);
    return imageLoad(brickPoolChildrenColors, childBrickAddress + localPositionInChild);
}

void loadAdjacentVoxels(ivec3 position, Direction direction) {
    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            for (int z = -1; z <= 1; z++) {
                ivec3 lookupPosition = position + ivec3(x, y, z);

                bool shouldSkipX = (
                    direction.axis != X_AXIS &&
                    lookupPosition.x < 0 &&
                    lookupPosition.x > 4
                );
                bool shouldSkipY = (
                    direction.axis != Y_AXIS &&
                    lookupPosition.y < 0 &&
                    lookupPosition.y > 4
                );
                bool shouldSkipZ = (
                    direction.axis != Z_AXIS &&
                    lookupPosition.z < 0 &&
                    lookupPosition.z > 4
                );

                if (shouldSkipX || shouldSkipY || shouldSkipZ) {
                    continue;
                }

                bool calculateXNeighbor = (
                    direction.axis == X_AXIS &&
                    ((direction.sign == -1 && lookupPosition.x < 0) ||
                     (direction.sign == 1 && lookupPosition.x > 4))
                );
                bool calculateYNeighbor = (
                    direction.axis == Y_AXIS &&
                    ((direction.sign == -1 && lookupPosition.y < 0) ||
                     (direction.sign == 1 && lookupPosition.y > 4))
                );
                bool calculateZNeighbor = (
                    direction.axis == Z_AXIS &&
                    ((direction.sign == -1 && lookupPosition.z < 0) ||
                     (direction.sign == 1 && lookupPosition.z > 4))
                );

                vec4 color = vec4(0);
                if (calculateXNeighbor) {
                    color = getNeighborColor(lookupPosition, X_AXIS);
                } else if (calculateYNeighbor) {
                    color = getNeighborColor(lookupPosition, Y_AXIS);
                } else if (calculateZNeighbor) {
                    color = getNeighborColor(lookupPosition, Z_AXIS);
                } else if (!isOutsideRange(lookupPosition, ivec3(0), ivec3(4))) {
                    color = getColor(lookupPosition);
                }

                adjacentVoxels[x + 1][y + 1][z + 1] = color;
            }
        }
    }
}

/// `position`: Position of the parent voxel in joined coordinates
/// `axis`: One of the three axis:
/// - 0: X
/// - 1: Y
/// - 2: Z
/// - `sign`: -1 or 1 to indicate direction going forward or backwards
/// We should never call this function on parent voxels that are on the base of the direction
//vec4 mipmapAnisotropic(ivec3 position, Direction direction) {
// Encontrar una base de 9 voxels, dirección abajo hacia arriba agarramos los de abajo
vec4 mipmapAnisotropic(ivec3 position) {
    Direction direction = Direction(axis, signn);
    loadAdjacentVoxels(position, direction);
    vec4 color = vec4(0);
    int baseOffset = direction.sign == -1 ? 2 : 0;
    float weightSum = 0;
    float alphaWeightSum = 0;
    vec4 newColor = vec4(0);

    // Encontrar una base de 9 voxels, dirección abajo hacia arriba agarramos los de abajo
    for (int i = -1; i <= 1; i++) {
        for (int j = -1; j <= 1; j++) {
            int distance = abs(i) + abs(j);
            float weight = gaussianWeights[distance];

            if (direction.axis == X_AXIS) {
                bool shouldSkipY = position.y + i < 0 || position.y + i > 4;
                bool shouldSkipZ = position.z + j < 0 || position.z + j > 4;
                if (shouldSkipY || shouldSkipZ) {
                    continue;
                }

                newColor = calculateDirectionalValue(ivec3(baseOffset, i + 1, j + 1), direction) * weight;
                // To treat fully transparent colors as just affecting the transparency of the object 
                if (newColor.a > 0.001) {
                  color += newColor;
                  weightSum += weight;
                  alphaWeightSum += weight;
                } else {
                  alphaWeightSum += weight;
                }
            } else if (direction.axis == Y_AXIS) {
                bool shouldSkipX = position.x + i < 0 || position.x + i > 4;
                bool shouldSkipZ = position.z + j < 0 || position.z + j > 4;
                if (shouldSkipX || shouldSkipZ) {
                    continue;
                }

                newColor = calculateDirectionalValue(ivec3(i + 1, baseOffset, j + 1), direction) * weight;
                if (newColor.a > 0.001) {
                  color += newColor;
                  weightSum += weight;
                  alphaWeightSum += weight;
                } else {
                  alphaWeightSum += weight;
                }
            } else if (direction.axis == Z_AXIS) {
                bool shouldSkipX = position.x + i < 0 || position.x + i > 4;
                bool shouldSkipY = position.y + j < 0 || position.y + j > 4;
                if (shouldSkipX || shouldSkipY) {
                    continue;
                }

                newColor = calculateDirectionalValue(ivec3(i + 1, j + 1, baseOffset), direction) * weight;
                if (newColor.a > 0.001) {
                  color += newColor;
                  weightSum += weight;
                  alphaWeightSum += weight;
                } else {
                  alphaWeightSum += weight;
                }
            }
        }
    }

    //return adjacentVoxels[2][1][1];
    return vec4(color.rgb / weightSum, color.a / alphaWeightSum);
}
