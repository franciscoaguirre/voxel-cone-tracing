// Requires:
// - _helpers
// - binding brickPoolPhotons
// - binding nodePool
// - _helpers

int childNodeIDs[] = {0, 0, 0, 0, 0, 0, 0, 0};

// const float gaussianWeights[4] = { 0.125, 0.0625, 0.03125, 0.015625 };
const float gaussianWeights[4] = { 0.25, 0.125, 0.0625, 0.03125 };

void loadChildNodeIDs(in int nodeID) {
    for (int i = 0; i < 8; i++) {
        int childNodeID = int(imageLoad(nodePool, nodeID * CHILDREN_PER_NODE + i).r);
        childNodeIDs[i] = childNodeID;
    }
    memoryBarrier();
}

uint getValue(in ivec3 position) {
    ivec3 childOffset = ivec3(round(vec3(position) / 4.0));
    int childIndex = childOffset.x + childOffset.y * 2 + childOffset.z * 4;
    ivec3 localPositionInChild = position - 2 * childOffset;

    ivec3 childBrickAddress = calculateBrickCoordinates(childNodeIDs[childIndex]);
    return imageLoad(brickPoolPhotons, childBrickAddress + localPositionInChild).r;
}

uint mipmapIsotropic(in ivec3 position) {
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
                    uint value = getValue(lookupPosition);

                    // TODO: Figure out if we want to use the weights
                    finalValue += weight * float(value);
                    // finalValue += float(value);
                    // weightSum += weight;
                }
            }
        }
    }

    return uint(finalValue);
}
