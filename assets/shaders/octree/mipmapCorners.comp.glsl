#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, rgba8) image3D brickPoolValues;
uniform layout(binding = 2, r32ui) uimageBuffer levelStartIndices;

uniform uint octreeLevel;

#include "./_helpers.glsl"
#include "./_threadNodeUtil.glsl"
#include "./_mipmapUtil.glsl"

void main() {
    int nodeAddress = getThreadNode();

    if (nodeAddress == NODE_NOT_FOUND) {
        return;
    }

    loadChildNodeIDs(nodeAddress);
    vec4 nearRightTop = mipmapIsotropic(ivec3(4, 4, 0));
    vec4 nearRightBottom = mipmapIsotropic(ivec3(4, 0, 0));
    vec4 nearLeftTop = mipmapIsotropic(ivec3(0, 4, 0));
    vec4 nearLeftBottom = mipmapIsotropic(ivec3(0, 0, 0));
    vec4 farRightTop = mipmapIsotropic(ivec3(4, 4, 4));
    vec4 farRightBottom = mipmapIsotropic(ivec3(4, 0, 4));
    vec4 farLeftTop = mipmapIsotropic(ivec3(0, 4, 4));
    vec4 farLeftBottom = mipmapIsotropic(ivec3(0, 0, 4));
  
    memoryBarrier();
  
    ivec3 brickAddress = calculateBrickCoordinates(nodeAddress);
    imageStore(brickPoolValues, brickAddress + ivec3(2, 2, 0), nearRightTop);
    imageStore(brickPoolValues, brickAddress + ivec3(2, 0, 0), nearRightBottom);
    imageStore(brickPoolValues, brickAddress + ivec3(0, 2, 0), nearLeftTop);
    imageStore(brickPoolValues, brickAddress + ivec3(0, 0, 0), nearLeftBottom);
    imageStore(brickPoolValues, brickAddress + ivec3(2, 2, 2), farRightTop);
    imageStore(brickPoolValues, brickAddress + ivec3(2, 0, 2), farRightBottom);
    imageStore(brickPoolValues, brickAddress + ivec3(0, 2, 2), farLeftTop);
    imageStore(brickPoolValues, brickAddress + ivec3(0, 0, 2), farLeftBottom);
}
