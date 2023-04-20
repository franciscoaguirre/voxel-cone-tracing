// Requires:
// - _helpers

int childNodeIDs[] = {0, 0, 0, 0, 0, 0, 0, 0};

// const float gaussianWeights[4] = { 0.125, 0.0625, 0.03125, 0.015625 };
const float gaussianWeights[4] = { 0.25, 0.125, 0.0625, 0.03125 };

void loadChildNodeIDs(in int nodeID, uimageBuffer nodePool) {
    for (int i = 0; i < 8; i++) {
        int childNodeID = int(imageLoad(nodePool, nodeID * CHILDREN_PER_NODE + i).r);
        childNodeIDs[i] = childNodeID;
    }
    memoryBarrier();
}

vec4 getValue(in ivec3 position, image3D brickPoolValues) {
    ivec3 childOffset = ivec3(round(vec3(position) / 4.0));
    int childIndex = childOffset.x + childOffset.y * 2 + childOffset.z * 4;
    ivec3 localPositionInChild = position - 2 * childOffset;

    ivec3 childBrickAddress = calculateBrickCoordinates(childNodeIDs[childIndex]);
    return imageLoad(brickPoolValues, childBrickAddress + localPositionInChild);
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
    float weightSum = 0.0;

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
                    weightSum += weight;
                }
            }
        }
    }

    return finalValue / weightSum;
}

uint mipmapIsotropic(in ivec3 position, uimage3D brickPoolValues) {
    float finalValue = 0;
    float weightSum = 0.0;

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
                    weightSum += weight;
                }
            }
        }
    }

    // TODO: We're having an overflow issue here with photons
    return uint(round(finalValue / weightSum));
}
