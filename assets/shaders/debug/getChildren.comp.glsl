#version 460 core

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) writeonly uimageBuffer childrenBuffer;
uniform layout(binding = 1, r32ui) readonly uimageBuffer nodePool;

uniform uint nodeID;

#include "assets/shaders/octree/_constants.glsl"

void main() {
    for (int offset = 0; offset < CHILDREN_PER_NODE; offset++) {
        uint childNodeID = imageLoad(nodePool, int(nodeID) * CHILDREN_PER_NODE + offset).r;
        imageStore(childrenBuffer, offset, uvec4(childNodeID, 0, 0, 0));
    }
}
