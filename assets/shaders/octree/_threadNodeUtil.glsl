// Dependencies:
// - _constants
// - uniform uimageBuffer (r32ui) levelStartIndices
// - uniform uint octreeLevel

int getThreadNode() {
    uint index;

    int levelStart = int(imageLoad(levelStartIndices, int(octreeLevel)).r) * CHILDREN_PER_NODE;
    int nextLevelStart = int(imageLoad(levelStartIndices, int(octreeLevel + 1)).r) * CHILDREN_PER_NODE;
    memoryBarrier();

    index = uint(levelStart) + uint(gl_GlobalInvocationID.x);

    if (index >= uint(nextLevelStart)) {
        return NODE_NOT_FOUND;
    }

    return int(index);
}