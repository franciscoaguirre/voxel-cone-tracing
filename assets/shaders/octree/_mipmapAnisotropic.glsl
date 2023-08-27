// Requires:
// - _constants
// - _brickCoordinates
// - uniform uimageBuffer nodePool
// - uniform uimageBuffer directionalNeighbors
// - uniform image3D brickPoolValuesRead

const float gaussianWeights[3] = { 1, 0.5, 0.25 };
vec4 adjacentVoxels[3][3][3];
int childNodeIDs[] = {0, 0, 0, 0, 0, 0, 0, 0};
int neighborChildNodeIDs[] = {0, 0, 0, 0, 0, 0, 0, 0};

struct Direction {
    int axis;
    int sign;
};

uniform Direction direction;

void loadChildNodeIDs(int nodeID) {
    for (int i = 0; i < 8; i++) {
        int childNodeID = int(imageLoad(nodePool, nodeID * CHILDREN_PER_NODE + i).r);
        childNodeIDs[i] = childNodeID;
    }
    memoryBarrier();
}

void loadNeighborChildNodeIDs(int neighborNodeID) {
    for (int i = 0; i < 8; i++) {
        int childNodeID = int(imageLoad(nodePool, neighborNodeID * CHILDREN_PER_NODE + i).r);
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
    color += (1 - color.a) * voxelColor;
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
        // TODO: Without this, sponza looks much nicer.
        // But, the triangle seen from the front is much darker.
        // if (l != 1) {
        //   color.a *= 0.5;
        // }
        // Accumulate changes directly the value of accumulatedColor
        accumulate(accumulatedColor, color);
        if (accumulatedColor.a >= 1) {
            break;
        }
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
    return imageLoad(brickPoolValuesRead, childBrickAddress + localPositionInChild);
}

vec4 getColor(ivec3 position) {
    ivec3 childOffset = ivec3(round(vec3(position) / 4.0));
    int childIndex = childOffset.x + childOffset.y * 2 + childOffset.z * 4;
    ivec3 localPositionInChild = position - 2 * childOffset;

    ivec3 childBrickAddress = calculateBrickCoordinates(childNodeIDs[childIndex]);
    return imageLoad(brickPoolValuesRead, childBrickAddress + localPositionInChild);
}

void loadAdjacentVoxels(ivec3 position, Direction direction) {
    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            for (int z = -1; z <= 1; z++) {
                ivec3 lookupPosition = position + ivec3(x, y, z);

                bool shouldSkipX = (
                    direction.axis != X_AXIS &&
                    isOutsideRange(lookupPosition.x, 0, 4)
                );
                bool shouldSkipY = (
                    direction.axis != Y_AXIS &&
                    isOutsideRange(lookupPosition.y, 0, 4)
                );
                bool shouldSkipZ = (
                    direction.axis != Z_AXIS &&
                    isOutsideRange(lookupPosition.z, 0, 4)
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
vec4 mipmapAnisotropic(ivec3 position) {
    loadAdjacentVoxels(position, direction);
    vec4 color = vec4(0);
    int baseOffset = direction.sign == -1 ? 2 : 0;
    float weightSum = 0;
    // TODO: Empty directional values are making the overall value darker.
    // This could be solved if we knew how many directional values were empty beforehand, but we can't know that
    // unless we get all neighbors, and if we do that, we might as well use them for the calculation instead of
    // doing partial averages.
    vec4 newColor = vec4(0);

    // Encontrar una base de 9 voxels, dirección abajo hacia arriba agarramos los de abajo
    for (int i = -1; i <= 1; i++) {
        for (int j = -1; j <= 1; j++) {
            int distance = abs(i) + abs(j);
            float distanceWeight = gaussianWeights[distance];

            if (direction.axis == X_AXIS) {
                bool shouldSkipY = isOutsideRange(position.y + i, 0, 4);
                bool shouldSkipZ = isOutsideRange(position.z + j, 0, 4);
                if (shouldSkipY || shouldSkipZ) {
                    continue;
                }

                // Partial weight that will be fixed by borderTransfer
                float partialWeight = 1;

                // On the edge with bottom or top brick
                if (position.y + i == 0 || position.y + i == 4) {
                    partialWeight *= 0.5;
                }

                // On the edge with far or near brick
                if (position.z + j == 0 || position.z + j == 4) {
                    partialWeight *= 0.5;
                }

                float weight = distanceWeight * partialWeight;
                ivec3 baseVoxel = ivec3(baseOffset, i + 1, j + 1);
                newColor = calculateDirectionalValue(baseVoxel, direction) * weight;
                color += newColor;
                weightSum += weight;
            } else if (direction.axis == Y_AXIS) {
                bool shouldSkipX = isOutsideRange(position.x + i, 0, 4);
                bool shouldSkipZ = isOutsideRange(position.z + j, 0, 4);
                if (shouldSkipX || shouldSkipZ) {
                    continue;
                }

                float partialWeight = 1;

                // On the edge with left or right brick
                if (position.x + i == 0 || position.x + i == 4) {
                    partialWeight *= 0.5;
                }

                // On the edge with far or near brick
                if (position.z + j == 0 || position.z + j == 4) {
                    partialWeight *= 0.5;
                }

                float weight = distanceWeight * partialWeight;
                ivec3 baseVoxel = ivec3(i + 1, baseOffset, j + 1);
                newColor = calculateDirectionalValue(baseVoxel, direction) * weight;
                color += newColor;
                weightSum += weight;
            } else if (direction.axis == Z_AXIS) {
                bool shouldSkipX = isOutsideRange(position.x + i, 0, 4);
                bool shouldSkipY = isOutsideRange(position.y + j, 0, 4);
                if (shouldSkipX || shouldSkipY) {
                    continue;
                }

                // Partial weight that will be fixed by borderTransfer
                float partialWeight = 1;

                // On the edge with left or right brick
                if (position.x + i == 0 || position.x + i == 4) {
                    partialWeight *= 0.5;
                }

                // On the edge with bottom or top brick
                if (position.y + j == 0 || position.y + j == 4) {
                    partialWeight *= 0.5;
                }

                float weight = distanceWeight * partialWeight;
                ivec3 baseVoxel = ivec3(i + 1, j + 1, baseOffset);
                newColor = calculateDirectionalValue(baseVoxel, direction) * weight;
                color += newColor;
                weightSum += weight;
            }
        }
    }

    return color / weightSum;
}
