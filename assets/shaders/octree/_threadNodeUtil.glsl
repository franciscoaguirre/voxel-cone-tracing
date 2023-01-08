// Dependencies:
// - _constants
// - uniform uimageBuffer (r32ui) levelStartIndices
// - uniform uint octreeLevel

int getThreadNode() {
    uint index;

    int levelStart = int(imageLoad(levelStartIndices, int(octreeLevel)).r) * NODES_PER_TILE;
    int nextLevelStart = int(imageLoad(levelStartIndices, int(octreeLevel + 1)).r) * NODES_PER_TILE;
    memoryBarrier();

    index = uint(levelStart) + uint(gl_GlobalInvocationID.x);

    if (index >= uint(nextLevelStart)) {
        return NODE_NOT_FOUND;
    }

    return int(index);
}