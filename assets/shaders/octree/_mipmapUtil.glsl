// Requires:
// - Uniform nodePool
// - Uniform nodePoolBrickPointers
// - Uniform brickPoolValues

uint childNodes[] = {0, 0, 0, 0, 0, 0, 0, 0};
uint childBrickPointers[] = {0, 0, 0, 0, 0, 0, 0, 0};
const uvec3 childOffsets[8] = {
    uvec3(0, 0, 0),
    uvec3(1, 0, 0),
    uvec3(0, 1, 0),
    uvec3(1, 1, 0),
    uvec3(0, 0, 1),
    uvec3(1, 0, 1),
    uvec3(0, 1, 1),
    uvec3(1, 1, 1),
};

const float gaussianWeight[4] = {0.25, 0.125, 0.0625, 0.03125};

void loadChildTile(in int tileAddress) {
    for (int i = 0; i < 8; i++) {
        childNodes[i] = imageLoad(nodePool, tileAddress + i).r;
        childBrickPointers[i] = imageLoad(nodePoolBrickPointers, tileAddress + i).r;
    }
    memoryBarrier();
}

vec4 getColor(in ivec3 pos) {
    ivec3 childPos = ivec3(round(vec3(pos) / 4.0));
    int childIndex = childPos.x + 2 * childPos.y + 4 * childPos.z;
    ivec3 localPos = pos - 2 * childPos;

    // imageStore();

    ivec3 childBrickAddress = ivec3(uintXYZ10ToVec3(childBrickPointers[childIndex]));
    return imageLoad(brickPoolValues, childBrickAddress + localPos);
}

vec4 mipmapIsotropic(in ivec3 pos) {
    vec4 col = vec4(0);
    float weightSum = 0.0;

    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            for (int z = -1; z <= 1; z++) {
                const ivec3 lookupPos = pos + ivec3(x, y, z);

                // TODO: Why <= 4 and not < 4?
                if (
                    lookupPos.x >= 0 && lookupPos.y >= 0 && lookupPos.z >= 0 &&
                    lookupPos.x <= 4 && lookupPos.y <= 4 && lookupPos.z <= 4
                ) {
                    const int manhattanDistance = abs(x) + abs(y) + abs(z);
                    const float weight = gaussianWeight[manhattanDistance];
                    const vec4 lookupColor = getColor(lookupPos);

                    col += weight * lookupColor;
                    weightSum += weight;
                }
            }
        }
    }
    
    return col / weightSum;
}
