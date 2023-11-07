// Dependencies:
// - _constants
// - uniform uint levelStart
// - uniform uint nextLevelStart

int getThreadNode() {
    uint index = levelStart + uint(gl_GlobalInvocationID.x);

    if (index >= nextLevelStart) {
        return NODE_NOT_FOUND;
    }

    return int(index);
}
