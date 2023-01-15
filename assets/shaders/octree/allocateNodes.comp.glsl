#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform int firstTileInLevel;
uniform int firstFreeTile;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;

uniform layout(binding = 0, offset = 0) atomic_uint allocatedTilesCounter;

bool isNodeFlagged(uint node) {
    return (node & NODE_FLAG_VALUE) != 0;
}

void main()
{
    uint allocatedTileIndex;
    uint threadIndex = gl_GlobalInvocationID.x;
    int parentNodeIndex = firstTileInLevel * NODES_PER_TILE + int(threadIndex);
    uint parentNode = imageLoad(nodePool, parentNodeIndex).r;

    if (isNodeFlagged(parentNode)) {
        allocatedTileIndex = atomicCounterIncrement(allocatedTilesCounter);
        allocatedTileIndex += firstFreeTile;

        imageStore(nodePool, parentNodeIndex, uvec4(allocatedTileIndex, 0, 0, 0));
    }
}
