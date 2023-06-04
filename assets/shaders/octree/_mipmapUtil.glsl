// Requires:
// - _helpers

int childNodeIDs[] = {0, 0, 0, 0, 0, 0, 0, 0};

const float gaussianWeights[4] = { 0.125, 0.0625, 0.03125, 0.015625 };

void loadChildNodeIDs(in int nodeID, uimageBuffer nodePool) {
    for (int i = 0; i < 8; i++) {
        int childNodeID = int(imageLoad(nodePool, nodeID * CHILDREN_PER_NODE + i).r);
        childNodeIDs[i] = childNodeID;
    }
    memoryBarrier();
}

/// Gets the value from the position in brick pool.
/// Based on position (from 0 to 4), finds the corresponding child.
/// If the voxel is on a border, it accounts for half on each dimension.
vec4 getValue(in ivec3 position, image3D brickPoolValues) {
    ivec3 childOffset = ivec3(round(vec3(position) / 4.0));
    int childIndex = childOffset.x + childOffset.y * 2 + childOffset.z * 4;
    ivec3 localPositionInChild = position - 2 * childOffset;
    ivec3 childBrickAddress = calculateBrickCoordinates(childNodeIDs[childIndex]);
    vec4 voxelColor = imageLoad(brickPoolValues, childBrickAddress + localPositionInChild);

    // uvec3 isOnBorder = uvec3(equal(position, ivec3(0))) | uvec3(equal(position, ivec3(4)));
    // vec3 multiplierVector = vec3(1) - (vec3(isOnBorder) * vec3(0.5));
    // float multiplier = multiplierVector.x * multiplierVector.y * multiplierVector.z;

    float multiplier = 1;
    if (position.x == 0 || position.x == 4) {
        multiplier *= 0.5;
    }
    if (position.y == 0 || position.y == 4) {
        multiplier *= 0.5;
    }
    if (position.z == 0 || position.z == 4) {
        multiplier *= 0.5;
    }
    voxelColor *= multiplier;

    return voxelColor;
}

uint getValue(in ivec3 position, uimage3D brickPoolValues) {
    ivec3 childOffset = ivec3(round(vec3(position) / 4.0));
    int childIndex = childOffset.x + childOffset.y * 2 + childOffset.z * 4;
    ivec3 localPositionInChild = position - 2 * childOffset;

    ivec3 childBrickAddress = calculateBrickCoordinates(childNodeIDs[childIndex]);
    return imageLoad(brickPoolValues, childBrickAddress + localPositionInChild).r;
}

vec4 mipmapIsotropic(in ivec3 position, image3D brickPoolValues) {
    vec4 finalValue = vec4(0);

    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            for (int z = -1; z <= 1; z++) {
                ivec3 lookupPosition = position + ivec3(x, y, z);
                
                if (
                    lookupPosition.x >= 0 &&
                    lookupPosition.x <= 4 &&
                    lookupPosition.y >= 0 &&
                    lookupPosition.y <= 4 &&
                    lookupPosition.z >= 0 &&
                    lookupPosition.z <= 4
                ) {
                    // It's a voxel from our children and not a neighbor
                    int distance = abs(x) + abs(y) + abs(z);
                    float weight = gaussianWeights[distance];
                    vec4 value = getValue(lookupPosition, brickPoolValues);

                    finalValue += weight * value;
                }
            }
        }
    }

    return finalValue;
}

uint mipmapIsotropic(in ivec3 position, uimage3D brickPoolValues) {
    float finalValue = 0;

    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            for (int z = -1; z <= 1; z++) {
                ivec3 lookupPosition = position + ivec3(x, y, z);
                
                if (
                    lookupPosition.x >= 0 &&
                    lookupPosition.x <= 4 &&
                    lookupPosition.y >= 0 &&
                    lookupPosition.y <= 4 &&
                    lookupPosition.z >= 0 &&
                    lookupPosition.z <= 4
                ) {
                    // It's a voxel from our children and not a neighbor
                    int distance = abs(x) + abs(y) + abs(z);
                    float weight = gaussianWeights[distance];
                    uint value = getValue(lookupPosition, brickPoolValues);

                    finalValue += weight * float(value);
                }
            }
        }
    }

    return uint(finalValue);
}
