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

    loadChildNodeIDs(nodeAddress, nodePool);
    vec4 nearBottom = mipmapIsotropic(ivec3(2, 0, 0));
    vec4 nearRight = mipmapIsotropic(ivec3(4, 2, 0));
    vec4 nearTop = mipmapIsotropic(ivec3(2, 4, 0));
    vec4 nearLeft = mipmapIsotropic(ivec3(0, 2, 0));
    vec4 farBottom = mipmapIsotropic(ivec3(2, 0, 4));
    vec4 farRight = mipmapIsotropic(ivec3(4, 2, 4));
    vec4 farTop = mipmapIsotropic(ivec3(2, 4, 4));
    vec4 farLeft = mipmapIsotropic(ivec3(0, 2, 4));
    vec4 leftBottom = mipmapIsotropic(ivec3(0, 0, 2));
    vec4 leftTop = mipmapIsotropic(ivec3(0, 4, 2));
    vec4 rightBottom = mipmapIsotropic(ivec3(4, 0, 2));
    vec4 rightTop = mipmapIsotropic(ivec3(4, 4, 2));

    memoryBarrier();

    ivec3 brickAddress = calculateBrickCoordinates(nodeAddress);
    imageStore(brickPoolValues, brickAddress + ivec3(1,0,0), nearBottom);
    imageStore(brickPoolValues, brickAddress + ivec3(2,1,0), nearRight);
    imageStore(brickPoolValues, brickAddress + ivec3(1,2,0), nearTop);
    imageStore(brickPoolValues, brickAddress + ivec3(0,1,0), nearLeft);
    imageStore(brickPoolValues, brickAddress + ivec3(1,0,2), farBottom);
    imageStore(brickPoolValues, brickAddress + ivec3(2,1,2), farRight);
    imageStore(brickPoolValues, brickAddress + ivec3(1,2,2), farTop);
    imageStore(brickPoolValues, brickAddress + ivec3(0,1,2), farLeft);
    imageStore(brickPoolValues, brickAddress + ivec3(0, 0, 1), leftBottom);
    imageStore(brickPoolValues, brickAddress + ivec3(2, 0, 1), rightBottom);
    imageStore(brickPoolValues, brickAddress + ivec3(0, 2, 1), leftTop);
    imageStore(brickPoolValues, brickAddress + ivec3(2, 2, 1), rightTop);
}
