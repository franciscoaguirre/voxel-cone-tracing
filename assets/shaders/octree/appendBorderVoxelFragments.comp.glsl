#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) readonly uimageBuffer levelStartIndices;
uniform layout(binding = 1, rgb10_a2ui) writeonly uimageBuffer borderVoxelFragments;
uniform layout(binding = 2, rgb10_a2ui) readonly uimageBuffer nodePositions;
uniform layout(binding = 3, r32ui) readonly uimageBuffer nodePoolNeighbors[MAX_NEIGHBORS];

uniform layout(binding = 0, offset = 0) atomic_uint nextVoxelFragmentCounter;

uniform uint octreeLevel;
uniform bool shouldStore;

const uvec4 NEIGHBOR_OFFSETS[MAX_NEIGHBORS] = {
    uvec4(1, 0, 0, 0),
    uvec4(-1, 0, 0, 0),
    uvec4(0, 1, 0, 0),
    uvec4(0, -1, 0, 0),
    uvec4(0, 0, 1, 0),
    uvec4(0, 0, -1, 0)
};

#include "./_helpers.glsl"
#include "./_threadNodeUtil.glsl"

void main() {
    int nodeID = getThreadNode();

    if (nodeID == NODE_NOT_FOUND) {
        return;
    }

    uvec4 nodePosition = imageLoad(nodePositions, nodeID);

    for (uint i = 0; i < MAX_NEIGHBORS; i++) {
        uint neighborID = imageLoad(nodePoolNeighbors[i], nodeID).r;

        if (neighborID == 0) {
            uvec4 borderVoxelFragmentPosition = nodePosition + NEIGHBOR_OFFSETS[i];
            uint nextVoxelFragment = atomicCounterIncrement(nextVoxelFragmentCounter);
            if (shouldStore) {
                imageStore(borderVoxelFragments, int(nextVoxelFragment), borderVoxelFragmentPosition);
            }
        }
    }
}
