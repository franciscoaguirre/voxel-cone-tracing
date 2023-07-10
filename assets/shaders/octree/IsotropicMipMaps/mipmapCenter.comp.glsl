#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, rgba8) image3D brickPoolValues;
uniform layout(binding = 2, r32ui) uimageBuffer levelStartIndices;

uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_helpers.glsl"
#include "./_threadNodeUtil.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"
#include "./_mipmapUtil.glsl"

void main() {
    int nodeAddress = getThreadNode();

    if (nodeAddress == NODE_NOT_FOUND) {
        return;
    }

    loadChildNodeIDs(nodeAddress);
    vec4 color = mipmapIsotropic(ivec3(2, 2, 2));

    memoryBarrier();

    ivec3 brickAddress = calculateBrickCoordinates(nodeAddress);
    imageStore(brickPoolValues, brickAddress + ivec3(1, 1, 1), color);
}