#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, r32ui) uimageBuffer nodePoolBrickPointers;
uniform layout(binding = 2, rgba8) image3D brickPoolValues;
uniform layout(binding = 3, r32ui) uimageBuffer levelStartIndices;

uniform uint octreeLevel;

#include "./_helpers.glsl"
#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"
#include "./_mipmapUtil.glsl"

#include "./_threadNodeUtil.glsl"

void main() {
    int nodeAddress = getThreadNode();

    if (nodeAddress == NODE_NOT_FOUND) {
        return;
    }

    ivec3 brickAddress = ivec3(uintXYZ10ToVec3(imageLoad(nodePoolBrickPointers, int(nodeAddress)).r));

    uint childAddress = imageLoad(nodePool, int(nodeAddress)).r * NODES_PER_TILE;
    loadChildTile(int(childAddress));

    vec4 nearRightTop = mipmapIsotropic(ivec3(4, 4, 0));
    vec4 nearRightBottom = mipmapIsotropic(ivec3(4, 0, 0));
    vec4 nearLeftTop = mipmapIsotropic(ivec3(0, 4, 0));
    vec4 nearLeftBottom = mipmapIsotropic(ivec3(0, 0, 0));
    vec4 farRightTop = mipmapIsotropic(ivec3(4, 4, 4));
    vec4 farRightBottom = mipmapIsotropic(ivec3(4, 0, 4));
    vec4 farLeftTop = mipmapIsotropic(ivec3(0, 4, 4));
    vec4 farLeftBottom = mipmapIsotropic(ivec3(0, 0, 4));
  
    memoryBarrier();
  
    imageStore(brickPool_value, brickAddress + ivec3(2, 2, 0), nearRightTop);
    imageStore(brickPool_value, brickAddress + ivec3(2, 0, 0), nearRightBottom);
    imageStore(brickPool_value, brickAddress + ivec3(0, 2, 0), nearLeftTop);
    imageStore(brickPool_value, brickAddress + ivec3(0, 0, 0), nearLeftBottom);
    imageStore(brickPool_value, brickAddress + ivec3(2, 2, 2), farRightTop);
    imageStore(brickPool_value, brickAddress + ivec3(2, 0, 2), farRightBottom);
    imageStore(brickPool_value, brickAddress + ivec3(0, 2, 2), farLeftTop);
    imageStore(brickPool_value, brickAddress + ivec3(0, 0, 2), farLeftBottom);
}
