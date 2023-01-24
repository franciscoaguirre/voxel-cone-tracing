#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, r32ui) uimageBuffer nodePoolBrickPointers;
uniform layout(binding = 2, rgba8) image3D brickPoolValues;
uniform layout(binding = 3, r32ui) uimageBuffer levelStartIndices;

uniform uint octreeLevel;

#include "./_helpers.glsl"
#include "./_threadNodeUtil.glsl"
#include "./_mipmapUtil.glsl"

void main() {
    int nodeAddress = getThreadNode();

    if (nodeAddress == NODE_NOT_FOUND) {
        return;
    }

    ivec3 brickAddress = ivec3(uintXYZ10ToVec3(imageLoad(nodePoolBrickPointers, int(nodeAddress)).r));

    vec4 finalColor = vec4(0);

    uint childAddress = imageLoad(nodePool, int(nodeAddress)).r * NODES_PER_TILE;

    uint childBrickPointer = ivec3(uintXYZ10ToVec3(imageLoad(nodePoolBrickPointers, childAddress).r));
    loadChildTile(int(childAddress));

    vec4 color = mipmapIsotropic(ivec3(2, 2, 2));
    memoryBarrier();

    imageStore(brickPoolValues, brickAddress + ivec3(1, 1, 1), color);
}
