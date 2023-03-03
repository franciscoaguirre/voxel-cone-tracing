#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform int firstNodeInLevel;
uniform int firstFreeNode;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;

uniform layout(binding = 0, offset = 0) atomic_uint allocatedNodesCounter;

bool isNodeFlagged(uint node) {
    return (node & NODE_FLAG_VALUE) != 0;
}

void main()
{
    uint allocatedNodeIndex;
    uint threadIndex = gl_GlobalInvocationID.x;
    int parentNodeIndex = firstNodeInLevel * CHILDREN_PER_NODE + int(threadIndex);
    uint parentNode = imageLoad(nodePool, parentNodeIndex).r;

    if (isNodeFlagged(parentNode)) {
        allocatedNodeIndex = atomicCounterIncrement(allocatedNodesCounter);
        allocatedNodeIndex += firstFreeNode;

        imageStore(nodePool, parentNodeIndex, uvec4(allocatedNodeIndex, 0, 0, 0));
    }
}
